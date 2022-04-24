use super::Plugin;

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct BorderCollapsePlugin;

impl Plugin for BorderCollapsePlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "collapse" => write!(css_content, "border-collapse: collapse;"),
            "separate" => write!(css_content, "border-collapse: separate;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TableLayoutPlugin;

impl Plugin for TableLayoutPlugin {
    fn namespace(&self) -> &str {
        "table"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "auto" => write!(css_content, "table-layout: auto;"),
            "fixed" => write!(css_content, "table-layout: fixed;"),
            _ => Ok(()),
        }
    }
}
