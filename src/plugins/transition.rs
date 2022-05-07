use super::Plugin;
use crate::utils::value_matchers::*;

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Write;

lazy_static! {
    static ref PROPERTY_REGEX: Regex = Regex::new(r"[^\d]+").unwrap();
}

#[derive(Debug)]
pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn namespace(&self) -> &str {
        "transition"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        PROPERTY_REGEX.is_match(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-property: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        match modifier {
            "" => write!(css_content, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
transition-duration: 150ms;").is_ok(),
            "none" => write!(css_content, "transition-property: none;").is_ok(),
            "all" => write!(css_content, "transition-property: all;
transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
transition-duration: 150ms;").is_ok(),
            "colors" => write!(css_content, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            "opacity" => write!(css_content, "transition-property: opacity;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            "shadow" => write!(css_content, "transition-property: box-shadow;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            "transform" => write!(css_content, "transition-property: transform;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct DurationPlugin;

impl Plugin for DurationPlugin {
    fn namespace(&self) -> &str {
        "duration"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_time(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-duration: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(duration) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{duration}ms"), css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct DelayPlugin;

impl Plugin for DelayPlugin {
    fn namespace(&self) -> &str {
        "delay"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_time(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-delay: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(delay) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{delay}ms"), css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct EasePlugin;

impl Plugin for EasePlugin {
    fn namespace(&self) -> &str {
        "ease"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-timing-function: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        match modifier {
            "linear" => self.css_template_value("linear", css_content),
            "in" => self.css_template_value("cubic-bezier(0.4, 0, 1, 1)", css_content),
            "out" => self.css_template_value("cubic-bezier(0, 0, 0.2, 1)", css_content),
            "in-out" => self.css_template_value("cubic-bezier(0.4, 0, 0.2, 1)", css_content),
            _ => false,
        }
    }
}

/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register("animate-none", "animation: none;".to_string());
    selectors.register("animate-spin", "".to_string()); // TODO
    selectors.register("animate-ping", "".to_string()); // TODO
    selectors.register("animate-pulse", "".to_string()); // TODO
    selectors.register("animate-bounce", "".to_string()); // TODO
}*/
