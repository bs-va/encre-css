#![doc = include_str!("README.md")]
use super::{CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2};
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{format_negative, indent, value_matchers::is_matching_angle},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "backdrop-hue-rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "--en-backdrop-hue-rotate: hue-rotate({}{}deg);",
                format_negative(is_negative),
                value
            )?,
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-hue-rotate: hue-rotate({value});",
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}
