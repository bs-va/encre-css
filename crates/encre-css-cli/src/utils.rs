use std::clone::Clone;
use std::path::Path;

use color_eyre::Result;
use encre_css::Config;
use toml::to_string_pretty;

pub fn generate_config() -> Result<()> {
    let config = Config::default();
    let config_str = to_string_pretty(&config)?;
    println!("{config_str}");
    Ok(())
}

pub fn has_file_match<'a, T, U>(watched_paths: T, reported_paths: &'a U) -> bool
where
    T: Iterator<Item = &'a Path>,
    U: Iterator<Item = &'a &'a Path> + Clone,
{
    for w in watched_paths {
        if reported_paths.clone().any(|p| *p == w) {
            return true;
        }
    }
    false
}
