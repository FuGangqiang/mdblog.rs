use std::error::Error as StdError;
use std::fmt::{self, Error as FmtError};
use std::io::Error as IoError;

use tera::TeraError;
use toml;

use self::Error::{
    RootDirExisted,
    ThemeNotFound,
    PostHead,
    PostBody,
    Render,
    Io,
    Toml,
    Fmt,
};

pub type Result<T> = ::std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    RootDirExisted,
    ThemeNotFound,
    PostHead,
    PostBody,
    Render(TeraError),
    Io(IoError),
    Toml(Vec<toml::ParserError>),
    Fmt(FmtError),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Render(ref err) => err.fmt(f),
            Io(ref err) => err.fmt(f),
            Toml(ref errs) => {
                for err in errs {
                    f.write_str(err.description())?;
                }
                Ok(())
            }
            Fmt(ref err) => err.fmt(f),
            ref err => f.write_str(err.description()),
        }
    }
}


impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            RootDirExisted => "blog root directory already exists",
            ThemeNotFound => "theme not found",
            PostHead => "post head part parse error",
            PostBody => "post must have body part",
            Render(ref err) => err.description(),
            Io(ref err) => err.description(),
            Toml(ref errs) => errs[0].description(),
            Fmt(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Render(ref err) => Some(err),
            Io(ref err) => Some(err),
            Toml(ref errs) => errs[0].cause(),
            Fmt(ref err) => Some(err),
            _ => None,
        }
    }
}


impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}


impl From<TeraError> for Error {
    fn from(err: TeraError) -> Error {
        Error::Render(err)
    }
}


impl From<FmtError> for Error {
    fn from(err: FmtError) -> Error {
        Error::Fmt(err)
    }
}


impl From<Vec<toml::ParserError>> for Error {
    fn from(errs: Vec<toml::ParserError>) -> Error {
        Error::Toml(errs)
    }
}
