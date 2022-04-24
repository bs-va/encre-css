use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

#[derive(Debug)]
pub struct WidthPlugin;

impl Plugin for WidthPlugin {
    fn namespace(&self) -> &str {
        "w"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended_size(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct MinWidthPlugin;

impl Plugin for MinWidthPlugin {
    fn namespace(&self) -> &str {
        "min-w"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("min-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "0" => Some(self.css_template_value("0")),
            "full" => Some(self.css_template_value("100%")),
            "min" => Some(self.css_template_value("min-content")),
            "max" => Some(self.css_template_value("max-content")),
            "fit" => Some(self.css_template_value("fit-content")),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct MaxWidthPlugin;

impl Plugin for MaxWidthPlugin {
    fn namespace(&self) -> &str {
        "max-w"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("max-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "0" => Some(self.css_template_value("0rem")),
            "none" => Some(self.css_template_value("none")),
            "xs" => Some(self.css_template_value("20rem")),
            "sm" => Some(self.css_template_value("24rem")),
            "md" => Some(self.css_template_value("28rem")),
            "lg" => Some(self.css_template_value("32rem")),
            "xl" => Some(self.css_template_value("36rem")),
            "2xl" => Some(self.css_template_value("42rem")),
            "3xl" => Some(self.css_template_value("48rem")),
            "4xl" => Some(self.css_template_value("56rem")),
            "5xl" => Some(self.css_template_value("64rem")),
            "6xl" => Some(self.css_template_value("72rem")),
            "7xl" => Some(self.css_template_value("80rem")),
            "full" => Some(self.css_template_value("100%")),
            "min" => Some(self.css_template_value("min-content")),
            "max" => Some(self.css_template_value("max-content")),
            "fit" => Some(self.css_template_value("fit-content")),
            "prose" => Some(self.css_template_value("65ch")),
            "screen-sm" => Some(self.css_template_value("640px")),
            "screen-md" => Some(self.css_template_value("768px")),
            "screen-lg" => Some(self.css_template_value("1024px")),
            "screen-xl" => Some(self.css_template_value("1280px")),
            "screen-2xl" => Some(self.css_template_value("1536px")),
            _ => None,
        }
    }
}

// Height

#[derive(Debug)]
pub struct HeightPlugin;

impl Plugin for HeightPlugin {
    fn namespace(&self) -> &str {
        "h"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("height: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended_size(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct MinHeightPlugin;

impl Plugin for MinHeightPlugin {
    fn namespace(&self) -> &str {
        "min-h"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("min-height: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "0" => Some(self.css_template_value("0")),
            "full" => Some(self.css_template_value("100%")),
            "min" => Some(self.css_template_value("min-content")),
            "max" => Some(self.css_template_value("max-content")),
            "fit" => Some(self.css_template_value("fit-content")),
            "screen" => Some(self.css_template_value("100vh")),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct MaxHeightPlugin;

impl Plugin for MaxHeightPlugin {
    fn namespace(&self) -> &str {
        "max-h"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("max-height: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "0" => Some(self.css_template_value("0rem")),
            "none" => Some(self.css_template_value("none")),
            "xs" => Some(self.css_template_value("20rem")),
            "sm" => Some(self.css_template_value("24rem")),
            "md" => Some(self.css_template_value("28rem")),
            "lg" => Some(self.css_template_value("32rem")),
            "xl" => Some(self.css_template_value("36rem")),
            "2xl" => Some(self.css_template_value("42rem")),
            "3xl" => Some(self.css_template_value("48rem")),
            "4xl" => Some(self.css_template_value("56rem")),
            "5xl" => Some(self.css_template_value("64rem")),
            "6xl" => Some(self.css_template_value("72rem")),
            "7xl" => Some(self.css_template_value("80rem")),
            "full" => Some(self.css_template_value("100%")),
            "min" => Some(self.css_template_value("min-content")),
            "max" => Some(self.css_template_value("max-content")),
            "screen" => Some(self.css_template_value("100vh")),
            "fit" => Some(self.css_template_value("fit-content")),
            _ => None,
        }
    }
}
