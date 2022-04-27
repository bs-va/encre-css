use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct MixBlendModePlugin;

impl Plugin for MixBlendModePlugin {
    fn namespace(&self) -> &str {
        "mix-blend"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "normal" => write!(css_content, "mix-blend-mode: normal;"),
            "multiply" => write!(css_content, "mix-blend-mode: multiply;"),
            "screen" => write!(css_content, "mix-blend-mode: screen;"),
            "overlay" => write!(css_content, "mix-blend-mode: overlay;"),
            "darken" => write!(css_content, "mix-blend-mode: darken;"),
            "lighten" => write!(css_content, "mix-blend-mode: lighten;"),
            "color-dodge" => write!(css_content, "mix-blend-mode: color-dodge;"),
            "color-burn" => write!(css_content, "mix-blend-mode: color-burn;"),
            "hard-light" => write!(css_content, "mix-blend-mode: hard-light;"),
            "soft-light" => write!(css_content, "mix-blend-mode: soft-light;"),
            "difference" => write!(css_content, "mix-blend-mode: difference;"),
            "exclusion" => write!(css_content, "mix-blend-mode: exclusion;"),
            "hue" => write!(css_content, "mix-blend-mode: hue;"),
            "saturation" => write!(css_content, "mix-blend-mode: saturation;"),
            "color" => write!(css_content, "mix-blend-mode: color;"),
            "luminosity" => write!(css_content, "mix-blend-mode: luminosity;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct BackgroundBlendModePlugin;

impl Plugin for BackgroundBlendModePlugin {
    fn namespace(&self) -> &str {
        "bg-blend"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "normal" => write!(css_content, "background-blend-mode: normal;"),
            "multiply" => write!(css_content, "background-blend-mode: multiply;"),
            "screen" => write!(css_content, "background-blend-mode: screen;"),
            "overlay" => write!(css_content, "background-blend-mode: overlay;"),
            "darken" => write!(css_content, "background-blend-mode: darken;"),
            "lighten" => write!(css_content, "background-blend-mode: lighten;"),
            "color-dodge" => write!(css_content, "background-blend-mode: color-dodge;"),
            "color-burn" => write!(css_content, "background-blend-mode: color-burn;"),
            "hard-light" => write!(css_content, "background-blend-mode: hard-light;"),
            "soft-light" => write!(css_content, "background-blend-mode: soft-light;"),
            "difference" => write!(css_content, "background-blend-mode: difference;"),
            "exclusion" => write!(css_content, "background-blend-mode: exclusion;"),
            "hue" => write!(css_content, "background-blend-mode: hue;"),
            "saturation" => write!(css_content, "background-blend-mode: saturation;"),
            "color" => write!(css_content, "background-blend-mode: color;"),
            "luminosity" => write!(css_content, "background-blend-mode: luminosity;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct BoxShadowPlugin;

impl Plugin for BoxShadowPlugin {
    fn namespace(&self) -> &str {
        "shadow"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_shadow(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "box-shadow: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "" => self.css_template_value(
                "0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);",
                css_content,
            ),
            "sm" => self.css_template_value("0 1px 2px 0 rgba(0, 0, 0, 0.05)", css_content),
            "md" => self.css_template_value(
                "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
                css_content,
            ),
            "lg" => self.css_template_value(
                "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
                css_content,
            ),
            "xl" => self.css_template_value(
                "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",
                css_content,
            ),
            "2xl" => self.css_template_value("0 25px 50px -12px rgba(0, 0, 0, 0.25)", css_content),
            "inner" => {
                self.css_template_value("inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)", css_content)
            }
            "none" => self.css_template_value("none", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct BoxShadowColorPlugin;

impl Plugin for BoxShadowColorPlugin {
    fn namespace(&self) -> &str {
        "shadow"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "--tw-shadow-color: {};",
                val.replace("/ var(--tw-opacity)", "")
            )
        } else {
            write!(css_content, "--tw-shadow-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(color) = default_colors::get(modifier) {
            self.css_template_value(&color, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "opacity"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            write!(css_content, "opacity: {};", opacity_value / 100.)
        } else {
            Ok(())
        }
    }
}
