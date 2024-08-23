#![no_main]

use encre_css::Config;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let config = Config::default();

        encre_css::generate([s], &config);
    }
});
