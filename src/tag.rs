use std::rc::Rc;

use serde::Serialize;

use crate::post::Post;

/// blog tag
#[derive(Serialize)]
pub struct Tag {
    /// tag name
    pub name: String,
    /// the number of tag posts
    pub num: isize,
    /// the posts
    pub posts: Vec<Rc<Post>>,
}

impl Tag {
    /// create new `Tag`
    pub fn new(name: &str) -> Tag {
        Tag {
            name: name.to_string(),
            num: 0,
            posts: Vec::new(),
        }
    }

    /// add a post to `Tag`
    pub fn add(&mut self, post: Rc<Post>) {
        self.num += 1;
        self.posts.push(post);
    }
}
