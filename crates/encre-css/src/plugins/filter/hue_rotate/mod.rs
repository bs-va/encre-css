#![doc = include_str!("README.md")]
use super::CSS_FILTER;
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, format_negative, value_matchers::is_matching_angle},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "hue-rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().map_or(false, |v| v <= 360),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "--en-hue-rotate: hue-rotate({}{}deg);",
                format_negative(is_negative),
                value
            )?,
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-hue-rotate: hue-rotate({});",
                to_css_value(value)
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}
