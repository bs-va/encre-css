#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::is_matching_length,
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        writeln!(buffer, "{indentation}--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);")?;
        match modifier {
            Modifier::Builtin { value, .. } => {
                writeln!(buffer, "{indentation}--en-ring-offset-width: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-ring-offset-width: {value};",)?;
            }
        }

        Ok(())
    }
}
