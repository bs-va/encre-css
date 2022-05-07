use super::Plugin;
use crate::selector::Modifier;
use crate::utils::{default_colors, value_matchers::*};

use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::Write, borrow::Cow};

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
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "--tw-bg-opacity: 1;
background-color: {};",
                val.replace("--tw-opacity", "--tw-bg-opacity")
            )
            .is_ok()
        } else {
            write!(css_content, "background-color: {val};").is_ok()
        }
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        if let Some(color) = default_colors::get(modifier.content()) {
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

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
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

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
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

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.to_f32() {
            write!(css_content, "--tw-bg-opacity: {};", opacity_value / 100.).is_ok()
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

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "bg-none" => self.css_template_value("none", css_content),
            "gradient-to-t" => self.css_template_value(
                "linear-gradient(to top, var(--tw-gradient-stops))",
                css_content,
            ),
            "gradient-to-tr" => self.css_template_value(
                "linear-gradient(to top right, var(--tw-gradient-stops))",
                css_content,
            ),
            "gradient-to-r" => self.css_template_value(
                "linear-gradient(to right, var(--tw-gradient-stops))",
                css_content,
            ),
            "gradient-to-br" => self.css_template_value(
                "linear-gradient(to bottom right, var(--tw-gradient-stops))",
                css_content,
            ),
            "gradient-to-b" => self.css_template_value(
                "linear-gradient(to bottom, var(--tw-gradient-stops))",
                css_content,
            ),
            "gradient-to-bl" => self.css_template_value(
                "linear-gradient(to bottom left, var(--tw-gradient-stops))",
                css_content,
            ),
            "gradient-to-l" => self.css_template_value(
                "linear-gradient(to left, var(--tw-gradient-stops))",
                css_content,
            ),
            "gradient-to-tl" => self.css_template_value(
                "linear-gradient(to top left, var(--tw-gradient-stops))",
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

        let val = if val.contains("--tw-opacity") {
            val.replace(" / var(--tw-opacity)", "")
        } else {
            val.to_string()
        };

        write!(
            css_content,
            "--tw-gradient-from: {val};
--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, {default_to});"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        if let Some(color) = default_colors::get(modifier.content()) {
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

        let val = if val.contains("--tw-opacity") {
            val.replace(" / var(--tw-opacity)", "")
        } else {
            val.to_string()
        };

        write!(
            css_content,
            "--tw-gradient-stops: var(--tw-gradient-from), {}, var(--tw-gradient-to, {});",
            val, default_to
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        if let Some(color) = default_colors::get(modifier.content()) {
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
        let val = if val.contains("--tw-opacity") {
            val.replace(" / var(--tw-opacity)", "")
        } else {
            val.to_string()
        };

        write!(css_content, "--tw-gradient-to: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        if let Some(color) = default_colors::get(modifier.content()) {
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

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
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

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
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

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "auto" => self.css_template_value("auto", css_content),
            "cover" => self.css_template_value("cover", css_content),
            "contain" => self.css_template_value("contain", css_content),
            _ => false,
        }
    }
}
