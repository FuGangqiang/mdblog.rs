#![feature(question_mark)]

#[macro_use]
extern crate log;
extern crate pulldown_cmark;
extern crate serde_json;
extern crate tera;
extern crate walkdir;

mod post;
mod theme;
mod utils;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use serde_json::Map;
use tera::{Tera, Context};
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use post::Post;
use theme::Theme;
use utils::{create_error, create_file};


pub struct Mdblog {
    root: PathBuf,
    theme: Theme,
    posts: Vec<Rc<Post>>,
    tags: BTreeMap<String, Vec<Rc<Post>>>,
    renderer: Option<Tera>,
}


impl Mdblog {
    pub fn new<P: AsRef<Path>>(root: P) -> Mdblog {
        Mdblog {
            root: root.as_ref().to_owned(),
            theme: Theme::new(&root),
            posts: Vec::new(),
            tags: BTreeMap::new(),
            renderer: None,
        }
    }

    pub fn init(&self) -> ::std::io::Result<()> {
        if self.root.exists() {
            return create_error(format!("{root} directory already existed.", root=self.root.display()));
        }

        let mut hello_post = create_file(&self.root.join("posts").join("hello.md"))?;
        hello_post.write_all(b"date: 1970-01-01\n")?;
        hello_post.write_all(b"tags: hello, world\n")?;
        hello_post.write_all(b"\n")?;
        hello_post.write_all(b"# hello\n\nhello world!\n")?;

        let mut config = create_file(&self.root.join("config.toml"))?;
        config.write_all(b"[blog]\ntheme = simple\n")?;

        let mut theme = Theme::new(&self.root);
        theme.load("simple")?;
        theme.init_dir()?;

        fs::create_dir_all(self.root.join("media"))?;

        Ok(())
    }

    pub fn build(&mut self, theme: &str) -> ::std::io::Result<()> {
        self.load_theme(&theme)?;
        self.load_posts()?;
        self.export()?;
        Ok(())
    }

    pub fn server(&self, port: u16) {
        println!("server blog at localhost:{}", port);
    }

    pub fn load_theme(&mut self, theme: &str) -> ::std::io::Result<()> {
        self.theme.load(theme)?;
        let template_dir = self.root.join("_themes").join(&theme).join("templates");
        debug!("template dir: {}", template_dir.display());
        self.renderer = Some(Tera::new(&format!("{}/*", template_dir.display())));
        Ok(())
    }

