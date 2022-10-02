#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
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
                "none",
                "gradient-to-t",
                "gradient-to-tr",
                "gradient-to-r",
                "gradient-to-br",
                "gradient-to-b",
                "gradient-to-bl",
                "gradient-to-l",
                "gradient-to-tl",
            ]
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "image" || *hint == "url" || (hint.is_empty() && is_matching_image(value))
            }
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "none" => writeln!(buffer, "background-image: none;")?,
                "gradient-to-t" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to top, var(--en-gradient-stops));"
                )?,
                "gradient-to-tr" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to top right, var(--en-gradient-stops));"
                )?,
                "gradient-to-r" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to right, var(--en-gradient-stops));"
                )?,
                "gradient-to-br" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to bottom right, var(--en-gradient-stops));"
                )?,
                "gradient-to-b" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to bottom, var(--en-gradient-stops));"
                )?,
                "gradient-to-bl" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to bottom left, var(--en-gradient-stops));"
                )?,
                "gradient-to-l" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to left, var(--en-gradient-stops));"
                )?,
                "gradient-to-tl" => writeln!(
                    buffer,
                    "{indentation}background-image: linear-gradient(to top left, var(--en-gradient-stops));"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}background-image: {value};")?;
            }
        }

        Ok(())
    }
}
