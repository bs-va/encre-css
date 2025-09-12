use color_eyre::Result;
use encre_css::Config;
use toml::to_string_pretty;

pub fn generate_config() -> Result<()> {
    let config = Config::default();
    let config_str = to_string_pretty(&config)?;
    println!("{config_str}");
    Ok(())
}
