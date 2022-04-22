use lazy_static::lazy_static;
use regex::Regex;

use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

lazy_static! {
    static ref OPACITY_IN_RGB_REGEX: Regex = Regex::new(r"/.*\)").unwrap();
}

#[derive(Debug)]
pub struct BackgroundColorPlugin;

impl Plugin for BackgroundColorPlugin {
    fn namespace(&self) -> String {
        "bg".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        if val.contains("--tw-opacity") {
            format!(
                "--tw-bg-opacity: 1;
  background-color: {};",
                val.replace("--tw-opacity", "--tw-bg-opacity")
            )
        } else {
            format!("background-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct BackgroundAttachmentPlugin;

impl Plugin for BackgroundAttachmentPlugin {
    fn namespace(&self) -> String {
        "bg".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "fixed" => Some("background-attachment: fixed;".to_string()),
            "local" => Some("background-attachment: local;".to_string()),
            "scroll" => Some("background-attachment: scroll;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BackgroundClipPlugin;

impl Plugin for BackgroundClipPlugin {
    fn namespace(&self) -> String {
        "bg-clip".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "border" => Some("background-clip: border-box;".to_string()),
            "padding" => Some("background-clip: padding-box;".to_string()),
            "content" => Some("background-clip: content-box;".to_string()),
            "text" => Some("background-clip: text;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BackgroundOpacityPlugin;

impl Plugin for BackgroundOpacityPlugin {
    fn namespace(&self) -> String {
        "bg-opacity".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            Some(format!("--tw-bg-opacity: {};", opacity_value / 100.))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BackgroundImagePlugin;

impl Plugin for BackgroundImagePlugin {
    fn namespace(&self) -> String {
        "bg".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        // FIXME: Is this really list? (probably because of comma)
        hint == "list" || is_matching_image(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("background-image: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "bg-none" => Some(self.css_template_value("none")),
            "gradient-to-t" => {
                Some(self.css_template_value("linear-gradient(to top, var(--tw-gradient-stops))"))
            }
            "gradient-to-tr" => Some(
                self.css_template_value("linear-gradient(to top right, var(--tw-gradient-stops))"),
            ),
            "gradient-to-r" => {
                Some(self.css_template_value("linear-gradient(to right, var(--tw-gradient-stops))"))
            }
            "gradient-to-br" => {
                Some(self.css_template_value(
                    "linear-gradient(to bottom right, var(--tw-gradient-stops))",
                ))
            }
            "gradient-to-b" => Some(
                self.css_template_value("linear-gradient(to bottom, var(--tw-gradient-stops))"),
            ),
            "gradient-to-bl" => {
                Some(self.css_template_value(
                    "linear-gradient(to bottom left, var(--tw-gradient-stops))",
                ))
            }
            "gradient-to-l" => {
                Some(self.css_template_value("linear-gradient(to left, var(--tw-gradient-stops))"))
            }
            "gradient-to-tl" => Some(
                self.css_template_value("linear-gradient(to top left, var(--tw-gradient-stops))"),
            ),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BackgroundGradientFromPlugin;

impl Plugin for BackgroundGradientFromPlugin {
    fn namespace(&self) -> String {
        "from".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        // TODO: Prevent `.to_string()`ing
        let default_to = if val == "inherit" || val == "currentColor" {
            "rgb(255 255 255 / 0)".to_string()
        } else {
            OPACITY_IN_RGB_REGEX.replace(val, "/ 0)").to_string()
        };

        let val = if val.contains("--tw-opacity") {
            val.replace(" / var(--tw-opacity)", "")
        } else {
            val.to_string()
        };

        format!(
            "--tw-gradient-from: {val};
  --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, {default_to});"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "inherit" {
            return Some(self.css_template_value("inherit"));
        }

        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct BackgroundGradientViaPlugin;

impl Plugin for BackgroundGradientViaPlugin {
    fn namespace(&self) -> String {
        "via".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        // TODO: Prevent `.to_string()`ing
        let default_to = if val == "inherit" || val == "currentColor" {
            "rgb(255 255 255 / 0)".to_string()
        } else {
            OPACITY_IN_RGB_REGEX.replace(val, "/ 0)").to_string()
        };

        let val = if val.contains("--tw-opacity") {
            val.replace(" / var(--tw-opacity)", "")
        } else {
            val.to_string()
        };

        format!(
            "--tw-gradient-stops: var(--tw-gradient-from), {}, var(--tw-gradient-to, {});",
            val, default_to
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "inherit" {
            return Some(self.css_template_value("inherit"));
        }

        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct BackgroundGradientToPlugin;

impl Plugin for BackgroundGradientToPlugin {
    fn namespace(&self) -> String {
        "to".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        // TODO: Prevent `.to_string()`ing
        let val = if val.contains("--tw-opacity") {
            val.replace(" / var(--tw-opacity)", "")
        } else {
            val.to_string()
        };

        format!("--tw-gradient-to: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "inherit" {
            return Some(self.css_template_value("inherit"));
        }

        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct BackgroundPositionPlugin;

impl Plugin for BackgroundPositionPlugin {
    fn namespace(&self) -> String {
        "bg".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        // TODO: Is that really list?
        // https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
        hint == "list"
            || val
                .split(',')
                .all(|v| v.split('_').all(is_matching_position))
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("background-position: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "bottom" => Some(self.css_template_value("bottom")),
            "center" => Some(self.css_template_value("center")),
            "left" => Some(self.css_template_value("left")),
            "left-bottom" => Some(self.css_template_value("left-bottom")),
            "left-top" => Some(self.css_template_value("left-top")),
            "right" => Some(self.css_template_value("right")),
            "right-bottom" => Some(self.css_template_value("right-bottom")),
            "right-top" => Some(self.css_template_value("right-top")),
            "top" => Some(self.css_template_value("top")),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BackgroundRepeatPlugin;

impl Plugin for BackgroundRepeatPlugin {
    fn namespace(&self) -> String {
        "bg".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "repeat" => Some("background-repeat: repeat;".to_string()),
            "no-repeat" => Some("background-repeat: no-repeat;".to_string()),
            "repeat-x" => Some("background-repeat: repeat-x;".to_string()),
            "repeat-y" => Some("background-repeat: repeat-y;".to_string()),
            "repeat-round" => Some("background-repeat: round;".to_string()),
            "repeat-space" => Some("background-repeat: spacet;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BackgroundSizePlugin;

impl Plugin for BackgroundSizePlugin {
    fn namespace(&self) -> String {
        "bg".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length"
            || val.split(',').all(|v| {
                v.split('_').all(|v| {
                    is_matching_length(v)
                        || is_matching_percentage(v)
                        || is_matching_auto(v)
                        || ["contain", "cover"].contains(&v)
                })
            })
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("background-size: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "auto" => Some(self.css_template_value("auto")),
            "cover" => Some(self.css_template_value("cover")),
            "contain" => Some(self.css_template_value("contain")),
            _ => None,
        }
    }
}