    pub fn load_posts(&mut self) -> ::std::io::Result<()> {
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
            for tag in post.tags() {
                let mut ps = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                ps.push(post.clone());
            }
            self.posts.push(post.clone());
        }
        self.posts.sort_by(|p1, p2| p2.datetime().cmp(&p1.datetime()));
        for (_, tag_posts) in self.tags.iter_mut() {
            tag_posts.sort_by(|p1, p2| p2.datetime().cmp(&p1.datetime()));
        }
        debug!("Tags: {:?}", self.tags.keys().collect::<Vec<&String>>());
        Ok(())
    }

    pub fn export(&self) -> ::std::io::Result<()> {
        self.export_media()?;
        self.export_static()?;
        self.export_posts()?;
        self.export_index()?;
        self.export_tags()?;
        Ok(())
    }

    pub fn media_dest<P: AsRef<Path>>(&self, media: P) -> PathBuf {
        let relpath = media.as_ref().strip_prefix(&self.root.join("media")).expect("create post path error").to_owned();
        self.root.join("_builds/media").join(relpath)
    }

    pub fn export_media(&self) -> ::std::io::Result<()> {
        let walker = WalkDir::new(&self.root.join("media")).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.expect("get walker entry error");
            let src_path = entry.path();
            if src_path.is_dir() {
                fs::create_dir_all(self.media_dest(src_path))?;
                continue;
            }
            fs::copy(src_path, self.media_dest(src_path))?;
        }
        Ok(())
    }

    pub fn export_static(&self) -> ::std::io::Result<()> {
        self.theme.export_static()
    }

    pub fn export_posts(&self) -> ::std::io::Result<()> {
        for post in &self.posts {
            let dest = post.dest();
            let mut f = create_file(&dest)?;
            match self.render_post(post) {
                Ok(s) => {
                    f.write(s.as_bytes())?;
                    debug!("created html: {:?}", dest.display());
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn export_index(&self) -> ::std::io::Result<()> {
        let dest = self.root.join("_builds/index.html");
        let mut f = create_file(&dest)?;
        match self.render_index() {
            Ok(s) => {
                f.write(s.as_bytes())?;
                debug!("created html: {:?}", dest.display());
            },
            Err(e) => {
                return Err(e);
            }
        }
    Ok(())
    }

    pub fn export_tags(&self) -> ::std::io::Result<()> {
        for tag in self.tags.keys() {
            let dest = self.root.join(format!("_builds/blog/tags/{}.html", tag));
            let mut f = create_file(&dest)?;
            match self.render_tag(tag) {
                Ok(s) => {
                    f.write(s.as_bytes())?;
                    debug!("created html: {:?}", dest.display());
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn tag_url(&self, name: &str) -> String {
        format!("/blog/tags/{}.html", &name)
    }

    fn tag_map<T>(&self, name:&str, posts: &Vec<T>) -> Map<&str, String> {
        let mut map = Map::new();
        map.insert("name", name.to_string());
        let tag_len = format!("{:?}", &posts.len());
        map.insert("num", tag_len);
        map.insert("url", self.tag_url(&name));
        map
    }

    pub fn base_context(&self, title: &str) -> Context {
        let mut context = Context::new();
        context.add("title", &title);
        let mut all_tags = Vec::new();
        for (tag_key, tag_posts) in &self.tags {
            all_tags.push(self.tag_map(&tag_key, &tag_posts));
        }
        context.add("all_tags", &all_tags);

        context
    }

    pub fn render_post(&self, post: &Post) -> ::std::io::Result<String> {
        debug!("rendering post: {}", post.path.display());
        let tera = self.renderer.as_ref().expect("get renderer error");
        let mut context = self.base_context(&post.title());
        context.add("content", &post.content());
        context.add("datetime", &post.datetime().to_string());
        let mut post_tags = Vec::new();
        for tag_key in post.tags() {
            let tag_posts = self.tags.get(tag_key).expect(&format!("post tag({}) does not add to blog tags", tag_key));
            post_tags.push(self.tag_map(&tag_key, &tag_posts));
        }
        context.add("post_tags", &post_tags);

        match tera.render("post.tpl", context) {
            Ok(s) => {
                return Ok(s);
            },
            Err(e) => {
                return create_error(format!("post({path}) render error: {descrition}",
                                            path=post.path.display(),
                                            descrition=e.description()));
            }
        }
    }

    pub fn render_index(&self) -> ::std::io::Result<String> {
        debug!("rendering index");
        let tera = self.renderer.as_ref().expect("get renderer error");
        let mut context = self.base_context("Fu");
        context.add("posts", &self.posts_maps(&self.posts));
        match tera.render("index.tpl", context) {
            Ok(s) => {
                return Ok(s);
            },
            Err(e) => {
                return create_error(format!("index render error: {}", descrition=e.description()));
            }
        }
    }

    fn posts_maps<'a>(&self, posts: &'a Vec<Rc<Post>>) -> Vec<Map<&'a str, String>> {
        let mut maps = Vec::new();
        for post in posts {
            maps.push(post.map());
        }

        maps
    }

    pub fn render_tag(&self, tag:&str) -> ::std::io::Result<String> {
        debug!("rendering tag: {}", tag);
        let tera = self.renderer.as_ref().expect("get renderer error");
        let mut context = self.base_context(&tag);
        let posts = self.tags.get(tag).expect(&format!("get tag({}) error", &tag));
        context.add("posts", &self.posts_maps(&posts));
        match tera.render("tag.tpl", context) {
            Ok(s) => {
                return Ok(s);
            },
            Err(e) => {
                return create_error(format!("tag({tag}) render error: {descrition}",
                                            tag=tag,
                                            descrition=e.description()));
            }
        }
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
        }
    }
}
