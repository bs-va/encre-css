#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "ease"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "linear" => writeln!(context.buffer, "transition-timing-function: linear;")?,
                "in" => writeln!(
                    context.buffer,
                    "transition-timing-function: cubic-bezier(0.4, 0, 1, 1);"
                )?,
                "out" => writeln!(
                    context.buffer,
                    "transition-timing-function: cubic-bezier(0, 0, 0.2, 1);"
                )?,
                "in-out" => writeln!(
                    context.buffer,
                    "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "transition-timing-function: {};",
                to_css_value(value)
            )?,
        }

        Ok(())
    }
}
