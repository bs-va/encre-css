use super::Plugin;
use crate::selector::Modifier;
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::Write;

pub const CSS_TRANSFORM: &str = "transform: translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y));";

#[derive(Debug)]
pub struct OriginPlugin;

impl Plugin for OriginPlugin {
    fn namespace(&self) -> &str {
        "origin"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_').all(is_matching_position)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transform-origin: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        if modifier.is_one_of(&[
            "center",
            "top",
            "top-right",
            "right",
            "bottom-right",
            "bottom",
            "bottom-left",
            "left",
            "top-left",
        ]) {
            self.css_template_value(modifier.content(), css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct TranslateRotateScaleSkewPlugin;

impl Plugin for TranslateRotateScaleSkewPlugin {
    fn namespace(&self) -> &str {
        ""
    }

    fn get_css_for_modifier(&self, modifier: &Modifier, css_content: &mut String) -> bool {
        let mut result = false;

        if let Some(val) = modifier.strip_prefix("translate-") {
            if let Some(val) = val.strip_prefix("x-") {
                if let Some(length) = default_lengths::get_extended(val, modifier.is_negative()) {
                    write!(
                        css_content,
                        "--tw-translate-x: {};
{}",
                        length, CSS_TRANSFORM
                    )
                    .ok();
                    result = true;
                }
            } else if let Some(val) = val.strip_prefix("y-") {
                if let Some(length) = default_lengths::get_extended(val, modifier.is_negative()) {
                    write!(
                        css_content,
                        "--tw-translate-y: {};
{}",
                        length, CSS_TRANSFORM
                    )
                    .ok();
                    result = true;
                }
            }
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Some(val) = modifier.strip_prefix("scale-") {
            if let Some(val) = val.strip_prefix("x-") {
                if let Ok(scale_value) = val.parse::<f32>() {
                    write!(
                        css_content,
                        "--tw-scale-x: {};
{}",
                        scale_value / 100.,
                        CSS_TRANSFORM
                    )
                    .ok();
                    result = true;
                }
            } else if let Some(val) = val.strip_prefix("y-") {
                if let Ok(scale_value) = val.parse::<f32>() {
                    write!(
                        css_content,
                        "--tw-scale-y: {};
{}",
                        scale_value / 100.,
                        CSS_TRANSFORM
                    )
                    .ok();
                    result = true;
                }
            } else if let Ok(scale_value) = val.parse::<f32>() {
                write!(
                    css_content,
                    "--tw-scale-x: {val};
--tw-scale-y: {val};
{}",
                    CSS_TRANSFORM,
                    val = scale_value / 100.
                )
                .ok();
                result = true;
            }
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Some(val) = modifier.strip_prefix("rotate-") {
            if let Ok(rotate_value) = val.parse::<f32>() {
                write!(
                    css_content,
                    "--tw-rotate: {rotate_value}deg;
{}",
                    CSS_TRANSFORM
                )
                .ok();
                result = true;
            }
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Some(val) = modifier.strip_prefix("skew-") {
            if let Some(val) = val.strip_prefix("x-") {
                if let Ok(skew_value) = val.parse::<f32>() {
                    write!(
                        css_content,
                        "--tw-skew-x: {skew_value}deg;
{}",
                        CSS_TRANSFORM
                    )
                    .ok();
                    result = true;
                }
            } else if let Some(val) = val.strip_prefix("y-") {
                if let Ok(skew_value) = val.parse::<f32>() {
                    write!(
                        css_content,
                        "--tw-skew-y: {skew_value}deg;
{}",
                        CSS_TRANSFORM
                    )
                    .ok();
                    result = true;
                }
            }
        }

        result
    }
}
