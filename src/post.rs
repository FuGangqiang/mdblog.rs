use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde_yaml;
use chrono::{DateTime, Local};

use errors::{Error, Result};
use utils::markdown_to_html;

/// blog post
///
/// every blog post is composed of `head` part and `body` part.
/// the two part is separated by the first blank line.
#[derive(Serialize)]
pub struct Post {
    /// blog root path
    root: PathBuf,
    /// post path from relative root directory
    pub path: PathBuf,
    /// the post title
    pub title: String,
    /// the post url
    pub url: PathBuf,
    /// post headers
    pub headers: PostHeaders,
    /// post html body
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostHeaders {
    /// post created time, `date: 1970-01-01 00:00:00`
    pub created: DateTime<Local>,
    /// post hidden flag, `hidden: true`
    pub hidden: Option<bool>,
    /// post tags, `tags: hello, world`
    pub tags: Vec<String>,
}

impl Post {
    pub fn new<P: AsRef<Path>>(root: P, path: P) -> Result<Post> {
        let root = root.as_ref();
        let path = path.as_ref();
        debug!("loading post: {}", path.display());

        let fp = root.join(&path);
        let mut fo = File::open(fp)?;
        let mut content = String::new();
        fo.read_to_string(&mut content)?;

        let v: Vec<&str> = content.splitn(2, "\n\n").collect();
        if v.len() != 2 {
            return Err(Error::PostOnlyOnePart(path.into()));
        }
        let head = v[0].trim();
        let body = v[1].trim();
        if head.is_empty() {
            return Err(Error::PostNoHead(path.into()));
        }
        if head.is_empty() {
            return Err(Error::PostNoBody(path.into()));
        }

        let title = path.file_stem()
                        .and_then(|x| x.to_str())
                        .expect(&format!("post filename format error: {}", path.display()));
        let url = Path::new("/blog").join(path).with_extension("html");
        let headers: PostHeaders = serde_yaml::from_str(head)?;
        let content = markdown_to_html(body);

        Ok(Post {
            root: root.to_owned(),
            path: path.to_owned(),
            title: title.to_owned(),
            url: url,
            headers: headers,
            content: content,
        })
    }

    /// the absolute path of blog post markdown file
    pub fn src(&self) -> PathBuf {
        self.root.join(&self.path)
    }

    /// the absolute path of blog post html file
    pub fn dest(&self) -> PathBuf {
        Path::new("blog").join(&self.path).with_extension("html")
    }

    pub fn is_hidden(&self) -> bool {
        self.headers.hidden.unwrap_or(false)
    }
}
