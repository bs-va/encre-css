#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "place-self"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "start", "center", "end", "stretch"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(buffer, "{indentation}place-self: auto;")?,
                "start" => writeln!(buffer, "{indentation}place-self: start;")?,
                "center" => writeln!(buffer, "{indentation}place-self: center;")?,
                "end" => writeln!(buffer, "{indentation}place-self: end;")?,
                "stretch" => writeln!(buffer, "{indentation}place-self: stretch;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
