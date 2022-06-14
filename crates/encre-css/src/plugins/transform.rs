use super::Plugin;
use crate::utils::{default_lengths, format_negative, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::{self, Write};

// TODO: Avoid repeating this CSS in all transform classes
pub const CSS_TRANSFORM: &str = "transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));";

pub struct OriginPlugin;

impl Plugin for OriginPlugin {
    fn namespace(&self) -> &str {
        "origin"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
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
            .contains(&value.as_str()),
            Modifier::Arbitrary { value, .. } => value.split('_').all(is_matching_position),
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        match modifier {
            Modifier::Basic { value, .. } | Modifier::Arbitrary { value, .. } => {
                indent(indentation, buffer)?;
                writeln!(buffer, "transform-origin: {value};")?;
            }
        }

        Ok(())
    }
}

pub fn translate_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            default_lengths::get_extended(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { hint, value } => hint == "length" || is_matching_length(value),
    }
}

pub fn translate_handle(
    css_prop: &str,
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    indent(indentation, buffer)?;
    match modifier {
        Modifier::Basic { is_negative, value } => writeln!(
            buffer,
            "{}: {};",
            css_prop,
            default_lengths::get_extended(value, *is_negative).unwrap()
        )?,
        Modifier::Arbitrary { value, .. } => writeln!(buffer, "{}: {value};", css_prop)?,
    }

    indent(indentation, buffer)?;
    writeln!(buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

pub struct TranslateXPlugin;

impl Plugin for TranslateXPlugin {
    fn namespace(&self) -> &'static str {
        "translate-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        translate_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        translate_handle("--en-translate-x", modifier, indentation, buffer)
    }
}

pub struct TranslateYPlugin;

impl Plugin for TranslateYPlugin {
    fn namespace(&self) -> &'static str {
        "translate-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        translate_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        translate_handle("--en-translate-y", modifier, indentation, buffer)
    }
}

pub struct RotatePlugin;

impl Plugin for RotatePlugin {
    fn namespace(&self) -> &'static str {
        "rotate"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value } => hint == "angle" || is_matching_angle(value),
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { is_negative, value } => writeln!(
                buffer,
                "--en-rotate: {}{}deg;",
                format_negative(is_negative),
                value.parse::<usize>().unwrap(),
            )?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "--en-rotate: {value};")?,
        }

        indent(indentation, buffer)?;
        writeln!(buffer, "{}", CSS_TRANSFORM)?;
        Ok(())
    }
}

pub fn scale_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { .. } => false,
    }
}

pub fn scale_handle(
    css_properties: &[&str],
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    // NOTE: Not-compatible with TailwindCSS, support all values
    match modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "{}: {}{};",
                    css_prop,
                    format_negative(is_negative),
                    value.parse::<usize>().unwrap() as f32 / 100.
                )?;
            }
        }
        Modifier::Arbitrary { .. } => unreachable!(),
    }

    indent(indentation, buffer)?;
    writeln!(buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

pub struct ScalePlugin;

impl Plugin for ScalePlugin {
    fn namespace(&self) -> &'static str {
        "scale"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scale_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scale_handle(
            &["--en-scale-x", "--en-scale-y"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct ScaleXPlugin;

impl Plugin for ScaleXPlugin {
    fn namespace(&self) -> &'static str {
        "scale-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scale_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scale_handle(&["--en-scale-x"], modifier, indentation, buffer)
    }
}

pub struct ScaleYPlugin;

impl Plugin for ScaleYPlugin {
    fn namespace(&self) -> &'static str {
        "scale-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scale_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scale_handle(&["--en-scale-y"], modifier, indentation, buffer)
    }
}

pub fn skew_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { hint, value } => hint == "angle" || is_matching_angle(value),
    }
}

pub fn skew_handle(
    css_prop: &str,
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    // NOTE: Not-compatible with TailwindCSS, support all values
    indent(indentation, buffer)?;
    match modifier {
        Modifier::Basic { is_negative, value } => writeln!(
            buffer,
            "{}: {}{value}deg;",
            css_prop,
            format_negative(is_negative),
        )?,
        Modifier::Arbitrary { value, .. } => writeln!(buffer, "{}: {value};", css_prop)?,
    }

    indent(indentation, buffer)?;
    writeln!(buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

pub struct SkewXPlugin;

impl Plugin for SkewXPlugin {
    fn namespace(&self) -> &'static str {
        "skew-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        skew_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        skew_handle("--en-skew-x", modifier, indentation, buffer)
    }
}

pub struct SkewYPlugin;

impl Plugin for SkewYPlugin {
    fn namespace(&self) -> &'static str {
        "skew-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        skew_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        skew_handle("--en-skew-y", modifier, indentation, buffer)
    }
}
