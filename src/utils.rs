use errors::{Result, Error};
use std::fs::File;
use std::path::Path;
use failure::Fail;

/// create the file of `path`
///
/// if parent of `path` does not existed, create it first.
pub fn create_file(path: &Path) -> Result<File> {
    if let Some(p) = path.parent() {
        ::std::fs::create_dir_all(p)?;
    }
    Ok(File::create(path)?)
}

/// print error chain
pub fn print_error(err: &Error) {
    eprintln!("error: {}", err);

    for cause in err.causes() {
        eprintln!("{}", cause);
    }

    if let Some(backtrace) = err.backtrace() {
        eprintln!("backtrace: {:?}", backtrace);
    }
}