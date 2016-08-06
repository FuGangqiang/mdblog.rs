use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local, TimeZone};
use pulldown_cmark::{html, Parser, Options, OPTION_ENABLE_TABLES};

use utils::create_error;


pub struct Post {
    pub root: PathBuf,
    pub path: PathBuf,
    pub head: String,
    pub body: String,
    pub publish_datetime: DateTime<Local>,
    pub modify_datetime: DateTime<Local>,
    pub metadata: HashMap<String, String>,
}


impl Post {
    pub fn new<P: AsRef<Path>>(root: P, path: P) -> Post {
        let metadata = fs::metadata(root.as_ref().join(&path.as_ref())).expect("get post metadata error");
        let (ctime, ctime_nsec) = (metadata.ctime(), metadata.ctime_nsec());
        let (mtime, mtime_nsec) = (metadata.mtime(), metadata.mtime_nsec());
        Post {
            root: root.as_ref().to_owned(),
            path: path.as_ref().to_owned(),
            head: String::new(),
            body: String::new(),
            publish_datetime: Local.timestamp(ctime, ctime_nsec as u32),
            modify_datetime: Local.timestamp(mtime, mtime_nsec as u32),
            metadata: HashMap::new(),
        }
    }

    pub fn src(&self) -> PathBuf {
        self.root.join(&self.path)
    }

    pub fn dest(&self) -> PathBuf {
        self.root.join("builds/blog").join(self.path.with_extension("html"))
    }

    pub fn title(&self) -> &str {
        self.path
            .file_stem()
            .and_then(|x| x.to_str())
            .expect(&format!("post filename format error: {}", self.path.display()))
    }

    pub fn url(&self) -> PathBuf {
        Path::new("/blog").join(&self.path)
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
            let mut res = tag_str.split(',')
                                 .map(|x| x.trim())
                                 .filter(|x| x.len() != 0)
                                 .collect::<Vec<&str>>();
            res.sort();
            res
        } else {
            Vec::new()
        }
    }

    pub fn load(&mut self) -> ::std::io::Result<()> {
        debug!("loading post: {}", self.path.display());
        debug!("    published: {:?}", self.publish_datetime);
        debug!("    modified: {:?}", self.modify_datetime);
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
}
