use crate::{config::Config, generator::EncreGenerator, Preflight};

pub(crate) fn base_config() -> Config {
    // Disable the preflight to simplify test assertions
    let mut config = Config::default();
    config.preflight = Preflight::None;
    config
}

pub(crate) fn generate_css(selector: &str) -> String {
    let config = base_config();
    let mut generator = EncreGenerator::new(&config);

    generator.add_selector(selector);
    generator.generate()
}
