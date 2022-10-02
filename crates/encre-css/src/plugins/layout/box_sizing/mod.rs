#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "box"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["border", "content"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                writeln!(buffer, "{indentation}box-sizing: {value}-box;")?;
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
