#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "min-w"
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "full" => writeln!(buffer, "{indentation}min-width: 100%;")?,
                "min" => writeln!(buffer, "{indentation}min-width: min-content;")?,
                "max" => writeln!(buffer, "{indentation}min-width: max-content;")?,
                "fit" => writeln!(buffer, "{indentation}min-width: fit-content;")?,
                "screen" => writeln!(buffer, "{indentation}min-width: 100vw;")?,
                _ => writeln!(
                    buffer,
                    "{indentation}min-width: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}min-width: {value};")?;
            }
        }

        Ok(())
    }
}
