#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{
        indent,
        value_matchers::{is_matching_length, is_matching_percentage},
    },
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "decoration"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "from-font"].contains(&&**value) || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "percentage"
                    || (hint.is_empty()
                        && (["auto", "from-font"].contains(&&**value)
                            || is_matching_length(value)
                            || is_matching_percentage(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if ["auto", "from-font"].contains(&&**value) {
                    return writeln!(context.buffer, "text-decoration-thickness: {value};");
                }

                writeln!(context.buffer, "text-decoration-thickness: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "text-decoration-thickness: {value};")?;
            }
        }

        Ok(())
    }
}
