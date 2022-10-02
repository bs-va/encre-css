#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "object"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "bottom",
                "center",
                "left",
                "left-bottom",
                "left-top",
                "right",
                "right-bottom",
                "right-top",
                "top",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_position(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "bottom" => writeln!(buffer, "{indentation}object-position: bottom;")?,
                "center" => writeln!(buffer, "{indentation}object-position: center;")?,
                "left" => writeln!(buffer, "{indentation}object-position: left;")?,
                "left-bottom" => writeln!(buffer, "{indentation}object-position: left bottom;")?,
                "left-top" => writeln!(buffer, "{indentation}object-position: left top;")?,
                "right" => writeln!(buffer, "{indentation}object-position: right;")?,
                "right-bottom" => writeln!(buffer, "{indentation}object-position: right bottom;")?,
                "right-top" => writeln!(buffer, "{indentation}object-position: right top;")?,
                "top" => writeln!(buffer, "{indentation}object-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}object-position: {value};")?;
            }
        }

        Ok(())
    }
}
