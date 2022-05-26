use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::Write;

#[derive(Debug)]
pub struct WidthPlugin;

impl Plugin for WidthPlugin {
    fn namespace(&self) -> &str {
        "w"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "width: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("screen") {
            return self.css_template_value("100vw", css_content);
        }

        if let Some(length) =
            default_lengths::get_extended_size(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "min-width: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "0" => self.css_template_value("0", css_content),
            "full" => self.css_template_value("100%", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "fit" => self.css_template_value("fit-content", css_content),
            _ => false,
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "max-width: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "0" => self.css_template_value("0rem", css_content),
            "none" => self.css_template_value("none", css_content),
            "xs" => self.css_template_value("20rem", css_content),
            "sm" => self.css_template_value("24rem", css_content),
            "md" => self.css_template_value("28rem", css_content),
            "lg" => self.css_template_value("32rem", css_content),
            "xl" => self.css_template_value("36rem", css_content),
            "2xl" => self.css_template_value("42rem", css_content),
            "3xl" => self.css_template_value("48rem", css_content),
            "4xl" => self.css_template_value("56rem", css_content),
            "5xl" => self.css_template_value("64rem", css_content),
            "6xl" => self.css_template_value("72rem", css_content),
            "7xl" => self.css_template_value("80rem", css_content),
            "full" => self.css_template_value("100%", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "fit" => self.css_template_value("fit-content", css_content),
            "prose" => self.css_template_value("65ch", css_content),
            "screen-sm" => self.css_template_value("640px", css_content),
            "screen-md" => self.css_template_value("768px", css_content),
            "screen-lg" => self.css_template_value("1024px", css_content),
            "screen-xl" => self.css_template_value("1280px", css_content),
            "screen-2xl" => self.css_template_value("1536px", css_content),
            _ => false,
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "height: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("screen") {
            return self.css_template_value("100vh", css_content);
        }

        if let Some(length) =
            default_lengths::get_extended_size(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "min-height: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "0" => self.css_template_value("0", css_content),
            "full" => self.css_template_value("100%", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "fit" => self.css_template_value("fit-content", css_content),
            "screen" => self.css_template_value("100vh", css_content),
            _ => false,
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "max-height: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "0" => self.css_template_value("0rem", css_content),
            "none" => self.css_template_value("none", css_content),
            "xs" => self.css_template_value("20rem", css_content),
            "sm" => self.css_template_value("24rem", css_content),
            "md" => self.css_template_value("28rem", css_content),
            "lg" => self.css_template_value("32rem", css_content),
            "xl" => self.css_template_value("36rem", css_content),
            "2xl" => self.css_template_value("42rem", css_content),
            "3xl" => self.css_template_value("48rem", css_content),
            "4xl" => self.css_template_value("56rem", css_content),
            "5xl" => self.css_template_value("64rem", css_content),
            "6xl" => self.css_template_value("72rem", css_content),
            "7xl" => self.css_template_value("80rem", css_content),
            "full" => self.css_template_value("100%", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "screen" => self.css_template_value("100vh", css_content),
            "fit" => self.css_template_value("fit-content", css_content),
            _ => false,
        }
    }
}
