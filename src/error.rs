use std::error::Error as StdError;
use std::path::PathBuf;

use derive_more::{Display, From};

/// Result type
pub type Result<T> = std::result::Result<T, Error>;

/// The Error type
#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "IO error")]
    Io(std::io::Error),
    #[display(fmt = "Path strip prefix error")]
    PathStripPrefix(std::path::StripPrefixError),
    #[display(fmt = "Config merge error")]
    ConfigMerge(config::ConfigError),
    #[display(fmt = "Template error")]
    Template(tera::Error),
    #[display(fmt = "Notify error")]
    Notify(notify::Error),
    #[display(fmt = "Glob pattern error")]
    GlobPattern(glob::PatternError),
    #[display(fmt = "Toml export error")]
    TomlExport(toml::ser::Error),
    #[display(fmt = "Path expand error")]
    PathExpend(shellexpand::LookupError<std::env::VarError>),
    #[display(fmt = "Post head parse error, please use yaml grammar")]
    PostHeadPaser(serde_yaml::Error),

    #[from(ignore)]
    #[display(fmt = "blog root directory {:?} already exists", _0)]
    RootDirExisted(PathBuf),
    #[from(ignore)]
    #[display(
        fmt = "post path {:?} format error: must be relative path without file extension",
        _0
    )]
    PostPathInvaild(PathBuf),
    #[from(ignore)]
    #[display(fmt = "post path {:?} already existed", _0)]
    PostPathExisted(PathBuf),
    #[from(ignore)]
    #[display(fmt = "Theme template file {:?} encoding error", _0)]
    ThemeFileEncoding(String),
    #[from(ignore)]
    #[display(fmt = "blog theme {:?} in use, can not be deleted", _0)]
    ThemeInUse(String),
    #[from(ignore)]
    #[display(fmt = "blog theme {:?} not found", _0)]
    ThemeNotFound(String),
    #[from(ignore)]
    #[display(
        fmt = "post {:?} must has two parts: headers and body, splitted by first blank line",
        _0
    )]
    PostOnlyOnePart(PathBuf),
    #[from(ignore)]
    #[display(fmt = "post {:?} head part is empty", _0)]
    PostNoHead(PathBuf),
    #[from(ignore)]
    #[display(fmt = "post {:?} body part is empty", _0)]
    PostNoBody(PathBuf),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;
        match self {
            Io(e) => Some(e),
            PathStripPrefix(e) => Some(e),
            ConfigMerge(e) => Some(e),
            Template(e) => Some(e),
            Notify(e) => Some(e),
            GlobPattern(e) => Some(e),
            TomlExport(e) => Some(e),
            PathExpend(e) => Some(e),
            PostHeadPaser(e) => Some(e),
            RootDirExisted(_) => None,
            PostPathInvaild(_) => None,
            PostPathExisted(_) => None,
            ThemeFileEncoding(_) => None,
            ThemeInUse(_) => None,
            ThemeNotFound(_) => None,
            PostOnlyOnePart(_) => None,
            PostNoHead(_) => None,
            PostNoBody(_) => None,
        }
    }
}
