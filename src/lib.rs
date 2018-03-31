//! static site generator from markdown files.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://docs.rs/mdblog")]

extern crate chrono;
extern crate config;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate hyper;
extern crate futures;
extern crate pulldown_cmark;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;
extern crate tera;
extern crate walkdir;
extern crate open;
extern crate notify;
extern crate glob;
extern crate mime_guess;
extern crate shellexpand;
extern crate percent_encoding;

mod errors;
mod settings;
mod post;
mod theme;
mod tag;
mod utils;
mod service;

use std::thread;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;

use glob::Pattern;
use chrono::Local;
use hyper::server::Http;
use tera::{Context, Tera};
use walkdir::{DirEntry, WalkDir};
use notify::{DebouncedEvent, RecursiveMode, Watcher, watcher};

pub use errors::{Error, Result};
pub use settings::Settings;
pub use theme::Theme;
pub use post::Post;
pub use tag::Tag;
pub use post::PostHeaders;
pub use utils::log_error;
use config::Config;
use service::HttpService;
use utils::write_file;

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
    /// tags map
    tags_map: BTreeMap<String, Tag>,
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
            settings: settings,
            theme: theme,
            posts: Vec::new(),
            tags_map: BTreeMap::new(),
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
        if self.settings.url_prefix.ends_with('/') {
            self.settings.url_prefix = self.settings.url_prefix.trim_right_matches('/').to_string();
        }
        Ok(())
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
            if post.is_hidden() {
               continue;
            }
            let post = Rc::new(post);
            posts.push(post.clone());
            for tag_name in &post.headers.tags {
                let mut tag = tags_map.entry(tag_name.to_string())
                                      .or_insert(Tag::new(tag_name, &self.tag_url(tag_name)));
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

    /// init blog directory.
    pub fn init(&mut self) -> Result<()> {
        if self.root.exists() {
            return Err(Error::RootDirExisted(self.root.clone()));
        }

        let mut tera = Tera::default();
        tera.add_raw_template("hello.md.tpl", include_str!("demo_templates/hello.md.tpl"))?;
        tera.add_raw_template("math.md.tpl", include_str!("demo_templates/math.md.tpl"))?;

        let now = Local::now();
        let mut context = Context::new();
        context.add("now", &now.format("%Y-%m-%dT%H:%M:%S%:z").to_string());

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
        self.export_media()?;
        self.export_static()?;
        self.export_posts()?;
        self.export_index()?;
        self.export_tags()?;
        Ok(())
    }

    /// serve the blog static files in the `build_dir` directory.
    pub fn serve(&mut self, port: u16) -> Result<()> {
        let addr_str = format!("127.0.0.1:{}", port);
        let mut server_url = format!("http://{}", &addr_str);
        server_url.push_str(&self.settings.url_prefix);
        let addr = addr_str.parse()?;
        let build_dir = self.build_root_dir()?;
        let url_prefix = self.settings.url_prefix.clone();
        info!("server blog at {}", server_url);

        let child = thread::spawn(move || {
            let server = Http::new()
                .bind(&addr, move || {
                    Ok(HttpService {
                        root: build_dir.clone(),
                        url_prefix: url_prefix.clone(),
                    })})
                .expect("server start error");
            server.run().unwrap();
        });

        open::that(server_url)?;
        self.watch()?;
        child.join().expect("Couldn't join the server thread");

        Ok(())
    }

    /// watch blog files, rebuild blog when some files modified.
    fn watch(&mut self) -> Result<()> {
        let (tx, rx) = channel();
        let ignore_patterns = self.ignore_patterns()?;
        info!("watching dir: {}", self.root.display());
        let mut watcher = watcher(tx, Duration::new(2, 0))?;
        watcher.watch(&self.root, RecursiveMode::Recursive)?;
        let interval = Duration::new(self.settings.rebuild_interval as u64, 0);
        let mut last_run: Option<Instant> = None;
        loop {
            match rx.recv() {
                Err(why) => error!("watch error: {:?}", why),
                Ok(event) => {
                    match event {
                        DebouncedEvent::Create(ref fpath) |
                        DebouncedEvent::Write(ref fpath)  |
                        DebouncedEvent::Remove(ref fpath) |
                        DebouncedEvent::Rename(ref fpath, _) => {
                            if ignore_patterns.iter().any(|ref pat| pat.matches_path(fpath)) {
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
                            info!("Rebuild blog again...");
                            if let Err(ref e) = self.load_posts() {
                                log_error(e);
                                continue
                            }
                            if let Err(ref e) = self.build() {
                                log_error(e);
                                continue
                            }
                            info!("Rebuild done!");
                        },
                        _ => {},
                    }
                },
            }
        }
        #[allow(unreachable_code)]
        Ok(())
    }

    /// blog build directory absolute path.
    pub fn build_root_dir(&self) -> Result<PathBuf> {
        get_dir(&self.root, &self.settings.build_dir)
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
        let build_dir = self.build_root_dir()?
                            .to_str()
                            .expect("get build dir error")
                            .to_string();
        patterns.push(Pattern::new(&format!("{}/**/*", build_dir.trim_right_matches("/")))?);
        Ok(patterns)
    }

    /// create a new sample post.
    pub fn create_post(&self, path: &Path, tags: &Vec<String>) -> Result<()> {
        let post_title = path.file_stem();
        if !path.is_relative()
            || path.extension().is_some()
            || path.to_str().unwrap_or("").is_empty()
            || post_title.is_none()
            || self.ignore_patterns()?.iter().any(|ref pat| pat.matches_path(path)) {
            return Err(Error::PostPathInvaild(path.to_owned()));
        }
        if path.is_dir() {
            return Err(Error::PostPathExisted(path.to_owned()));
        }
        let post_path = self.post_root_dir()?.join(path).with_extension("md");
        if post_path.exists() {
            return Err(Error::PostPathExisted(path.to_owned()));
        }
        let now = Local::now();
        let content = format!("created: {}\n\
                               tags: [{}]\n\
                               \n\
                               this is a new post!\n",
                              now.format("%Y-%m-%dT%H:%M:%S%:z"),
                              tags.join(", "));
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
        let rel_path = media.as_ref()
            .strip_prefix(&self.media_root_dir()?)?
            .to_owned();
        Ok(build_dir.join(rel_path))
    }

    /// export blog media files.
    pub fn export_media(&self) -> Result<()> {
        debug!("exporting media ...");
        let walker = WalkDir::new(&self.media_root_dir()?).into_iter();
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
        let dest = build_dir.join("index.html");
        let html = self.render_index()?;
        write_file(&dest, html.as_bytes())?;
        Ok(())
    }

    /// export blog tag index pages.
    pub fn export_tags(&self) -> Result<()> {
        let build_dir = self.build_root_dir()?;
        for tag in self.tags_map.keys() {
            let dest = build_dir.join(format!("blog/tags/{}.html", tag));
            let html = self.render_tag(tag)?;
            write_file(&dest, html.as_bytes())?;
        }
        Ok(())
    }

    /// get base context of `theme.renderer` templates
    fn get_base_context(&self) -> Result<Context> {
        let mut context = Context::new();
        context.add("site_name", &self.settings.site_name);
        context.add("site_motto", &self.settings.site_motto);
        context.add("footer_note", &self.settings.footer_note);
        context.add("url_prefix", &self.settings.url_prefix);
        context.add("all_tags", &self.tags_map.values().collect::<Vec<_>>());
        Ok(context)
    }

    /// blog tag url.
    fn tag_url(&self, name: &str) -> String {
        format!("/blog/tags/{}.html", &name)
    }

    /// render blog post html.
    pub fn render_post(&self, post: &Post) -> Result<String> {
        debug!("rendering post({}) ...", post.path.display());
        let post_tags = self.tags_map.iter()
            .filter(|&(name, _)| post.headers.tags.contains(name))
            .map(|(_, tag)| tag)
            .collect::<Vec<_>>();
        let mut context = self.get_base_context()?;
        context.add("post", &post);
        context.add("post_tags", &post_tags);
        Ok(self.theme.renderer.render("post.tpl", &context)?)
    }

    /// render index page html.
    pub fn render_index(&self) -> Result<String> {
        debug!("rendering index ...");
        let mut context = self.get_base_context()?;
        context.add("posts", &self.posts);
        Ok(self.theme.renderer.render("index.tpl", &context)?)
    }

    /// render tag pages html.
    pub fn render_tag(&self, tag: &str) -> Result<String> {
        debug!("rendering tag({}) ...", tag);
        let tag = self.tags_map.get(tag).expect(&format!("not found tag: {}", &tag));
        let mut context = self.get_base_context()?;
        context.add("tag", &tag);
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
                info!("* {}", path.file_name()
                                  .expect("theme name error")
                                  .to_str()
                                  .expect("theme name error"));
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
            return Err(Error::ThemeInUse(name.to_string()));
        }
        let theme_path = self.theme_root_dir()?.join(name);
        if !theme_path.exists() || !theme_path.is_dir() {
            return Err(Error::ThemeNotFound(name.to_string()));
        }
        std::fs::remove_dir_all(theme_path)?;
        Ok(())
    }

    /// set blog theme.
    pub fn set_blog_theme(&mut self, name: &str) -> Result<()> {
        let theme_path = self.theme_root_dir()?.join(name);
        if !theme_path.exists() || !theme_path.is_dir() {
            return Err(Error::ThemeNotFound(name.to_string()));
        }
        self.settings.theme = name.to_string();
        self.export_config()?;
        Ok(())
    }
}

/// check directory entry is a hidden file.
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
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
        },
        Some(s) => {
            if s.starts_with(|c| (c == '.') | (c == '~')) {
                return false;
            } else if s.ends_with(".md") {
                return true;
            } else {
                return false;
            }
        },
    }
}

/// create a directory pathbuf from setting config.
fn get_dir<P: AsRef<Path>>(root: P, value: &str) -> Result<PathBuf> {
    let expanded_path = shellexpand::full(value)?.into_owned();
    let dir = PathBuf::from(expanded_path.to_string());
    if dir.is_relative() {
        return Ok(root.as_ref().join(&dir));
    } else {
        return Ok(dir);
    }
}
