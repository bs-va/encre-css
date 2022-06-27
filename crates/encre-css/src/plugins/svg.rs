use super::{to_css_value, Plugin};
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{color, indent, value_matchers::*},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct FillPlugin;

impl Plugin for FillPlugin {
    fn namespace(&self) -> &str {
        "fill"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        writeln!(context.buffer, "fill: {value};")
    }
}

#[derive(Debug)]
pub struct StrokeColorPlugin;

impl Plugin for StrokeColorPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        writeln!(context.buffer, "stroke: {value};")
    }
}

#[derive(Debug)]
pub struct StrokeWidthPlugin;

impl Plugin for StrokeWidthPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => {
                is_matching_length(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "stroke-width: {value}px;")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "stroke-width: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}
