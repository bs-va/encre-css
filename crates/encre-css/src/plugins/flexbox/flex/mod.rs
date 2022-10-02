#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["1", "auto", "initial", "none"].contains(value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "1" => writeln!(buffer, "{indentation}flex: 1 1 0%;")?,
                "auto" => writeln!(buffer, "{indentation}flex: 1 1 auto;")?,
                "initial" => writeln!(buffer, "{indentation}flex: 0 1 auto;")?,
                "none" => writeln!(buffer, "{indentation}flex: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}flex: {value};")?;
            }
        }

        Ok(())
    }
}
