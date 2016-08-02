use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use utils::{create_file, render_html, create_error};
use theme::Theme;


pub struct Post {
    pub root: PathBuf,
    pub path: PathBuf,
    pub head: String,
    pub body: String,
    pub metadata: HashMap<String, String>,
}


impl Post {
    pub fn new<P: AsRef<Path>>(root: P, path: P) -> Post {
        Post {
            root: root.as_ref().to_owned(),
            path: path.as_ref().to_owned(),
            head: String::new(),
            body: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn dest(&self) -> PathBuf {
        self.root.join("builds").join(self.path.with_extension("html"))
    }

    pub fn load(&mut self) -> ::std::io::Result<()> {
        debug!("loading post: {}", self.path.display());
        let mut pf = File::open(self.root.join(&self.path))?;
        let mut content = String::new();
        pf.read_to_string(&mut content)?;
        let v: Vec<&str> = content.splitn(2, "\n\n").collect();
        if v.len() != 2 {
            return create_error(format!("post({path}) must both have `head` and `body` parts",
                                        path=self.path.display()));
        }
        self.head = v[0].to_string();
        self.body = v[1].to_string();
        for line in self.head.lines() {
            let pair: Vec<&str> = line.splitn(2, ':').collect();
            if pair.len() != 2 {
                return create_error(format!("post({path}) `head` part parse error: {line}",
                                            path=self.path.display(),
                                            line=line));
            }
            self.metadata.insert(pair[0].trim().to_owned(), pair[1].trim().to_owned());
        }
        Ok(())
    }

    pub fn render_html(&self, theme: &Theme) -> ::std::io::Result<()> {
        debug!("rendering post: {}", self.path.display());
        let dest = self.dest();
        let f = create_file(&dest)?;
        debug!("created html: {:?}", dest.display());
        Ok(())
    }
}
