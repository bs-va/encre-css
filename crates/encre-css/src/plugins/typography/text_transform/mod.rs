#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["uppercase", "lowercase", "capitalize", "normal-case"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "uppercase" => writeln!(buffer, "{indentation}text-transform: uppercase;")?,
                "lowercase" => writeln!(buffer, "{indentation}text-transform: lowercase;")?,
                "capitalize" => writeln!(buffer, "{indentation}text-transform: capitalize;")?,
                "normal-case" => writeln!(buffer, "{indentation}text-transform: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
