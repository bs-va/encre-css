#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "repeat",
                "no-repeat",
                "repeat-x",
                "repeat-y",
                "repeat-round",
                "repeat-space",
            ]
            .contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "repeat" => writeln!(buffer, "{indentation}background-repeat: repeat;")?,
                "no-repeat" => writeln!(buffer, "{indentation}background-repeat: no-repeat;")?,
                "repeat-x" => writeln!(buffer, "{indentation}background-repeat: repeat-x;")?,
                "repeat-y" => writeln!(buffer, "{indentation}background-repeat: repeat-y;")?,
                "repeat-round" => writeln!(buffer, "{indentation}background-repeat: round;")?,
                "repeat-space" => writeln!(buffer, "{indentation}background-repeat: space;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
