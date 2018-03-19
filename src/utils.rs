use errors::Result;
use std::fs::File;
use std::path::Path;

/// create the file of `path`
///
/// if parent of `path` does not existed, create it first.
pub fn create_file(path: &Path) -> Result<File> {
    if let Some(p) = path.parent() {
        ::std::fs::create_dir_all(p)?;
    }
    Ok(File::create(path)?)
}
