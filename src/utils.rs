use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use failure::Fail;
use log::error;
use pulldown_cmark::{html, Options, Parser};

use crate::errors::{Error, Result};

/// create the file of `path` and append content
///
/// if parent of `path` does not existed, create it first.
pub fn write_file(path: &Path, buf: &[u8]) -> Result<()> {
    if let Some(p) = path.parent() {
        ::std::fs::create_dir_all(p)?;
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
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    let mut s = String::with_capacity(content.len() * 3 / 2);
    let p = Parser::new_ext(content, opts);
    html::push_html(&mut s, p);
    s
}

/// log mdblog error chain
pub fn log_error(err: &Error) {
    for cause in (err as &Fail).iter_chain() {
        error!("{}", cause);
    }

    if let Some(backtrace) = err.backtrace() {
        error!("backtrace: {:?}", backtrace);
    }
}
