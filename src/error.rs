use std::error::Error as StdError;
use std::fmt;
use std::path::{Path, PathBuf};

/// Result type
pub type Result<T> = std::result::Result<T, Error>;

/// The Error type
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    source: Option<Source>,
}

type Source = Box<dyn StdError + Send + Sync>;

/// The kind of error
#[derive(Debug)]
pub enum ErrorKind {
    Io,
    PathStripPrefix,
    ConfigMerge,
    Template,
    Notify,
    GlobPattern,
    TomlExport,
    PathExpend,
    PostHeadPaser,

    RootDirExisted(PathBuf),
    PostPathInvaild(PathBuf),
    PostPathExisted(PathBuf),
    ThemeFileEncoding(String),
    ThemeInUse(String),
    ThemeNotFound(String),
    PostOnlyOnePart(PathBuf),
    PostNoHead(PathBuf),
    PostNoBody(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "IO error"),
            ErrorKind::PathStripPrefix => write!(f, "Path strip prefix error"),
            ErrorKind::ConfigMerge => write!(f, "Config merge error"),
            ErrorKind::Template => write!(f, "Template error"),
            ErrorKind::Notify => write!(f, "Notify error"),
            ErrorKind::GlobPattern => write!(f, "Glob pattern error"),
            ErrorKind::TomlExport => write!(f, "Toml export error"),
            ErrorKind::PathExpend => write!(f, "Path expand error"),
            ErrorKind::PostHeadPaser => write!(f, "Post head parse error, please use yaml grammar"),

            ErrorKind::RootDirExisted(ref path) => write!(f, "blog root directory {:?} already exists", path),
            ErrorKind::PostPathInvaild(ref path) => write!(
                f,
                "post path {:?} format error: must be relative path without file extension",
                path
            ),
            ErrorKind::PostPathExisted(ref path) => write!(f, "post path {:?} already existed", path),
            ErrorKind::ThemeFileEncoding(ref file) => write!(f, "Theme template file {:?} encoding error", file),
            ErrorKind::ThemeInUse(ref theme) => write!(f, "blog theme {} in use, can not be deleted", theme),
            ErrorKind::ThemeNotFound(ref theme) => write!(f, "blog theme {} not found", theme),
            ErrorKind::PostOnlyOnePart(ref path) => write!(f, "post {:?} must has two parts: headers and body, splitted by first blank line", path),
            ErrorKind::PostNoHead(ref path) => write!(f, "post {:?} head part is empty", path),
            ErrorKind::PostNoBody(ref path) => write!(f, "post {:?} body part is empty", path),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|c| &**c as &(dyn StdError + 'static))
    }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind, source: None }
    }

    pub(crate) fn with(mut self, source: impl Into<Source>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub(crate) fn root_dir_existed(path: &Path) -> Self {
        Self::new(ErrorKind::RootDirExisted(path.into()))
    }

    pub(crate) fn post_path_invalid(path: &Path) -> Self {
        Self::new(ErrorKind::PostPathInvaild(path.into()))
    }

    pub(crate) fn post_path_existed(path: &Path) -> Self {
        Self::new(ErrorKind::PostPathExisted(path.into()))
    }

    pub(crate) fn theme_file_encoding(file: &str) -> Self {
        Self::new(ErrorKind::ThemeFileEncoding(file.into()))
    }

    pub(crate) fn theme_in_use(theme: &str) -> Self {
        Self::new(ErrorKind::ThemeInUse(theme.into()))
    }

    pub(crate) fn theme_not_found(theme: &str) -> Self {
        Self::new(ErrorKind::ThemeNotFound(theme.into()))
    }
    
    pub(crate) fn post_only_one_part(path: &Path) -> Self {
        Self::new(ErrorKind::PostOnlyOnePart(path.into()))
    }

    pub(crate) fn post_no_head(path: &Path) -> Self {
        Self::new(ErrorKind::PostNoHead(path.into()))
    }

    pub(crate) fn post_empty_body(path: &Path) -> Self {
        Self::new(ErrorKind::PostNoBody(path.into()))
    }

}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Self::new(ErrorKind::Io).with(e)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(e: std::path::StripPrefixError) -> Self {
        Self::new(ErrorKind::PathStripPrefix).with(e)
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Self::new(ErrorKind::ConfigMerge).with(e)
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self::new(ErrorKind::Template).with(e)
    }
}

impl From<notify::Error> for Error {
    fn from(e: notify::Error) -> Self {
        Self::new(ErrorKind::Notify).with(e)
    }
}

impl From<glob::PatternError> for Error {
    fn from(e: glob::PatternError) -> Self {
        Self::new(ErrorKind::GlobPattern).with(e)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Self::new(ErrorKind::TomlExport).with(e)
    }
}

impl From<shellexpand::LookupError<std::env::VarError>> for Error {
    fn from(e: shellexpand::LookupError<std::env::VarError>) -> Self {
        Self::new(ErrorKind::PathExpend).with(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self::new(ErrorKind::PostHeadPaser).with(e)
    }
}
