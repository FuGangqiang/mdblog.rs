use std::error::Error as StdError;
use std::path::PathBuf;

use derive_more::{Display, From};

/// Result type
pub type Result<T> = std::result::Result<T, Error>;

/// The Error type
#[derive(Debug, Display, From)]
pub enum Error {
    /// IO error
    #[display("IO error")]
    Io(std::io::Error),
    /// path strip prefix error
    #[display("path strip prefix error")]
    PathStripPrefix(std::path::StripPrefixError),
    /// config merge error
    #[display("config merge error")]
    ConfigMerge(config::ConfigError),
    /// template error
    #[display("template error")]
    Template(tera::Error),
    ///datetime fetch error
    #[display("datetime fetch error")]
    DateTimeFetchError(time::error::IndeterminateOffset),
    /// datetime format error
    #[display("datetime format error")]
    DateTimeFormat(time::error::Format),
    /// notify error
    #[display("notify error")]
    Notify(notify::Error),
    /// glob pattern error
    #[display("glob pattern error")]
    GlobPattern(glob::PatternError),
    /// toml export error
    #[display("toml export error")]
    TomlExport(toml::ser::Error),
    /// path expand error
    #[display("path expand error")]
    PathExpend(shellexpand::LookupError<std::env::VarError>),
    /// post head parse error
    #[display("{:?}: post head parse error, please use yaml grammar", _1)]
    PostHeadPaser(serde_yaml::Error, PathBuf),

    /// blog root directory already exists error
    #[from(ignore)]
    #[display("blog root directory {:?} already exists", _0)]
    RootDirExisted(PathBuf),
    /// post path format error
    #[from(ignore)]
    #[display("post path {:?} format error: must be relative path without file extension", _0)]
    PostPathInvaild(PathBuf),
    /// post path already existed
    #[from(ignore)]
    #[display("post path {:?} already existed", _0)]
    PostPathExisted(PathBuf),
    /// theme template file encoding error
    #[from(ignore)]
    #[display("theme template file {:?} encoding error", _0)]
    ThemeFileEncoding(String),
    /// blog theme in use, can not be deleted error
    #[from(ignore)]
    #[display("blog theme {:?} in use, can not be deleted", _0)]
    ThemeInUse(String),
    /// blog theme not found error
    #[from(ignore)]
    #[display("blog theme {:?} not found", _0)]
    ThemeNotFound(String),
    /// post must has two parts error
    #[from(ignore)]
    #[display("post {:?} must has two parts: headers and body, splitted by first blank line", _0)]
    PostOnlyOnePart(PathBuf),
    /// post head part is empty error
    #[from(ignore)]
    #[display("post {:?} head part is empty", _0)]
    PostNoHead(PathBuf),
    /// post body part is empty error
    #[from(ignore)]
    #[display("post {:?} body part is empty", _0)]
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
            DateTimeFetchError(e) => Some(e),
            DateTimeFormat(e) => Some(e),
            Notify(e) => Some(e),
            GlobPattern(e) => Some(e),
            TomlExport(e) => Some(e),
            PathExpend(e) => Some(e),
            PostHeadPaser(e, _) => Some(e),
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
