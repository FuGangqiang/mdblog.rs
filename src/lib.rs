//! static site generator from markdown files.

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/mdblog"
)]
#![deny(unused_extern_crates)]
#![deny(missing_docs)]
#![allow(clippy::needless_return)]
#![allow(clippy::expect_fun_call)]
#![allow(clippy::or_fun_call)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use chrono::Local;
use config::Config;
use glob::Pattern;
use log::{debug, error, info};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use tempfile::{Builder as TempBuilder, TempDir};
use tera::{Context, Tera};
use walkdir::{DirEntry, WalkDir};

pub use crate::error::{Error, Result};
pub use crate::page::Page;
pub use crate::post::Post;
pub use crate::post::PostHeaders;
pub use crate::settings::Settings;
pub use crate::tag::Tag;
pub use crate::theme::Theme;
use crate::utils::write_file;

mod error;
mod page;
mod post;
mod settings;
mod tag;
mod theme;
mod utils;

/// blog object
pub struct Mdblog {
    /// blog root path
    root: PathBuf,
    /// blog settings
    settings: Settings,
    /// blog theme
    theme: Theme,
    /// collection of blog posts
    posts: Vec<Rc<Post>>,
    /// collection of blog index pages
    index_pages: Vec<Rc<Page>>,
    /// collection of blog tags pages
    tag_pages: BTreeMap<String, Vec<Rc<Page>>>,
    /// tags map
    tags_map: BTreeMap<String, Tag>,
    /// server root dir
    server_root_dir: Option<TempDir>,
}

impl Mdblog {
    /// create from the `root` path.
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Mdblog> {
        let root = root.as_ref();
        let settings: Settings = Default::default();
        let theme_root_dir = get_dir(root, &settings.theme_root_dir)?;
        let theme = Theme::new(theme_root_dir, &settings.theme)?;
        Ok(Mdblog {
            root: root.to_owned(),
            settings,
            theme,
            posts: Vec::new(),
            index_pages: Vec::new(),
            tag_pages: BTreeMap::new(),
            tags_map: BTreeMap::new(),
            server_root_dir: None,
        })
    }

    /// load blog customize settings.
    ///
    /// layered configuration system:
    /// * default settings
    /// * `config.toml`
    /// * `BLOG_` prefix environment variable
    pub fn load_customize_settings(&mut self) -> Result<()> {
        let mut settings = Config::new();
        settings.merge(self.settings.clone())?;
        settings.merge(config::File::with_name("config.toml"))?;
        settings.merge(config::Environment::with_prefix("BLOG"))?;
        self.settings = settings.try_into()?;
        if self.settings.site_url.ends_with('/') {
            self.settings.site_url = self.settings.site_url.trim_end_matches('/').to_string();
        }
        let theme_root_dir = self.theme_root_dir()?;
        self.theme = Theme::new(&theme_root_dir, &self.settings.theme)?;
        Ok(())
    }

    fn reset_site_url(&mut self, host: &str, port: u16) {
        self.settings.site_url = format!("http://{}:{}", host, port);
    }

