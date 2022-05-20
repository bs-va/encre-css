use super::Plugin;
use crate::{config::Config, selector::Modifier};
use crate::utils::{default_colors, value_matchers::*};

use std::fmt::Write;

const CSS_SHADOW: &str = "box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);";

#[derive(Debug)]
pub struct MixBlendModePlugin;

impl Plugin for MixBlendModePlugin {
    fn namespace(&self) -> &str {
        "mix-blend"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "normal" => write!(css_content, "mix-blend-mode: normal;").is_ok(),
            "multiply" => write!(css_content, "mix-blend-mode: multiply;").is_ok(),
            "screen" => write!(css_content, "mix-blend-mode: screen;").is_ok(),
            "overlay" => write!(css_content, "mix-blend-mode: overlay;").is_ok(),
            "darken" => write!(css_content, "mix-blend-mode: darken;").is_ok(),
            "lighten" => write!(css_content, "mix-blend-mode: lighten;").is_ok(),
            "color-dodge" => write!(css_content, "mix-blend-mode: color-dodge;").is_ok(),
            "color-burn" => write!(css_content, "mix-blend-mode: color-burn;").is_ok(),
            "hard-light" => write!(css_content, "mix-blend-mode: hard-light;").is_ok(),
            "soft-light" => write!(css_content, "mix-blend-mode: soft-light;").is_ok(),
            "difference" => write!(css_content, "mix-blend-mode: difference;").is_ok(),
            "exclusion" => write!(css_content, "mix-blend-mode: exclusion;").is_ok(),
            "hue" => write!(css_content, "mix-blend-mode: hue;").is_ok(),
            "saturation" => write!(css_content, "mix-blend-mode: saturation;").is_ok(),
            "color" => write!(css_content, "mix-blend-mode: color;").is_ok(),
            "luminosity" => write!(css_content, "mix-blend-mode: luminosity;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct BackgroundBlendModePlugin;

impl Plugin for BackgroundBlendModePlugin {
    fn namespace(&self) -> &str {
        "bg-blend"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "normal" => write!(css_content, "background-blend-mode: normal;").is_ok(),
            "multiply" => write!(css_content, "background-blend-mode: multiply;").is_ok(),
            "screen" => write!(css_content, "background-blend-mode: screen;").is_ok(),
            "overlay" => write!(css_content, "background-blend-mode: overlay;").is_ok(),
            "darken" => write!(css_content, "background-blend-mode: darken;").is_ok(),
            "lighten" => write!(css_content, "background-blend-mode: lighten;").is_ok(),
            "color-dodge" => write!(css_content, "background-blend-mode: color-dodge;").is_ok(),
            "color-burn" => write!(css_content, "background-blend-mode: color-burn;").is_ok(),
            "hard-light" => write!(css_content, "background-blend-mode: hard-light;").is_ok(),
            "soft-light" => write!(css_content, "background-blend-mode: soft-light;").is_ok(),
            "difference" => write!(css_content, "background-blend-mode: difference;").is_ok(),
            "exclusion" => write!(css_content, "background-blend-mode: exclusion;").is_ok(),
            "hue" => write!(css_content, "background-blend-mode: hue;").is_ok(),
            "saturation" => write!(css_content, "background-blend-mode: saturation;").is_ok(),
            "color" => write!(css_content, "background-blend-mode: color;").is_ok(),
            "luminosity" => write!(css_content, "background-blend-mode: luminosity;").is_ok(),
            _ => false,
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "box-shadow: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "" => write!(
                css_content,
                "--en-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
--en-shadow-colored: 0 1px 3px 0 var(--en-shadow-color), 0 1px 2px -1px var(--en-shadow-color);
{}", CSS_SHADOW).is_ok(),
            "sm" => write!(css_content, "--en-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
--en-shadow-colored: 0 1px 2px 0 var(--en-shadow-color);
{}", CSS_SHADOW).is_ok(),
            "md" => write!(
                css_content,
                "--en-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
--en-shadow-colored: 0 4px 6px -1px var(--en-shadow-color), 0 2px 4px -2px var(--en-shadow-color);
{}", CSS_SHADOW).is_ok(),
            "lg" => write!(
                css_content,
                "--en-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
--en-shadow-colored: 0 10px 15px -3px var(--en-shadow-color), 0 4px 6px -4px var(--en-shadow-color);
{}", CSS_SHADOW).is_ok(),
            "xl" => write!(
                css_content,
                "--en-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
--en-shadow-colored: 0 20px 25px -5px var(--en-shadow-color), 0 8px 10px -6px var(--en-shadow-color);
{}", CSS_SHADOW).is_ok(),
            "2xl" => write!(css_content, "--en-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
--en-shadow-colored: 0 25px 50px -12px var(--en-shadow-color);
{}", CSS_SHADOW).is_ok(),
            "inner" => {
                write!(css_content, "--en-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);
--en-shadow-colored: inset 0 2px 4px 0 var(--en-shadow-color);
{}", CSS_SHADOW).is_ok()
            }
            "none" => self.css_template_value("none", css_content),
            _ => false,
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        if val.contains("--en-opacity") {
            write!(
                css_content,
                "--en-shadow-color: {};
--en-shadow: var(--en-shadow-colored);",
                val.replace("/ var(--en-opacity)", "")
            )
            .is_ok()
        } else {
            write!(
                css_content,
                "--en-shadow-color: {val};
--en-shadow: var(--en-shadow-colored);"
            )
            .is_ok()
        }
    }

    fn get_css_for_modifier(&self, config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        if let Some(color) = default_colors::get(config, modifier.content()) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "opacity"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.to_f32() {
            write!(css_content, "opacity: {};", opacity_value / 100.).is_ok()
        } else {
            false
        }
    }
}
