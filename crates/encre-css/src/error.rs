//! Define a custom [`Error`] type.
use std::{fmt, path::PathBuf};
use thiserror::Error as ErrorTrait;

/// Shorthand for [`Result`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

/// The custom error type used everywhere in this crate and returned by [`EncreGenerator::generate`].
///
/// [`EncreGenerator::generate`]: crate::EncreGenerator::generate
#[derive(ErrorTrait, Debug)]
pub enum Error {
    /// Indicate that the configuration file is not found.
    #[error("configuration file `{0}` not found ({1})")]
    ConfigFileNotFound(PathBuf, std::io::Error),

    /// Indicate that an error happened when parsing a [TOML](https://toml.io) file.
    #[error("bad configuration file: `{0}`")]
    ConfigParsing(#[from] toml::de::Error),

    /// Indicate that an error happened when converting an hexadecimal color to an RGB one.
    #[error("error when converting the hexadecimal color `{0}` to rgb: {1:?}")]
    HexToRgbConversion(String, String),

    /// Indicate that an error happened when writing to a buffer.
    #[error("error when formatting: {0:?}")]
    Format(#[from] fmt::Error),
}
