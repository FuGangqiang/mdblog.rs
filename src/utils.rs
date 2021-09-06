use std::error::Error as StdError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use log::error;
use pulldown_cmark::{html, Options, Parser};

use crate::error::Result;

/// create the file of `path` and append content
///
/// if parent of `path` does not existed, create it first.
pub fn write_file(path: &Path, buf: &[u8]) -> Result<()> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    let mut file = File::create(path)?;
    file.write_all(buf)?;
    Ok(())
}

/// read the file content of `path` to `buf`
pub fn read_file<P: AsRef<Path>>(path: P, buf: &mut Vec<u8>) -> Result<()> {
    let mut f = File::open(path.as_ref())?;
    f.read_to_end(buf)?;
    Ok(())
}

/// the rendered html content of post body port
pub fn markdown_to_html(content: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let mut s = String::with_capacity(content.len() * 3 / 2);
    let p = Parser::new_ext(content, opts);
    html::push_html(&mut s, p);
    s
}

/// basic error reporting, including the "cause chain".
pub(crate) fn log_error_chain(mut e: &dyn StdError) {
    error!("error: {}", e);
    while let Some(source) = e.source() {
        error!("caused by: {}", source);
        e = source;
    }
}
