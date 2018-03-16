//! create blog from markdown files.
//!
//! # features
//!
//! * markdown format
//! * TeX style math support
//! * highlight code block
//! * post tags index
//! * hidden post
//! * post title is the title of markdown file
//! * post url is some to path of markdown file

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://docs.rs/mdblog")]
#![recursion_limit = "1024"]

extern crate chrono;
extern crate config;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate pulldown_cmark;
extern crate serde_json;
extern crate tera;
extern crate walkdir;

mod error;
mod post;
mod theme;
mod utils;

use config::Config;
pub use error::*;
pub use post::Post;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tera::{Context, Tera};
pub use theme::Theme;
pub use utils::create_file;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

/// blog object
pub struct Mdblog {
    /// blog root path
    root: PathBuf,
    /// blog settings
    settings: Config,
    /// blog theme
    theme: Theme,
    /// blog render
    renderer: Tera,
    /// collection of blog posts
    posts: Vec<Rc<Post>>,
    /// tagged posts
    tags: BTreeMap<String, Vec<Rc<Post>>>,
}

impl Mdblog {
    /// create Mdblog from the `root` path
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Mdblog> {
        let root = root.as_ref();
        let settings = Mdblog::get_settings().unwrap();
        let theme_name: String = settings.get("theme").unwrap();
        let theme = Mdblog::get_theme(root, &theme_name).unwrap();
        let renderer = Mdblog::get_renderer(root, &theme_name).unwrap();
        Ok(Mdblog {
            root: root.to_owned(),
            settings: settings,
            theme: theme,
            renderer: renderer,
            posts: Vec::new(),
            tags: BTreeMap::new(),
        })
    }

    /// get settings
    pub fn get_settings() -> Result<Config> {
        let mut settings = Config::default();
        settings.set_default("theme", "simple").unwrap();
        settings.set_default("site_logo", "/static/img/logo.png").unwrap();
        settings.set_default("site_name", "Mdblog").unwrap();
        settings.set_default("site_motto", "Simple is Beautiful!").unwrap();
        settings.set_default("footer_note", "Keep It Simple, Stupid!").unwrap();
        Ok(settings)
    }

    pub fn load_settings(&mut self) -> Result<()> {
        self.settings.merge(config::File::with_name("Config.toml")).unwrap();
        self.settings.merge(config::Environment::with_prefix("BLOG")).unwrap();
        Ok(())
    }

    /// get theme
    pub fn get_theme<P: AsRef<Path>>(root: P, name: &str) -> Result<Theme> {
        let mut theme = Theme::new(root.as_ref());
        theme.load(name).unwrap();
        Ok(theme)
    }

    pub fn get_renderer<P: AsRef<Path>>(root: P, theme_name: &str) -> Result<Tera> {
        let template_dir = root.as_ref()
                               .join("_themes")
                               .join(theme_name)
                               .join("templates");
        debug!("template dir: {}", template_dir.display());
        let renderer = Tera::new(&format!("{}/*", template_dir.display()))?;
        Ok(renderer)
    }

