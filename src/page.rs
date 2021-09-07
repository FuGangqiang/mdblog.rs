use std::rc::Rc;
use serde::Serialize;

use crate::post::Post;

/// blog page
///
/// index page or tag page
#[derive(Serialize)]
pub struct Page {
    /// page index, start from 1
    pub index: usize,
    /// page index name
    pub name: String,
    /// page posts array
    pub posts: Vec<Rc<Post>>
}
