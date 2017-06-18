use chrono::{DateTime, Local, TimeZone};
use pulldown_cmark::{OPTION_ENABLE_TABLES, Options, Parser, html};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::path::{Path, PathBuf};
use super::{Result, ErrorKind};

/// blog post object
///
/// every blog post is composed of `head` part and `body` part.
/// the two part is separated by the first blank line.
///
/// the blog header part supported headers:
///
/// * date: the publish datetime, required, `date: 1970-01-01 00:00:00`
/// * tags: the tags of blog post, required, `tags: hello, world`
/// * hidden: whether hidden blog post or not, optional, default `true`, `hidden: false`
pub struct Post {
    /// root path of blog
    root: PathBuf,
    /// relative path of post from blog root directory
    pub path: PathBuf,
    /// post origin head part
    head: String,
    /// post origin body part
    body: String,
    /// headers from parsing the post origin head part
    metadata: HashMap<String, String>,
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

    /// the absolute path of blog post markdown file
    pub fn src(&self) -> PathBuf {
        self.root.join(&self.path)
    }

    /// the absolute path of blog post html file
    pub fn dest(&self) -> PathBuf {
        self.root
            .join("_builds/blog")
            .join(self.path.with_extension("html"))
    }

    /// blog title
    pub fn title(&self) -> &str {
        self.path
            .file_stem()
            .and_then(|x| x.to_str())
            .expect(&format!("post filename format error: {}", self.path.display()))
    }

    /// blog publish time
    pub fn datetime(&self) -> DateTime<Local> {
        let date_value = self.metadata
                             .get("date")
                             .expect(&format!("post({}) require date header",
                                             &self.path.display()));
        match Local.datetime_from_str(&date_value, "%Y-%m-%d %H:%M:%S") {
            Ok(datetime) => datetime,
            Err(why) => {
                panic!("post({}) date header parse error: {}",
                       &self.path.display(),
                       why.description())
            },
        }
    }

    /// wether blog post is hidden or not
    pub fn is_hidden(&self) -> Result<bool> {
        let hidden_value = self.metadata
                               .get("hidden")
                               .unwrap_or(&"false".to_string())
                               .to_lowercase();
        match hidden_value.as_ref() {
            "false" | "f" => Ok(false),
            "true" | "t" => Ok(true),
            _ => bail!(ErrorKind::PostHead(self.path.clone())),
        }
    }

    /// the post url
    pub fn url(&self) -> PathBuf {
        Path::new("/blog").join(&self.path).with_extension("html")
    }

    /// the rendered html content of post body port
    pub fn content(&self) -> String {
        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        let mut s = String::with_capacity(self.body.len() * 3 / 2);
        let p = Parser::new_ext(&self.body, opts);
        html::push_html(&mut s, p);
        s
    }

    /// the post tags
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

    /// post context for render
    pub fn map(&self) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("title".to_string(), Value::String(self.title().to_string()));
        map.insert("url".to_string(),
                   Value::String(format!("{}", self.url().display())));
        map.insert("datetime".to_string(),
                   Value::String(self.datetime().format("%Y-%m-%d").to_string()));

        map
    }

    /// load post head part and body part
    pub fn load(&mut self) -> Result<()> {
        debug!("loading post: {}", self.path.display());
        let mut pf = File::open(self.src())?;
        let mut content = String::new();
        pf.read_to_string(&mut content)?;
        let v: Vec<&str> = content.splitn(2, "\n\n").collect();
        if v.len() != 2 {
            bail!(ErrorKind::PostNoBody(self.path.clone()));
        }
        if v[0].trim().is_empty() {
            bail!(ErrorKind::PostHead(self.path.clone()));
        }
        if v[1].trim().is_empty() {
            bail!(ErrorKind::PostNoBody(self.path.clone()));
        }
        self.head = v[0].to_string();
        self.body = v[1].to_string();
        for line in self.head.lines() {
            let pair: Vec<&str> = line.splitn(2, ':').collect();
            if pair.len() != 2 {
                bail!(ErrorKind::PostHead(self.path.clone()));
            }
            self.metadata
                .insert(pair[0].trim().to_owned(), pair[1].trim().to_owned());
        }
        Ok(())
    }
}
