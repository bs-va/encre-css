#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg-origin"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["border", "padding", "content"].contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                writeln!(buffer, "{indentation}background-origin: {value}-box;")?;
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
