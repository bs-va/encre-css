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
        "underline-offset"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok() || *value == "auto",
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "percentage"
                    || (hint.is_empty()
                        && (*value == "auto"
                            || is_matching_length(value)
                            || is_matching_percentage(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "auto" {
                    return writeln!(context.buffer, "text-underline-offset: auto;");
                }

                writeln!(context.buffer, "text-underline-offset: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "text-underline-offset: {value};")?;
            }
        }

        Ok(())
    }
}
