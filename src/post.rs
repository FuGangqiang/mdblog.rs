use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use utils::{create_file, create_error};
use theme::Theme;
use tera::{Tera, Context};
use std::error::Error;
use pulldown_cmark::{html, Parser, Options, OPTION_ENABLE_TABLES};


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

    pub fn src(&self) -> PathBuf {
        self.root.join(&self.path)
    }

    pub fn dest(&self) -> PathBuf {
        self.root.join("builds").join(self.path.with_extension("html"))
    }

    pub fn content(&self) -> String {
        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        let mut s = String::with_capacity(self.body.len() * 3 / 2);
        let p = Parser::new_ext(&self.body, opts);
        html::push_html(&mut s, p);
        s
    }

    pub fn tags(&self) -> Vec<&str> {
        if let Some(tag_str) = self.metadata.get("tags") {
            tag_str.split(',').map(|x| x.trim()).filter(|x| x.len() != 0).collect()
        } else {
            Vec::new()
        }
    }

    pub fn load(&mut self) -> ::std::io::Result<()> {
        debug!("loading post: {}", self.path.display());
        let mut pf = File::open(self.src())?;
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

    pub fn render_html(&self, tera: &Tera) -> ::std::io::Result<()> {
        debug!("rendering post: {}", self.path.display());
        let dest = self.dest();
        let mut f = create_file(&dest)?;
        let mut context = Context::new();
        let content = self.content();
        context.add("content", &content);
        match tera.render("post.tpl", context) {
            Ok(s) => {
                f.write(s.as_bytes())?;
                debug!("created html: {:?}", dest.display());
                return Ok(());
            }
            Err(e) => {
                return create_error(format!("post({path}) render error: {descrition}",
                                            path=self.path.display(),
                                            descrition=e.description()));
            }
        }
    }
}
