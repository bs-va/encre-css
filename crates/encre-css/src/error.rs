use std::{path::PathBuf, num::ParseIntError, fmt};
use serde::{de, ser};
use thiserror::Error as ErrorTrait;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ErrorTrait, Debug)]
pub enum Error {
    #[error("configuration file `{0}` not found ({1})")]
    ConfigFileNotFound(PathBuf, std::io::Error),

    #[error("bad configuration file: `{0}`")]
    ConfigParsing(#[from] toml::de::Error),

    #[error("error when converting the hexadecimal color `{0}` to rgb: {1:?}")]
    HexToRgbConversion(String, ParseIntError),

    #[error("error when deserializing: {0:?}")]
    Deserialize(String),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Deserialize(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Deserialize(msg.to_string())
    }
}