    pub fn load(&mut self) -> Result<()> {
        self.load_settings().unwrap();
        let posts_dir = self.root.join("posts");
        let walker = WalkDir::new(&posts_dir).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.expect("get walker entry error");
            if !is_markdown_file(&entry) {
                continue;
            }
            let mut post = Post::new(&self.root,
                                     &entry.path()
                                           .strip_prefix(&self.root)
                                           .expect("create post path error")
                                           .to_owned());
            post.load()?;
            let post = Rc::new(post);
            self.posts.push(post.clone());
            if !post.is_hidden()? {
                for tag in post.tags() {
                    let mut ps = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                    ps.push(post.clone());
                }
            }
        }
        self.posts.sort_by(|p1, p2| p2.datetime().cmp(&p1.datetime()));
        for (_, tag_posts) in self.tags.iter_mut() {
            tag_posts.sort_by(|p1, p2| p2.datetime().cmp(&p1.datetime()));
        }
        Ok(())
    }

    /// init Mdblog with `theme`.
    ///
    /// theme directory is created at `root/_theme` directory.
    /// if `theme` is `None`, use the default theme(`simple`).
    pub fn init(&mut self) -> Result<()> {
        if self.root.exists() {
            bail!(ErrorKind::RootDirExisted(self.root.clone()));
        }

        let mut hello_post = create_file(&self.root.join("posts").join("hello.md"))?;
        hello_post.write_all(b"date: 1970-01-01 00:00:00\n")?;
        hello_post.write_all(b"tags: hello, world\n")?;
        hello_post.write_all(b"\n")?;
        hello_post.write_all(b"# hello\n\nhello world!\n")?;

        let mut config_file = create_file(&self.root.join("Config.toml"))?;
        config_file.write_all(b"theme = \"simple\"\n\
                                site_logo = \"/static/img/logo.png\"\n\
                                site_name = \"Mdblog\"\n\
                                site_motto = \"Simple is Beautiful!\"\n\
                                footer_note = \"Keep It Simple, Stupid!\"\n\
                                ")?;
        self.theme.load(&self.settings.get_str("theme").unwrap())?;
        self.theme.init_dir()?;
        std::fs::create_dir_all(self.root.join("media"))?;
        Ok(())
    }

    /// create the blog html files to `root/_build/` directory.
    ///
    /// if `theme` is `None`, use the default theme(`simple`).
    pub fn build(&mut self) -> Result<()> {
        self.export()?;
        Ok(())
    }

    /// unimplemented.
    pub fn server(&self, port: u16) {
        println!("server blog at localhost:{}", port);
    }

    pub fn export(&self) -> Result<()> {
        self.export_media()?;
        self.export_static()?;
        self.export_posts()?;
        self.export_index()?;
        self.export_tags()?;
        Ok(())
    }

    pub fn media_dest<P: AsRef<Path>>(&self, media: P) -> PathBuf {
        let relpath = media.as_ref()
                           .strip_prefix(&self.root.join("media"))
                           .expect("create post path error")
                           .to_owned();
        self.root.join("_builds/media").join(relpath)
    }

    pub fn export_media(&self) -> Result<()> {
        debug!("exporting media ...");
        let walker = WalkDir::new(&self.root.join("media")).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.expect("get walker entry error");
            let src_path = entry.path();
            if src_path.is_dir() {
                std::fs::create_dir_all(self.media_dest(src_path))?;
                continue;
            }
            std::fs::copy(src_path, self.media_dest(src_path))?;
        }
        Ok(())
    }

    pub fn export_static(&self) -> Result<()> {
        self.theme.export_static()
    }

    pub fn export_posts(&self) -> Result<()> {
        for post in &self.posts {
            let dest = post.dest();
            let mut f = create_file(&dest)?;
            let html = self.render_post(post)?;
            f.write(html.as_bytes())?;
        }
        Ok(())
    }

    pub fn export_index(&self) -> Result<()> {
        let dest = self.root.join("_builds/index.html");
        let mut f = create_file(&dest)?;
        let html = self.render_index()?;
        f.write(html.as_bytes())?;
        Ok(())
    }

    pub fn export_tags(&self) -> Result<()> {
        for tag in self.tags.keys() {
            let dest = self.root.join(format!("_builds/blog/tags/{}.html", tag));
            let mut f = create_file(&dest)?;
            let html = self.render_tag(tag)?;
            f.write(html.as_bytes())?;
        }
        Ok(())
    }

    fn tag_url(&self, name: &str) -> String {
        format!("/blog/tags/{}.html", &name)
    }

    fn tag_map<T>(&self, name: &str, posts: &Vec<T>) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("name".to_string(), Value::String(name.to_string()));
        let tag_len = format!("{:?}", &posts.len());
        map.insert("num".to_string(), Value::String(tag_len));
        map.insert("url".to_string(), Value::String(self.tag_url(&name)));
        map
    }

    pub fn base_context(&self, title: &str) -> Context {
        let mut context = Context::new();
        context.add("title", &title);
        context.add("site_logo", &self.settings.get_str("site_logo").unwrap());
        context.add("site_name", &self.settings.get_str("site_name").unwrap());
        context.add("site_motto", &self.settings.get_str("site_motto").unwrap());
        context.add("footer_note",
                    &self.settings.get_str("footer_note").unwrap());
        let mut all_tags = Vec::new();
        for (tag_key, tag_posts) in &self.tags {
            all_tags.push(self.tag_map(&tag_key, &tag_posts));
        }
        all_tags.sort_by(|a, b| {
                             a.get("name").unwrap()
                              .as_str()
                              .unwrap()
                              .to_lowercase()
                              .cmp(&b.get("name").unwrap().as_str().unwrap().to_lowercase())
                         });
        context.add("all_tags", &all_tags);
        context
    }

    pub fn render_post(&self, post: &Post) -> Result<String> {
        debug!("rendering post({}) ...", post.path.display());
        let mut context = self.base_context(&post.title());
        context.add("content", &post.content());
        let mut post_tags = Vec::new();
        if !post.is_hidden()? {
            context.add("datetime",
                        &post.datetime().format("%Y-%m-%d %H:%M:%S").to_string());
            for tag_key in post.tags() {
                let tag_posts = self.tags.get(tag_key)
                                    .expect(&format!("post tag({}) does not add to blog tags",
                                                     tag_key));
                post_tags.push(self.tag_map(&tag_key, &tag_posts));
            }
        } else {
            context.add("datetime", &"".to_string());
        }

        context.add("post_tags", &post_tags);
        self.renderer.render("post.tpl", &context)
            .chain_err(|| "Template `post.tpl` render error")
    }

    pub fn render_index(&self) -> Result<String> {
        debug!("rendering index ...");
        let mut context = self.base_context(&self.settings.get_str("site_name").unwrap());
        context.add("posts", &self.posts_maps(&self.posts));
        self.renderer.render("index.tpl", &context)
            .chain_err(|| "Template `index.tpl` render error")
    }

    fn posts_maps(&self, posts: &Vec<Rc<Post>>) -> Vec<Map<String, Value>> {
        let mut maps = Vec::new();
        for post in posts.iter().filter(|p| !p.is_hidden().unwrap()) {
            maps.push(post.map());
        }
        maps
    }

    pub fn render_tag(&self, tag: &str) -> Result<String> {
        debug!("rendering tag({}) ...", tag);
        let mut context = self.base_context(&tag);
        let posts = self.tags
                        .get(tag)
                        .expect(&format!("get tag({}) error", &tag));
        context.add("posts", &self.posts_maps(&posts));
        self.renderer
            .render("tag.tpl", &context)
            .chain_err(|| "Template `tag.tpl` render error")
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

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
