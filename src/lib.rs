#![feature(question_mark)]

extern crate walkdir;

mod theme;
mod post;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use theme::Theme;
use post::Post;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};


pub struct Mdblog {
    root: PathBuf,
    theme: Theme,
    posts: Vec<Post>,
}


impl Mdblog {
    pub fn new<P: AsRef<Path>>(root: P) -> Mdblog {
        Mdblog {
            root: root.as_ref().to_owned(),
            theme: Theme::new(&root),
            posts: Vec::new(),
        }
    }

    pub fn init(&self) -> ::std::io::Result<()> {
        if self.root.exists() {
            return Err(Error::new(ErrorKind::Other,
                                  format!("{root} directory already existed.", root=self.root.display())));
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
        self.theme.load(theme)?;
        self.load_posts()?;
        Ok(())
    }

    pub fn server(&self, port: u16) {
        println!("server blog at localhost:{}", port);
    }

    pub fn load_posts(&mut self) -> ::std::io::Result<()> {
        let posts_dir = self.root.join("posts");
        let walker = WalkDir::new(&posts_dir).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.unwrap();
            if  entry.path() == posts_dir {
                continue;
            }
            self.posts.push(Post::new(&self.root,
                                      &entry.path().strip_prefix(&self.root).unwrap().to_owned()));
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
