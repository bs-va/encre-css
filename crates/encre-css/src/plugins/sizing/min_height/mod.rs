#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "full" => writeln!(buffer, "{indentation}min-height: 100%;")?,
                "min" => writeln!(buffer, "{indentation}min-height: min-content;")?,
                "max" => writeln!(buffer, "{indentation}min-height: max-content;")?,
                "fit" => writeln!(buffer, "{indentation}min-height: fit-content;")?,
                "screen" => writeln!(buffer, "{indentation}min-height: 100vh;")?,
                _ => writeln!(
                    buffer,
                    "{indentation}min-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}min-height: {value};")?;
            }
        }

        Ok(())
    }
}
