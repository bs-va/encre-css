use super::{to_css_value, Plugin};
use crate::{context::{ContextCanHandle, ContextHandle}, utils::{format_negative, indent, length, value_matchers::*}, selector::Modifier};

use std::fmt::{self, Write};

// TODO: Avoid repeating this CSS in all transform classes
pub const CSS_TRANSFORM: &str = "transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));";

#[derive(Debug)]
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn namespace(&self) -> &str {
        "transform"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "gpu", "cpu", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" | "cpu" => writeln!(context.buffer, "{}", CSS_TRANSFORM)?,
                "gpu" => writeln!(context.buffer, "transform: translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y));")?,
                "none" => writeln!(context.buffer, "transform: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OriginPlugin;

impl Plugin for OriginPlugin {
    fn namespace(&self) -> &str {
        "origin"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "center",
                "top",
                "top-right",
                "right",
                "bottom-right",
                "bottom",
                "bottom-left",
                "left",
                "top-left",
            ]
            .contains(value),
            Modifier::Arbitrary { value, .. } => value.split('_').all(is_matching_position),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        match context.modifier {
            Modifier::Basic { value, .. } | Modifier::Arbitrary { value, .. } => {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "transform-origin: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}

pub fn translate_can_handle(context: ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            length::get_extended(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

pub fn translate_handle(
    css_prop: &str,
    context: ContextHandle,
) -> fmt::Result {
    indent(context.indentation, context.buffer)?;
    match context.modifier {
        Modifier::Basic { is_negative, value } => writeln!(
            context.buffer,
            "{}: {};",
            css_prop,
            length::get_extended(value, *is_negative).unwrap()
        )?,
        Modifier::Arbitrary { value, .. } => {
            writeln!(context.buffer, "{}: {};", css_prop, to_css_value(value))?
        }
    }

    indent(context.indentation, context.buffer)?;
    writeln!(context.buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

#[derive(Debug)]
pub struct TranslateXPlugin;

impl Plugin for TranslateXPlugin {
    fn namespace(&self) -> &'static str {
        "translate-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        translate_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        translate_handle("--en-translate-x", context)
    }
}

#[derive(Debug)]
pub struct TranslateYPlugin;

impl Plugin for TranslateYPlugin {
    fn namespace(&self) -> &'static str {
        "translate-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        translate_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        translate_handle("--en-translate-y", context)
    }
}

#[derive(Debug)]
pub struct RotatePlugin;

impl Plugin for RotatePlugin {
    fn namespace(&self) -> &'static str {
        "rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { is_negative, value } => writeln!(
                context.buffer,
                "--en-rotate: {}{}deg;",
                format_negative(is_negative),
                value.parse::<usize>().unwrap(),
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-rotate: {};", to_css_value(value))?
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_TRANSFORM)?;
        Ok(())
    }
}

pub fn scale_can_handle(context: ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { .. } => false,
    }
}

pub fn scale_handle(
    css_properties: &[&str],
    context: ContextHandle,
) -> fmt::Result {
    // NOTE: Not-compatible with TailwindCSS, support all values
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "{}: {}{};",
                    css_prop,
                    format_negative(is_negative),
                    value.parse::<usize>().unwrap() as f32 / 100.
                )?;
            }
        }
        Modifier::Arbitrary { .. } => unreachable!(),
    }

    indent(context.indentation, context.buffer)?;
    writeln!(context.buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

#[derive(Debug)]
pub struct ScalePlugin;

impl Plugin for ScalePlugin {
    fn namespace(&self) -> &'static str {
        "scale"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scale_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scale_handle(&["--en-scale-x", "--en-scale-y"], context)
    }
}

#[derive(Debug)]
pub struct ScaleXPlugin;

impl Plugin for ScaleXPlugin {
    fn namespace(&self) -> &'static str {
        "scale-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scale_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scale_handle(&["--en-scale-x"], context)
    }
}

#[derive(Debug)]
pub struct ScaleYPlugin;

impl Plugin for ScaleYPlugin {
    fn namespace(&self) -> &'static str {
        "scale-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scale_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scale_handle(&["--en-scale-y"], context)
    }
}

pub fn skew_can_handle(context: ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { value, .. } => is_matching_angle(value),
    }
}

pub fn skew_handle(
    css_prop: &str,
    context: ContextHandle,
) -> fmt::Result {
    // NOTE: Not-compatible with TailwindCSS, support all values
    indent(context.indentation, context.buffer)?;
    match context.modifier {
        Modifier::Basic { is_negative, value } => writeln!(
            context.buffer,
            "{}: {}{value}deg;",
            css_prop,
            format_negative(is_negative),
        )?,
        Modifier::Arbitrary { value, .. } => {
            writeln!(context.buffer, "{}: {};", css_prop, to_css_value(value))?
        }
    }

    indent(context.indentation, context.buffer)?;
    writeln!(context.buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

#[derive(Debug)]
pub struct SkewXPlugin;

impl Plugin for SkewXPlugin {
    fn namespace(&self) -> &'static str {
        "skew-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        skew_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        skew_handle("--en-skew-x", context)
    }
}

#[derive(Debug)]
pub struct SkewYPlugin;

impl Plugin for SkewYPlugin {
    fn namespace(&self) -> &'static str {
        "skew-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        skew_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        skew_handle("--en-skew-y", context)
    }
}
