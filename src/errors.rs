use std::path::PathBuf;
use std::io::Error as IoError;
use config::ConfigError;
use tera::Error as TeraError;

/// The error type used by this crate.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO error")]
    Io(#[cause] ::std::io::Error),

    #[fail(display = "Config error")]
    Config(#[cause] ConfigError),

     #[fail(display = "Template error: {}", _0)]
     Template(String),
     // Template(#[cause] ::tera::Error),

    #[fail(display = "Fmt error")]
    Fmt(#[cause] ::std::fmt::Error),

    #[fail(display = "blog root directory `{:?}` already exists", _0)]
    RootDirExisted(PathBuf),

    #[fail(display = "blog theme `{}` not found", _0)]
    ThemeNotFound(String),

    #[fail(display = "post `{:?}` head part format error", _0)]
    PostHead(PathBuf),

    #[fail(display = "post `{:?}` has not body part", _0)]
    PostNoBody(PathBuf),
}

impl From<IoError> for Error {
     fn from(err: IoError) -> Error {
         Error::Io(err)
     }
}

impl From<ConfigError> for Error {
     fn from(err: ConfigError) -> Error {
         Error::Config(err)
     }
}

impl From<TeraError> for Error {
     fn from(err: TeraError) -> Error {
         Error::Template(err.description().to_string())
     }
}

/// A specialized `Result` type where the error is hard-wired to [`Error`].
///
/// [`Error`]: enum.Error.html
pub type Result<T> = ::std::result::Result<T, Error>;
