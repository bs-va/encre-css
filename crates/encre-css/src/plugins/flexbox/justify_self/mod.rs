#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "justify-self"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["stretch", "start", "center", "end", "auto"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "stretch" => writeln!(buffer, "{indentation}justify-self: stretch;")?,
                "start" => writeln!(buffer, "{indentation}justify-self: start;")?,
                "center" => writeln!(buffer, "{indentation}justify-self: center;")?,
                "end" => writeln!(buffer, "{indentation}justify-self: end;")?,
                "auto" => writeln!(buffer, "{indentation}justify-self: auto;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
