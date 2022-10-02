#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "auto-rows"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["auto", "min", "max", "fr"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(buffer, "{indentation}grid-auto-rows: auto;")?,
                "min" => writeln!(buffer, "{indentation}grid-auto-rows: min-content;")?,
                "max" => writeln!(buffer, "{indentation}grid-auto-rows: max-content;")?,
                "fr" => writeln!(buffer, "{indentation}grid-auto-rows: minmax(0, 1fr);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}grid-auto-rows: {value};")?;
            }
        }

        Ok(())
    }
}
