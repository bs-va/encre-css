use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

use std::fmt::Write;

#[derive(Debug)]
pub struct FillPlugin;

impl Plugin for FillPlugin {
    fn namespace(&self) -> &str {
        "fill"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "fill: {};",
                val.replace("/ var(--tw-opacity)", "")
            )
            .is_ok()
        } else {
            write!(css_content, "fill: {val};").is_ok()
        }
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(color) = default_colors::get(modifier) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct StrokeColorPlugin;

impl Plugin for StrokeColorPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "stroke: {};",
                val.replace("/ var(--tw-opacity)", "")
            )
            .is_ok()
        } else {
            write!(css_content, "stroke: {val};").is_ok()
        }
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(color) = default_colors::get(modifier) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct StrokeWidthPlugin;

impl Plugin for StrokeWidthPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val) || is_matching_percentage(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "stroke-width: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<f32>().is_ok() {
            self.css_template_value(&format!("{}px", modifier), css_content)
        } else {
            false
        }
    }
}
