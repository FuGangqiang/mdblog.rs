use std::path::Path;
use std::fs::File;


pub fn create_file(path: &Path) -> ::std::io::Result<File> {
    if let Some(p) = path.parent() {
        ::std::fs::create_dir_all(p)?;
    }
    Ok(File::create(path)?)
}
