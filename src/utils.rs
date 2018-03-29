use errors::{Result, Error};
use std::fs::File;
use std::path::Path;
use failure::Fail;
use pulldown_cmark::{html, Options, Parser, OPTION_ENABLE_TABLES};

/// create the file of `path`
///
/// if parent of `path` does not existed, create it first.
pub fn create_file(path: &Path) -> Result<File> {
    if let Some(p) = path.parent() {
        ::std::fs::create_dir_all(p)?;
    }
    Ok(File::create(path)?)
}

/// the rendered html content of post body port
pub fn markdown_to_html(content: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    let mut s = String::with_capacity(content.len() * 3 / 2);
    let p = Parser::new_ext(content, opts);
    html::push_html(&mut s, p);
    s
}

/// log error chain
pub fn log_error(err: &Error) {
    for cause in err.causes() {
        error!("{}", cause);
    }

    if let Some(backtrace) = err.backtrace() {
        error!("backtrace: {:?}", backtrace);
    }
}
