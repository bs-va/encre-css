use super::Plugin;
use crate::{config::Config, selector::Modifier};

use std::fmt::Write;

#[derive(Debug)]
pub struct ScreenReaderPlugin;

impl Plugin for ScreenReaderPlugin {
    fn namespace(&self) -> &str {
        ""
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "sr-only" => write!(
                css_content,
                "position: absolute;
width: 1px;
height: 1px;
padding: 0;
margin: -1px;
overflow: hidden;
clip: rect(0, 0, 0, 0);
white-space: nowrap;
border-width: 0;"
            )
            .is_ok(),
            "not-sr-only" => write!(
                css_content,
                "position: static;
width: auto;
height: auto;
padding: 0;
margin: 0;
overflow: visible;
clip: auto;
white-space: normal;"
            )
            .is_ok(),
            _ => false,
        }
    }
}
