use crate::error::{Result, Error};

use serde_derive::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DarkModeConfig {
    Class(String),
    Media,
}

impl Default for DarkModeConfig {
    fn default() -> Self {
        Self::Media
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct ThemeConfig {
    pub dark_mode: DarkModeConfig,
}

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub input: Vec<PathBuf>,

    #[serde(default)]
    pub theme: ThemeConfig,

    // custom_variants: Vec<VariantConfig>,
    // custom_plugins: Vec<PluginConfig>,

    // TODO: Prefix (en-), preflight, safelist, separator for {variants, arbitrary values, modifiers}
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        toml::from_str(&fs::read_to_string(&path).map_err(|e| Error::ConfigFileNotFound(path, e))?).map_err(|e| e.into())
    }
}
