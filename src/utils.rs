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

/// log error chain
pub fn log_error(err: &Error) {
    for cause in err.causes() {
        error!("{}", cause);
    }

    if let Some(backtrace) = err.backtrace() {
        error!("backtrace: {:?}", backtrace);
    }
}