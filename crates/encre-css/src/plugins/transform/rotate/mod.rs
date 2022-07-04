#![doc = include_str!("README.md")]
use super::CSS_TRANSFORM;
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{format_negative, indent, value_matchers::is_matching_angle},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "--en-rotate: {}{}deg;",
                format_negative(is_negative),
                value.parse::<usize>().unwrap(),
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-rotate: {};", to_css_value(value))?;
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_TRANSFORM)?;
        Ok(())
    }
}
