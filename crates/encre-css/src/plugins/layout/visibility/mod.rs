#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["visible", "invisible"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "visible" => writeln!(buffer, "{indentation}visibility: visible;")?,
                "invisible" => writeln!(buffer, "{indentation}visibility: hidden;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
