#![no_main]

use encre_css::Config;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let config = Config::default();
    encre_css::generate([data], &config);
});
