use super::Plugin;
use crate::utils::{default_colors, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::{borrow::Cow, fmt::{self, Write}};

pub struct FillPlugin;

impl Plugin for FillPlugin {
    fn namespace(&self) -> &str {
        "fill"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).is_some(),
            Modifier::Arbitrary { hint, value } => hint == "color" || is_matching_color(value),
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        let value = match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => Cow::from(&**value),
        };

        writeln!(buffer, "fill: {value};")
    }
}

pub struct StrokeColorPlugin;

impl Plugin for StrokeColorPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).is_some(),
            Modifier::Arbitrary { hint, value } => hint == "color" || is_matching_color(value),
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        let value = match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => Cow::from(&**value),
        };

        writeln!(buffer, "stroke: {value};")
    }
}

pub struct StrokeWidthPlugin;

impl Plugin for StrokeWidthPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value } => {
                hint == "length" || is_matching_length(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "stroke-width: {value}px;")?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "stroke-width: {value};")?,
        }

        Ok(())
    }
}
