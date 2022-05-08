use std::path::PathBuf;
use thiserror::Error as ErrorTrait;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ErrorTrait, Debug)]
pub enum Error {
    #[error("configuration file `{0}` not found ({1})")]
    ConfigFileNotFound(PathBuf, std::io::Error),

    #[error("bad configuration file: `{}`", .0)]
    ConfigParsing(#[from] toml::de::Error),
}
