#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "ring-offset"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length" || (hint.is_empty() && is_matching_length(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);")?;
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                writeln!(context.buffer, "--en-ring-offset-width: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-ring-offset-width: {value};",)?;
            }
        }

        Ok(())
    }
}
