#![feature(question_mark)]

extern crate chrono;
#[macro_use]
extern crate log;
extern crate pulldown_cmark;
extern crate tera;
extern crate walkdir;

mod post;
mod theme;
mod utils;

use std::collections::BTreeMap;
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use tera::{Tera, Context};
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use post::Post;
use theme::Theme;
use utils::{create_error, create_file};


pub struct Mdblog {
    root: PathBuf,
    theme: Theme,
    publisheds: Vec<Rc<Post>>,
    modifieds: Vec<Rc<Post>>,
    tags: BTreeMap<String, Vec<Rc<Post>>>,
    renderer: Option<Tera>,
}


#[derive(Debug)]
pub enum Index {
    Publish,
    Modify,
}


impl Mdblog {
    pub fn new<P: AsRef<Path>>(root: P) -> Mdblog {
        Mdblog {
            root: root.as_ref().to_owned(),
            theme: Theme::new(&root),
            publisheds: Vec::new(),
            modifieds: Vec::new(),
            tags: BTreeMap::new(),
            renderer: None,
        }
    }

    pub fn init(&self) -> ::std::io::Result<()> {
        if self.root.exists() {
            return create_error(format!("{root} directory already existed.", root=self.root.display()));
        }

        let mut hello_post = create_file(&self.root.join("posts").join("hello.md"))?;
        hello_post.write_all(b"tags: hello, world\n")?;
        hello_post.write_all(b"\n")?;
        hello_post.write_all(b"# hello\n\nhello world!\n")?;

        let mut config = create_file(&self.root.join("config.toml"))?;
        config.write_all(b"[blog]\ntheme = simple\n")?;

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
        let template_dir = self.root.join("themes").join(&theme).join("templates");
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
            self.publisheds.push(post.clone());
            self.modifieds.push(post.clone());
        }
        self.publisheds.sort_by_key(|p| p.publish_datetime);
        self.modifieds.sort_by_key(|p| p.modify_datetime);
        debug!("Tags: {:?}", self.tags.keys().collect::<Vec<&String>>());
        Ok(())
    }

    pub fn export(&self) -> ::std::io::Result<()> {
        self.export_publisheds()?;
        self.export_indexs()?;
        self.export_tags()?;
        Ok(())
    }

    pub fn export_publisheds(&self) -> ::std::io::Result<()> {
        for post in &self.publisheds {
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

    pub fn export_indexs(&self) -> ::std::io::Result<()> {
        for index in &[Index::Publish, Index::Modify] {
            let dest = self.index_dest(index);
            let mut f = create_file(&dest)?;
            match self.render_index(index) {
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

    pub fn export_tags(&self) -> ::std::io::Result<()> {
        for tag in self.tags.keys() {
            let dest = self.root.join(format!("builds/blog/tags/{}.html", tag));
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

    pub fn render_post(&self, post: &Post) -> ::std::io::Result<String> {
        debug!("rendering post: {}", post.path.display());
        let tera = self.renderer.as_ref().expect("get renderer error");
        let mut context = Context::new();
        context.add("content", &post.content());
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

    pub fn index_dest(&self, index: &Index) -> PathBuf {
        match *index {
            Index::Publish => self.root.join("builds/index.html"),
            Index::Modify => self.root.join("builds/blog/modified.html"),
        }
    }

    pub fn render_index(&self, index: &Index) -> ::std::io::Result<String> {
        debug!("rendering index: {:?}", index);
        let tera = self.renderer.as_ref().expect("get renderer error");
        let mut context = Context::new();
        let posts = match *index {
            Index::Publish => &self.publisheds,
            Index::Modify => &self.modifieds,
        };
        context.add("content", &format!("{:?}", index));
        match tera.render("index.tpl", context) {
            Ok(s) => {
                return Ok(s);
            },
            Err(e) => {
                return create_error(format!("index({index:?}) render error: {descrition}",
                                            index=index,
                                            descrition=e.description()));
            }
        }
    }

    pub fn render_tag(&self, tag:&str) -> ::std::io::Result<String> {
        debug!("rendering tag: {}", tag);
        let tera = self.renderer.as_ref().expect("get renderer error");
        let mut context = Context::new();
        context.add("content", &tag);
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
