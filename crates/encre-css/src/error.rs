//! Define a custom [`Error`] type.
use std::{ops::Range, path::PathBuf};
use thiserror::Error as ErrorTrait;

/// Shorthand for [`Result`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

/// The custom error type used everywhere in this crate
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
}

/// The kind of parsing error
#[derive(ErrorTrait, Debug, PartialEq)]
pub enum ParseErrorKind<'a> {
    /// Indicate that the selector is too short to be an existing selector (the shortest selector has 2 characters).
    #[error("the selector `{0}` is too short to be an existing selector")]
    TooShort(&'a str),

    /// Indicate that the selector has some variants but no modifier
    #[error("the selector `{0}` has some variants but no modifier")]
    VariantsWithoutModifier(&'a str),

    /// Indicate that no plugins were found to handle the selector
    #[error("no plugins found to handle the selector `{0}`")]
    UnknownPlugin(&'a str),

    /// Indicate that a variant of the selector does not exist
    #[error("the variant `{0}` of the selector `{1}` does not exist")]
    UnknownVariant(&'a str, &'a str),
}

/// The custom error type used to indicate that an error happened when parsing selectors.
///
/// By default, this kind of error is ignored but if you want to check that a selector is valid,
/// you can use [`utils::check_selectors`].
///
/// [`utils::check_selectors`]: crate::utils::check_selectors
#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    /// The location of the error in the provided content
    pub span: Range<usize>,

    /// The kind of parsing error
    pub kind: ParseErrorKind<'a>,
}

impl<'a> ParseError<'a> {
    pub(crate) fn new(span: Range<usize>, kind: ParseErrorKind<'a>) -> Self {
        Self { span, kind }
    }
}
