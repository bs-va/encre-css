use super::Plugin;
use crate::utils::value_matchers::*;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PROPERTY_REGEX: Regex = Regex::new(r"[^\d]+").unwrap();
}

#[derive(Debug)]
pub struct TransitionPropertyPlugin;

impl Plugin for TransitionPropertyPlugin {
    fn namespace(&self) -> String {
        "transition".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        PROPERTY_REGEX.is_match(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("transition-property: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "" => Some("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;".to_string()),
            "none" => Some("transition-property: none;".to_string()),
            "all" => Some("transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;".to_string()),
            "colors" => Some("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
  transition-timing-function: cubic-bezier(0.4, 0, 0);
  transition-duration: 150ms;".to_string()),
            "opacity" => Some("transition-property: opacity;
  transition-timing-function: cubic-bezier(0.4, 0, 0);
  transition-duration: 150ms;".to_string()),
            "shadow" => Some("transition-property: box-shadow;
  transition-timing-function: cubic-bezier(0.4, 0, 0);
  transition-duration: 150ms;".to_string()),
            "transform" => Some("transition-property: transform;
  transition-timing-function: cubic-bezier(0.4, 0, 0);
  transition-duration: 150ms;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct TransitionDurationPlugin;

impl Plugin for TransitionDurationPlugin {
    fn namespace(&self) -> String {
        "duration".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_time(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("transition-duration: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "75" => Some(self.css_template_value("75ms")),
            "100" => Some(self.css_template_value("100ms")),
            "150" => Some(self.css_template_value("150ms")),
            "200" => Some(self.css_template_value("200ms")),
            "300" => Some(self.css_template_value("300ms")),
            "500" => Some(self.css_template_value("500ms")),
            "700" => Some(self.css_template_value("700ms")),
            "1000" => Some(self.css_template_value("1000ms")),
            _ => None,
        }
    }
}

/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register("delay-75", "transition-delay: 75ms;".to_string());
    selectors.register("delay-100", "transition-delay: 100ms;".to_string());
    selectors.register("delay-150", "transition-delay: 150ms;".to_string());
    selectors.register("delay-200", "transition-delay: 200ms;".to_string());
    selectors.register("delay-300", "transition-delay: 300ms;".to_string());
    selectors.register("delay-500", "transition-delay: 500ms;".to_string());
    selectors.register("delay-700", "transition-delay: 700ms;".to_string());
    selectors.register("delay-1000", "transition-delay: 1000ms;".to_string());
    selectors.register(
        "ease-linear",
        "transition-timing-function: linear;".to_string(),
    );
    selectors.register(
        "ease-in",
        "transition-timing-function: cubic-bezier(0.4, 0, 1, 1);".to_string(),
    );
    selectors.register(
        "ease-out",
        "transition-timing-function: cubic-bezier(0, 0, 0.2, 1);".to_string(),
    );
    selectors.register(
        "ease-in-out",
        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);".to_string(),
    );
    selectors.register("animate-none", "animation: none;".to_string());
    selectors.register("animate-spin", "".to_string()); // TODO
    selectors.register("animate-ping", "".to_string()); // TODO
    selectors.register("animate-pulse", "".to_string()); // TODO
    selectors.register("animate-bounce", "".to_string()); // TODO
}*/
