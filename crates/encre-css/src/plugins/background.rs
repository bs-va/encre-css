use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use lazy_static::lazy_static;
use regex::Regex;
use std::{borrow::Cow, fmt::Write};

lazy_static! {
    static ref OPACITY_IN_RGB_REGEX: Regex = Regex::new(r"/.*\)").unwrap();
}

#[derive(Debug)]
pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        let property = if val.contains("url") {
            "background-image"
        } else {
            "background-color"
        };
        
        if val.contains("--en-opacity") {
            write!(
                css_content,
                "--en-bg-opacity: 1;
{property}: {};",
                val.replace("--en-opacity", "--en-bg-opacity")
            )
            .is_ok()
        } else {
            write!(css_content, "{property}: {val};").is_ok()
        }
    }

    fn get_css_for_modifier(
        &self,
        config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(color) = default_colors::get(config, modifier.content()) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct AttachmentPlugin;

impl Plugin for AttachmentPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "fixed" => write!(css_content, "background-attachment: fixed;").is_ok(),
            "local" => write!(css_content, "background-attachment: local;").is_ok(),
            "scroll" => write!(css_content, "background-attachment: scroll;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ClipPlugin;

impl Plugin for ClipPlugin {
    fn namespace(&self) -> &str {
        "bg-clip"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "border" => write!(css_content, "background-clip: border-box;").is_ok(),
            "padding" => write!(css_content, "background-clip: padding-box;").is_ok(),
            "content" => write!(css_content, "background-clip: content-box;").is_ok(),
            "text" => write!(css_content, "background-clip: text;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "bg-opacity"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.to_f32() {
            write!(css_content, "--en-bg-opacity: {};", opacity_value / 100.).is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        // FIXME: Is this really list? (probably because of comma)
        hint == "list" || is_matching_image(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "background-image: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "bg-none" => self.css_template_value("none", css_content),
            "gradient-to-t" => self.css_template_value(
                "linear-gradient(to top, var(--en-gradient-stops))",
                css_content,
            ),
            "gradient-to-tr" => self.css_template_value(
                "linear-gradient(to top right, var(--en-gradient-stops))",
                css_content,
            ),
            "gradient-to-r" => self.css_template_value(
                "linear-gradient(to right, var(--en-gradient-stops))",
                css_content,
            ),
            "gradient-to-br" => self.css_template_value(
                "linear-gradient(to bottom right, var(--en-gradient-stops))",
                css_content,
            ),
            "gradient-to-b" => self.css_template_value(
                "linear-gradient(to bottom, var(--en-gradient-stops))",
                css_content,
            ),
            "gradient-to-bl" => self.css_template_value(
                "linear-gradient(to bottom left, var(--en-gradient-stops))",
                css_content,
            ),
            "gradient-to-l" => self.css_template_value(
                "linear-gradient(to left, var(--en-gradient-stops))",
                css_content,
            ),
            "gradient-to-tl" => self.css_template_value(
                "linear-gradient(to top left, var(--en-gradient-stops))",
                css_content,
            ),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct GradientFromPlugin;

impl Plugin for GradientFromPlugin {
    fn namespace(&self) -> &str {
        "from"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        let default_to = if val == "inherit" || val == "currentColor" {
            Cow::Borrowed("rgb(255 255 255 / 0)")
        } else {
            OPACITY_IN_RGB_REGEX.replace(val, "/ 0)")
        };

        let val = if val.contains("--en-opacity") {
            val.replace(" / var(--en-opacity)", "")
        } else {
            val.to_string()
        };

        write!(
            css_content,
            "--en-gradient-from: {val};
--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, {default_to});"
        )
        .is_ok()
    }

    fn get_css_for_modifier(
        &self,
        config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(color) = default_colors::get(config, modifier.content()) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct GradientViaPlugin;

impl Plugin for GradientViaPlugin {
    fn namespace(&self) -> &str {
        "via"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        let default_to = if val == "inherit" || val == "currentColor" {
            Cow::Borrowed("rgb(255 255 255 / 0)")
        } else {
            OPACITY_IN_RGB_REGEX.replace(val, "/ 0)")
        };

        let val = if val.contains("--en-opacity") {
            val.replace(" / var(--en-opacity)", "")
        } else {
            val.to_string()
        };

        write!(
            css_content,
            "--en-gradient-stops: var(--en-gradient-from), {}, var(--en-gradient-to, {});",
            val, default_to
        )
        .is_ok()
    }

    fn get_css_for_modifier(
        &self,
        config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(color) = default_colors::get(config, modifier.content()) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct GradientToPlugin;

impl Plugin for GradientToPlugin {
    fn namespace(&self) -> &str {
        "to"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        let val = if val.contains("--en-opacity") {
            val.replace(" / var(--en-opacity)", "")
        } else {
            val.to_string()
        };

        write!(css_content, "--en-gradient-to: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(color) = default_colors::get(config, modifier.content()) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        // https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
        hint == "list"
            || val
                .split(',')
                .all(|v| v.split('_').all(is_matching_position))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "background-position: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "bottom" => self.css_template_value("bottom", css_content),
            "center" => self.css_template_value("center", css_content),
            "left" => self.css_template_value("left", css_content),
            "left-bottom" => self.css_template_value("left-bottom", css_content),
            "left-top" => self.css_template_value("left-top", css_content),
            "right" => self.css_template_value("right", css_content),
            "right-bottom" => self.css_template_value("right-bottom", css_content),
            "right-top" => self.css_template_value("right-top", css_content),
            "top" => self.css_template_value("top", css_content),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct RepeatPlugin;

impl Plugin for RepeatPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "repeat" => write!(css_content, "background-repeat: repeat;").is_ok(),
            "no-repeat" => write!(css_content, "background-repeat: no-repeat;").is_ok(),
            "repeat-x" => write!(css_content, "background-repeat: repeat-x;").is_ok(),
            "repeat-y" => write!(css_content, "background-repeat: repeat-y;").is_ok(),
            "repeat-round" => write!(css_content, "background-repeat: round;").is_ok(),
            "repeat-space" => write!(css_content, "background-repeat: space;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct SizePlugin;

impl Plugin for SizePlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length"
            || val.split(',').all(|v| {
                v.split('_').all(|v| {
                    is_matching_length(v)
                        || is_matching_percentage(v)
                        || ["contain", "cover", "auto"].contains(&v)
                })
            })
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "background-size: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "auto" => self.css_template_value("auto", css_content),
            "cover" => self.css_template_value("cover", css_content),
            "contain" => self.css_template_value("contain", css_content),
            _ => false,
        }
    }
}
