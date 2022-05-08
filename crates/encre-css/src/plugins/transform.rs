use super::Plugin;
use crate::selector::Modifier;
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::Write;

pub const CSS_TRANSFORM: &str = "transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));";

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
                        "--en-translate-x: {};
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
                        "--en-translate-y: {};
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
                        "--en-scale-x: {};
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
                        "--en-scale-y: {};
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
                    "--en-scale-x: {val};
--en-scale-y: {val};
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
                    "--en-rotate: {rotate_value}deg;
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
                        "--en-skew-x: {skew_value}deg;
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
                        "--en-skew-y: {skew_value}deg;
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
