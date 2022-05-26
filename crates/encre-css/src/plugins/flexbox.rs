use super::Plugin;
use crate::utils::value_matchers::*;
use crate::{config::Config, selector::Modifier};

use std::fmt::Write;

#[derive(Debug)]
pub struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn namespace(&self) -> &str {
        "order"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_number(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "order: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "first" => return self.css_template_value("-9999", css_content),
            "last" => return self.css_template_value("9999", css_content),
            "none" => return self.css_template_value("0", css_content),
            _ => (),
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.to_f32().is_ok() {
            self.css_template_value(&modifier.to_string(), css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "row" => write!(css_content, "flex-direction: row;").is_ok(),
            "row-reverse" => write!(css_content, "flex-direction: row-reverse;").is_ok(),
            "col" => write!(css_content, "flex-direction: column;").is_ok(),
            "col-reverse" => write!(css_content, "flex-direction: column-reverse;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "nowrap" => write!(css_content, "flex-wrap: nowrap;").is_ok(),
            "wrap" => write!(css_content, "flex-wrap: wrap;").is_ok(),
            "wrap-reverse" => write!(css_content, "flex-wrap: wrap-reverse;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct GrowShrinkBasisPlugin;

impl Plugin for GrowShrinkBasisPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        if hint == "list" {
            return true;
        }

        let mut split = val.split('_');
        let mut is_matching = (false, false, false);

        // flex-grow
        // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow#values
        if let Some(val) = split.next() {
            if is_matching_number(val) {
                is_matching.0 = true;
            }
        }

        // flex-shrink
        // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink#values
        if let Some(val) = split.next() {
            if is_matching_number(val) {
                is_matching.1 = true;
            }
        }

        // flex-basis
        // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis#values
        if let Some(val) = split.next() {
            if is_matching_length(val) || is_matching_percentage(val) || val == "auto" {
                is_matching.2 = true;
            }
        }

        is_matching.0 && is_matching.1 && is_matching.2
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "flex: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "1" => self.css_template_value("1 1 0%", css_content),
            "auto" => self.css_template_value("1 1 auto", css_content),
            "initial" => self.css_template_value("0 1 auto", css_content),
            "none" => self.css_template_value("none", css_content),
            "grow" => write!(css_content, "flex-grow: 1;").is_ok(),
            "grow-0" => write!(css_content, "flex-grow: 0;").is_ok(),
            "shrink" => write!(css_content, "flex-shrink: 1;").is_ok(),
            "shrink-0" => write!(css_content, "flex-shrink: 0;").is_ok(),
            _ => false,
        }
    }
}
