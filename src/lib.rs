#![feature(question_mark)]

#[macro_use]
extern crate log;
extern crate walkdir;
extern crate tera;
extern crate pulldown_cmark;

mod theme;
mod post;
mod utils;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;
use utils::create_error;
use theme::Theme;
use post::Post;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use tera::{Tera};


pub struct Mdblog {
    root: PathBuf,
    theme: Theme,
    posts: Vec<Post>,
    renderer: Option<Tera>,
}


impl Mdblog {
    pub fn new<P: AsRef<Path>>(root: P) -> Mdblog {
        Mdblog {
            root: root.as_ref().to_owned(),
            theme: Theme::new(&root),
            posts: Vec::new(),
            renderer: None,
        }
    }

    pub fn init(&self) -> ::std::io::Result<()> {
        if self.root.exists() {
            return create_error(format!("{root} directory already existed.", root=self.root.display()));
        }
        ::std::fs::create_dir_all(&self.root)?;

        let posts_dir = self.root.join("posts");
        ::std::fs::create_dir(&posts_dir)?;

        let mut hello = File::create(posts_dir.join("hello.md"))?;
        hello.write_all(b"published: 2016-06-05 17:14:43\n")?;
        hello.write_all(b"tags: [hello]\n")?;
        hello.write_all(b"\n")?;
        hello.write_all(b"# hello\n\nhello world!\n")?;

        let mut config = File::create(self.root.join("config.toml"))?;
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

    pub fn export(&self) -> ::std::io::Result<()> {
        self.export_post_html()?;
        Ok(())
    }

    pub fn load_theme(&mut self, theme: &str) -> ::std::io::Result<()> {
        debug!("loading theme: {}", theme);
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
            let entry = entry.unwrap();
            if !is_markdown_file(&entry) {
                continue;
            }
            self.posts.push(Post::new(&self.root,
                                      &entry.path().strip_prefix(&self.root).unwrap().to_owned()));
        }
        for post in self.posts.iter_mut() {
            post.load()?;
        }
        Ok(())
    }

    pub fn export_post_html(&self) -> ::std::io::Result<()> {
        let tera = self.renderer.as_ref().unwrap();
        for post in &self.posts {
            post.render_html(tera)?;
        }
        Ok(())
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
