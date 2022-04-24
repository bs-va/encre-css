use super::Plugin;
use crate::utils::value_matchers::*;

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn namespace(&self) -> &str {
        "order"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_number(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "order: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "first" => return self.css_template_value("-9999", css_content),
            "last" => return self.css_template_value("9999", css_content),
            "none" => return self.css_template_value("0", css_content),
            _ => (),
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<f32>().is_ok() {
            self.css_template_value(modifier, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "row" => write!(css_content, "flex-direction: row;"),
            "row-reverse" => write!(css_content, "flex-direction: row-reverse;"),
            "col" => write!(css_content, "flex-direction: column;"),
            "col-reverse" => write!(css_content, "flex-direction: column-reverse;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "nowrap" => write!(css_content, "flex-wrap: nowrap;"),
            "wrap" => write!(css_content, "flex-wrap: wrap;"),
            "wrap-reverse" => write!(css_content, "flex-wrap: wrap-reverse;"),
            _ => Ok(()),
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
            if is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val) {
                is_matching.2 = true;
            }
        }

        is_matching.0 && is_matching.1 && is_matching.2
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "flex: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "flex-1" => self.css_template_value("1 1 0%", css_content),
            "flex-auto" => self.css_template_value("1 1 auto", css_content),
            "flex-initial" => self.css_template_value("0 1 auto", css_content),
            "flex-none" => self.css_template_value("none", css_content),
            "flex-grow" => write!(css_content, "flex-grow: 1;"),
            "flex-grow-0" => write!(css_content, "flex-grow: 0;"),
            "flex-shrink" => write!(css_content, "flex-shrink: 1;"),
            "flex-shrink-0" => write!(css_content, "flex-shrink: 0;"),
            _ => Ok(()),
        }
    }
}
