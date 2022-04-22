use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

#[derive(Debug)]
pub struct SizingWidthPlugin;

impl Plugin for SizingWidthPlugin {
    fn namespace(&self) -> String {
        "w".to_string()
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
pub struct SizingMinWidthPlugin;

impl Plugin for SizingMinWidthPlugin {
    fn namespace(&self) -> String {
        "min-w".to_string()
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
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct SizingMaxWidthPlugin;

impl Plugin for SizingMaxWidthPlugin {
    fn namespace(&self) -> String {
        "max-w".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        println!("{:#?}", val);
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
pub struct SizingHeightPlugin;

impl Plugin for SizingHeightPlugin {
    fn namespace(&self) -> String {
        "h".to_string()
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
pub struct SizingMinHeightPlugin;

impl Plugin for SizingMinHeightPlugin {
    fn namespace(&self) -> String {
        "min-h".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("height: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "min-h-0" => Some(self.css_template_value("min-height: 0;")),
            "min-h-full" => Some(self.css_template_value("min-height: 100%;")),
            "min-h-min" => Some(self.css_template_value("min-height: min-content;")),
            "min-h-max" => Some(self.css_template_value("min-height: max-content;")),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct SizingMaxHeightPlugin;

impl Plugin for SizingMaxHeightPlugin {
    fn namespace(&self) -> String {
        "max-h".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("height: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "max-h-0" => Some(self.css_template_value("max-height: 0rem;")),
            "max-h-none" => Some(self.css_template_value("max-height: none;")),
            "max-h-xs" => Some(self.css_template_value("max-height: 20rem;")),
            "max-h-sm" => Some(self.css_template_value("max-height: 24rem;")),
            "max-h-md" => Some(self.css_template_value("max-height: 28rem;")),
            "max-h-lg" => Some(self.css_template_value("max-height: 32rem;")),
            "max-h-xl" => Some(self.css_template_value("max-height: 36rem;")),
            "max-h-2xl" => Some(self.css_template_value("max-height: 42rem;")),
            "max-h-3xl" => Some(self.css_template_value("max-height: 48rem;")),
            "max-h-4xl" => Some(self.css_template_value("max-height: 56rem;")),
            "max-h-5xl" => Some(self.css_template_value("max-height: 64rem;")),
            "max-h-6xl" => Some(self.css_template_value("max-height: 72rem;")),
            "max-h-7xl" => Some(self.css_template_value("max-height: 80rem;")),
            "max-h-full" => Some(self.css_template_value("max-height: 100%;")),
            "max-h-min" => Some(self.css_template_value("max-height: min-content;")),
            "max-h-max" => Some(self.css_template_value("max-height: max-content;")),
            "max-h-screen" => Some(self.css_template_value("max-height: 100vh;")),
            _ => None,
        }
    }
}
