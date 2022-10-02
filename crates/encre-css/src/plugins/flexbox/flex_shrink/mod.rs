#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "shrink"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.is_empty() || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            #[allow(clippy::cast_precision_loss)]
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(buffer, "{indentation}flex-shrink: 1;")?,
                _ => writeln!(buffer, "{indentation}flex-shrink: {value};")?,
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
