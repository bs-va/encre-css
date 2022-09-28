#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::is_matching_position,
};
use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg"
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
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                // https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
                *hint == "position"
                    || (hint.is_empty() && value.split(',').all(is_matching_position))
            }
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "bottom" => writeln!(buffer, "{indentation}background-position: bottom;")?,
                "center" => writeln!(buffer, "{indentation}background-position: center;")?,
                "left" => writeln!(buffer, "{indentation}background-position: left;")?,
                "left-bottom" => writeln!(buffer, "{indentation}background-position: left-bottom;")?,
                "left-top" => writeln!(buffer, "{indentation}background-position: left-top;")?,
                "right" => writeln!(buffer, "{indentation}background-position: right;")?,
                "right-bottom" => writeln!(buffer, "{indentation}background-position: right-bottom;")?,
                "right-top" => writeln!(buffer, "{indentation}background-position: right-top;")?,
                "top" => writeln!(buffer, "{indentation}background-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}background-position: {value};")?;
            }
        }

        Ok(())
    }
}