    /// load blog posts.
    pub fn load_posts(&mut self) -> Result<()> {
        let mut posts: Vec<Rc<Post>> = Vec::new();
        let mut tags_map: BTreeMap<String, Tag> = BTreeMap::new();
        let walker = WalkDir::new(&self.post_root_dir()?).into_iter();

        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.expect("get walker entry error");
            if !is_markdown_file(&entry) {
                continue;
            }
            let post_path = entry.path().strip_prefix(&self.root)?.to_owned();
            let post = Post::new(&self.root, &post_path)?;
            let post = Rc::new(post);
            posts.push(Rc::clone(&post));
            if post.headers.hidden {
                continue;
            }
            for name in &post.headers.tags {
                let tag = tags_map.entry(name.to_string()).or_insert(Tag::new(name));
                tag.add(post.clone());
            }
        }
        posts.sort_by(|p1, p2| p2.headers.created.cmp(&p1.headers.created));
        for tag in tags_map.values_mut() {
            tag.posts.sort_by(|p1, p2| p2.headers.created.cmp(&p1.headers.created));
        }
        self.posts = posts;
        self.tags_map = tags_map;
        Ok(())
    }

    /// build index pages
    pub fn build_index_pages(&mut self) -> Result<()> {
        let posts: Vec<_> = self.posts.iter().filter(|p| !p.headers.hidden).collect();
        let total = posts.len();
        let n = (total + self.settings.posts_per_page - 1) / self.settings.posts_per_page;
        let mut i = 1;
        while i <= n {
            let start = (i - 1) * self.settings.posts_per_page;
            let end = total.min(start + self.settings.posts_per_page);
            let page = Page {
                index: i,
                name: format_page_name("index", i, total),
                posts: posts[start..end].to_vec().into_iter().map(|p| p.to_owned()).collect(),
            };
            self.index_pages.push(Rc::new(page));
            i += 1;
        }
        Ok(())
    }

    /// build tag pages
    pub fn build_tag_pages(&mut self) -> Result<()> {
        for tag in self.tags_map.values() {
            let total = tag.posts.len();
            let n = (total + self.settings.posts_per_page - 1) / self.settings.posts_per_page;
            let mut i = 1;
            while i <= n {
                let start = (i - 1) * self.settings.posts_per_page;
                let end = total.min(start + self.settings.posts_per_page);
                let page = Page {
                    index: i,
                    name: format_page_name(&tag.name, i, total),
                    posts: tag.posts[start..end].to_vec().into_iter().collect(),
                };
                let pages = self.tag_pages.entry(tag.name.clone()).or_insert(Vec::new());
                pages.push(Rc::new(page));
                i += 1;
            }
        }
        Ok(())
    }

    /// init blog directory.
    pub fn init(&mut self) -> Result<()> {
        if self.root.exists() {
            return Err(Error::RootDirExisted(self.root.clone()));
        }

        let mut tera = Tera::default();
        tera.add_raw_template("hello.md.tpl", include_str!("demo/hello.md.tpl"))?;
        tera.add_raw_template("math.md.tpl", include_str!("demo/math.md.tpl"))?;

        let now = Local::now();
        let mut context = Context::new();
        context.insert("now", &now.format("%Y-%m-%dT%H:%M:%S%:z").to_string());

        let hello_content = tera.render("hello.md.tpl", &context)?;
        let math_content = tera.render("math.md.tpl", &context)?;
        write_file(&self.post_root_dir()?.join("hello.md"), hello_content.as_bytes())?;
        write_file(&self.post_root_dir()?.join("math.md"), math_content.as_bytes())?;

        self.export_config()?;

        self.theme.init_dir(&self.theme.name)?;
        std::fs::create_dir_all(self.root.join("media"))?;
        Ok(())
    }

    /// build the blog html files to `build_dir` directory.
    pub fn build(&mut self) -> Result<()> {
        self.load_posts()?;
        self.build_index_pages()?;
        self.build_tag_pages()?;
        self.export_media()?;
        self.export_static()?;
        self.export_posts()?;
        self.export_index()?;
        self.export_tags()?;
        for tag in self.tags_map.values() {
            self.export_tag(tag)?;
        }
        self.export_atom()?;
        Ok(())
    }

    /// serve the blog static files in the `build_dir` directory.
    pub fn serve(&mut self, host: String, port: u16) -> Result<()> {
        let addr_str = format!("{}:{}", host, port);
        let server_root_dir = TempBuilder::new().prefix("mdblog.").rand_bytes(10).tempdir()?;
        info!("server root dir: {}", &server_root_dir.path().display());

        self.server_root_dir = Some(server_root_dir);
        self.settings.site_url = format!("http://{}", &addr_str);
        self.build()?;

        info!("server blog at {}", &self.settings.site_url);
        let server_root_dir = self.server_root_dir.as_ref().unwrap().path().to_owned();

        self.reset_site_url(&host, port);
        thread::spawn(move || {
            let mut config = rocket::config::Config::production();
            config
                .set_address(&host)
                .expect(&format!("can not bind address: {}", host));
            config.set_port(port);
            rocket::custom(config)
                .mount("/", rocket_contrib::serve::StaticFiles::from(&server_root_dir))
                .launch();
        });

        self.open_browser();
        self.watch()?;
        Ok(())
    }

    /// watch blog files, rebuild blog when some files modified.
    fn watch(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let ignore_patterns = self.ignore_patterns()?;
        info!("watching dir: {}", self.root.display());
        let mut watcher = watcher(tx, Duration::new(2, 0))?;
        watcher.watch(&self.root, RecursiveMode::Recursive)?;
        let interval = Duration::new(self.settings.rebuild_interval.into(), 0);
        let mut last_run: Option<Instant> = None;
        loop {
            match rx.recv() {
                Err(why) => error!("watch error: {:?}", why),
                Ok(event) => match event {
                    DebouncedEvent::Create(ref fpath)
                    | DebouncedEvent::Write(ref fpath)
                    | DebouncedEvent::Remove(ref fpath)
                    | DebouncedEvent::Rename(ref fpath, _) => {
                        if ignore_patterns.iter().any(|pat| pat.matches_path(fpath)) {
                            continue;
                        }
                        let now = Instant::now();
                        if let Some(last_time) = last_run {
                            if now.duration_since(last_time) < interval {
                                continue;
                            }
                        }
                        last_run = Some(now);
                        info!("Modified file: {}", fpath.display());
                        if let Err(ref e) = self.rebuild() {
                            crate::utils::log_error_chain(e);
                            continue;
                        }
                    }
                    _ => {}
                },
            }
        }
        #[allow(unreachable_code)]
        Ok(())
    }

    /// open url with browser
    fn open_browser(&self) {
        let url = self.settings.site_url.clone();
        thread::spawn(move || {
            open::that(url).unwrap();
        });
    }

    /// rebuild blog
    pub fn rebuild(&mut self) -> Result<()> {
        info!("Rebuild blog again...");
        let site_url = self.settings.site_url.clone();
        self.load_customize_settings()?;
        self.settings.site_url = site_url;
        self.build()?;
        info!("Rebuild done!");
        Ok(())
    }

    /// blog build directory absolute path.
    pub fn build_root_dir(&self) -> Result<PathBuf> {
        if let Some(ref server_root_dir) = self.server_root_dir {
            Ok(server_root_dir.path().to_owned())
        } else {
            get_dir(&self.root, &self.settings.build_dir)
        }
    }

    /// blog theme root directory absolute path.
    pub fn theme_root_dir(&self) -> Result<PathBuf> {
        get_dir(&self.root, &self.settings.theme_root_dir)
    }

    /// blog media root directory absolute path.
    pub fn media_root_dir(&self) -> Result<PathBuf> {
        get_dir(&self.root, &self.settings.media_dir)
    }

    /// blog posts root directory.
    pub fn post_root_dir(&self) -> Result<PathBuf> {
        Ok(self.root.join("posts"))
    }

    /// blog glob ignore patterns.
    ///
    /// the patterns are used when :
    /// * `mdblog new` command, the post path is checked
    /// * `mdblog serve` command, the modified file path is checked
    pub fn ignore_patterns(&self) -> Result<Vec<Pattern>> {
        let mut patterns = vec![Pattern::new("**/.*")?];
        let build_dir = self
            .build_root_dir()?
            .to_str()
            .expect("get build dir error")
            .to_string();
        patterns.push(Pattern::new(&format!("{}/**/*", build_dir.trim_end_matches('/')))?);
        Ok(patterns)
    }

    /// create a new sample post.
    pub fn create_post(&self, path: &Path, tags: &[String]) -> Result<()> {
        let post_title = path.file_stem();
        if !path.is_relative()
            || path.extension().is_some()
            || path.to_str().unwrap_or("").is_empty()
            || post_title.is_none()
            || self.ignore_patterns()?.iter().any(|pat| pat.matches_path(path))
        {
            return Err(Error::PostPathInvaild(path.into()));
        }
        if path.is_dir() {
            return Err(Error::PostPathExisted(path.into()));
        }
        let post_path = self.post_root_dir()?.join(path).with_extension("md");
        if post_path.exists() {
            return Err(Error::PostPathExisted(path.into()));
        }
        let now = Local::now();
        let content = format!(
            "created: {}\n\
             tags: [{}]\n\
             \n\
             this is a new post!\n",
            now.format("%Y-%m-%dT%H:%M:%S%:z"),
            tags.join(", ")
        );
        write_file(&post_path, content.as_bytes())?;
        Ok(())
    }

    /// export blog config.toml file.
    pub fn export_config(&self) -> Result<()> {
        let content = toml::to_string(&self.settings)?;
        write_file(&self.root.join("config.toml"), content.as_bytes())?;
        Ok(())
    }

    fn media_dest<P: AsRef<Path>>(&self, media: P) -> Result<PathBuf> {
        let build_dir = self.build_root_dir()?;
        let rel_path = media.as_ref().strip_prefix(&self.media_root_dir()?)?.to_owned();
        Ok(build_dir.join("media").join(rel_path))
    }

    /// export blog media files.
    pub fn export_media(&self) -> Result<()> {
        debug!("exporting media ...");
        let media_root_dir = self.media_root_dir()?;
        if !media_root_dir.exists() {
            return Ok(());
        }
        let walker = WalkDir::new(&media_root_dir).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.expect("get walker entry error");
            let src_path = entry.path();
            if src_path.is_dir() {
                std::fs::create_dir_all(self.media_dest(src_path)?)?;
                continue;
            }
            std::fs::copy(src_path, self.media_dest(src_path)?)?;
        }
        Ok(())
    }

    /// export blog static files.
    pub fn export_static(&self) -> Result<()> {
        let build_dir = self.build_root_dir()?;
        self.theme.export_static(&build_dir)?;
        Ok(())
    }

    /// export blog posts.
    pub fn export_posts(&self) -> Result<()> {
        let build_dir = self.build_root_dir()?;
        for post in &self.posts {
            let dest = build_dir.join(post.dest());
            let html = self.render_post(post)?;
            write_file(&dest, html.as_bytes())?;
        }
        Ok(())
    }

    /// export blog index page.
    pub fn export_index(&self) -> Result<()> {
        let build_dir = self.build_root_dir()?;
        for (i, page) in self.index_pages.iter().enumerate() {
            let dest = build_dir.join(&page.name);
            let html = self.render_index(i)?;
            write_file(&dest, html.as_bytes())?;
        }
        Ok(())
    }

    /// export blog tags page.
    pub fn export_tags(&self) -> Result<()> {
        let build_dir = self.build_root_dir()?;
        let dest = build_dir.join("tags.html");
        let html = self.render_tags()?;
        write_file(&dest, html.as_bytes())?;
        Ok(())
    }

    /// export blog tag index page.
    pub fn export_tag(&self, tag: &Tag) -> Result<()> {
        let build_dir = self.build_root_dir()?;
        if let Some(pages) = self.tag_pages.get(&tag.name) {
            for (i, page) in pages.iter().enumerate() {
                let dest = build_dir.join("tags").join(&page.name);
                debug!("rendering tag: {} ...", dest.display());
                let html = self.render_tag(tag, i)?;
                write_file(&dest, html.as_bytes())?;
            }
        }
        Ok(())
    }

    /// export blog atom.xml
    pub fn export_atom(&self) -> Result<()> {
        debug!("rendering atom ...");
        let build_dir = self.build_root_dir()?;
        let dest = build_dir.join("atom.xml");
        let now = Local::now();
        let mut context = self.get_base_context()?;
        context.insert("now", &now);
        context.insert("posts", &self.posts[..10.min(self.posts.len())]);
        let html = self.theme.renderer.render("atom.tpl", &context)?;
        write_file(&dest, html.as_bytes())?;
        Ok(())
    }

    /// get base context of `theme.renderer` templates
    fn get_base_context(&self) -> Result<Context> {
        let mut context = Context::new();
        context.insert("config", &self.settings);
        let mut tags = self.tags_map.values().collect::<Vec<_>>();
        tags.sort_by_key(|x| x.name.to_lowercase());
        context.insert("tags", &tags);
        context.insert("tag_map", &self.tags_map);
        context.insert("index_pages", &self.index_pages);
        context.insert("tag_pages", &self.tag_pages);
        Ok(context)
    }

    /// render post.html.
    pub fn render_post(&self, post: &Post) -> Result<String> {
        debug!("rendering post({}) ...", post.path.display());
        let mut context = self.get_base_context()?;
        context.insert("post", &post);
        Ok(self.theme.renderer.render("post.tpl", &context)?)
    }

    /// render index*.html.
    pub fn render_index(&self, i: usize) -> Result<String> {
        debug!("rendering index ...");
        let mut context = self.get_base_context()?;
        context.insert("page", &self.index_pages[i]);
        context.insert("posts", &self.index_pages[i].posts);
        Ok(self.theme.renderer.render("index.tpl", &context)?)
    }

    /// render tags.html.
    pub fn render_tags(&self) -> Result<String> {
        debug!("rendering tags ...");
        let context = self.get_base_context()?;
        Ok(self.theme.renderer.render("tags.tpl", &context)?)
    }

    /// render tag.html.
    pub fn render_tag(&self, tag: &Tag, i: usize) -> Result<String> {
        debug!("rendering tag ...");
        let mut context = self.get_base_context()?;
        let page = self.tag_pages.get(&tag.name).unwrap().get(i).unwrap();
        context.insert("tag", &tag);
        context.insert("page", &page);
        context.insert("posts", &page.posts);
        Ok(self.theme.renderer.render("tag.tpl", &context)?)
    }

    /// list blog themes.
    pub fn list_blog_theme(&self) -> Result<()> {
        let theme_root = self.theme_root_dir()?;
        if !theme_root.exists() || !theme_root.is_dir() {
            error!("no theme");
        }
        for entry in std::fs::read_dir(theme_root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!(
                    "* {}",
                    path.file_name()
                        .expect("theme name error")
                        .to_str()
                        .expect("theme name error")
                );
            }
        }
        Ok(())
    }

    /// create a new blog theme as same as the current blog theme.
    pub fn create_blog_theme(&self, name: &str) -> Result<()> {
        self.theme.init_dir(name)?;
        Ok(())
    }

    /// delete a blog theme.
    pub fn delete_blog_theme(&self, name: &str) -> Result<()> {
        if self.settings.theme == name {
            return Err(Error::ThemeInUse(name.into()));
        }
        let theme_path = self.theme_root_dir()?.join(name);
        if !theme_path.exists() || !theme_path.is_dir() {
            return Err(Error::ThemeNotFound(name.into()));
        }
        std::fs::remove_dir_all(theme_path)?;
        Ok(())
    }

    /// set blog theme.
    pub fn set_blog_theme(&mut self, name: &str) -> Result<()> {
        let theme_path = self.theme_root_dir()?.join(name);
        if !theme_path.exists() || !theme_path.is_dir() {
            return Err(Error::ThemeNotFound(name.into()));
        }
        self.settings.theme = name.to_string();
        self.export_config()?;
        Ok(())
    }
}

/// check directory entry is a hidden file.
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

/// check directory entry is an markdown file.
fn is_markdown_file(entry: &DirEntry) -> bool {
    if !entry.path().is_file() {
        return false;
    }
    let fname = entry.file_name().to_str();
    match fname {
        None => {
            return false;
        }
        Some(s) => {
            if s.starts_with(|c| (c == '.') | (c == '~')) {
                return false;
            }
            return s.ends_with(".md");
        }
    }
}

/// create a directory pathbuf from setting config.
fn get_dir<P: AsRef<Path>>(root: P, value: &str) -> Result<PathBuf> {
    let expanded_path = shellexpand::full(value)?.into_owned();
    let dir = PathBuf::from(expanded_path);
    if dir.is_relative() {
        return Ok(root.as_ref().join(&dir));
    } else {
        return Ok(dir);
    }
}

fn format_page_name(prefix: &str, page: usize, total: usize) -> String {
    if page == 0 || page > total {
        return String::default();
    }
    let mut s = String::with_capacity(prefix.len() + 10);
    s.push_str(prefix);
    if page > 1 {
        s.push_str(&format!("-{}", page));
    }
    s.push_str(".html");
    s
}
