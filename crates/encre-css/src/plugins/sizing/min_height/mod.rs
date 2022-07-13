#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "min-h"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["full", "min", "max", "fit", "screen"].contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "full" => writeln!(context.buffer, "min-height: 100%;")?,
                "min" => writeln!(context.buffer, "min-height: min-content;")?,
                "max" => writeln!(context.buffer, "min-height: max-content;")?,
                "fit" => writeln!(context.buffer, "min-height: fit-content;")?,
                "screen" => writeln!(context.buffer, "min-height: 100vh;")?,
                _ => writeln!(
                    context.buffer,
                    "min-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "min-height: {value};")?;
            }
        }

        Ok(())
    }
}
