#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "content"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { .. } => {
                writeln!(buffer, "{indentation}--en-content: none;\n{indentation}content: var(--en-content)")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-content: {value};")?;
            }
        }

        Ok(())
    }
}
