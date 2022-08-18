#![doc = include_str!("README.md")]
#![doc(alias = "border")]
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
        "ring"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                value.is_empty() || *value == "inset" || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length" || (hint.is_empty() && is_matching_length(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "inset" {
                    return writeln!(context.buffer, "--en-ring-inset: inset;");
                }

                writeln!(context.buffer, "--en-ring-shadow: var(--en-ring-inset) 0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color);", if value.is_empty() { "3px" } else { value })?;
            }
            Modifier::Arbitrary { value, .. } => writeln!(context.buffer, "--en-ring-shadow: var(--en-ring-inset) 0 0 0 calc({value} + var(--en-ring-offset-width)) var(--en-ring-color);")?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "box-shadow: var(--en-ring-offset-shadow), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);")?;

        Ok(())
    }
}
