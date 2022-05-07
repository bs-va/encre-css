use super::Plugin;
use crate::selector::Modifier;

use std::fmt::Write;

#[derive(Debug)]
pub struct BorderCollapsePlugin;

impl Plugin for BorderCollapsePlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "collapse" => write!(css_content, "border-collapse: collapse;").is_ok(),
            "separate" => write!(css_content, "border-collapse: separate;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct TableLayoutPlugin;

impl Plugin for TableLayoutPlugin {
    fn namespace(&self) -> &str {
        "table"
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "auto" => write!(css_content, "table-layout: auto;").is_ok(),
            "fixed" => write!(css_content, "table-layout: fixed;").is_ok(),
            _ => false,
        }
    }
}
