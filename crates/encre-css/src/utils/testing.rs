use crate::{config::Config, Preflight};

pub(crate) fn base_config() -> Config {
    // Disable the preflight to simplify test assertions
    Config {
        preflight: Preflight::None,
        ..Default::default()
    }
}
