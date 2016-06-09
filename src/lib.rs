#![feature(question_mark)]

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, Error, ErrorKind};


pub struct Mdblog {
    root: PathBuf,
}


impl Mdblog {
    pub fn new<P: AsRef<Path>>(root: P) -> Mdblog {
        Mdblog {
            root: root.as_ref().to_owned(),
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

    pub fn build(&self, theme: &str) -> ::std::io::Result<()> {
        Ok(())
    }

    pub fn server(&self, port: u16) {
        println!("server blog at localhost:{}", port);
    }
}
