#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "transition"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "",
                "none",
                "all",
                "colors",
                "opacity",
                "shadow",
                "transform",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => {
                    writeln!(buffer, "{indentation}transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
{indentation}transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
{indentation}transition-duration: 150ms;")?;
                }
                "none" => writeln!(buffer, "{indentation}transition-property: none;")?,
                "all" => {
                    writeln!(buffer, "{indentation}transition-property: all;
{indentation}transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
{indentation}transition-duration: 150ms;")?;
                }
                "colors" => {
                    writeln!(buffer, "{indentation}transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
{indentation}transition-timing-function: cubic-bezier(0.4, 0, 0);
{indentation}transition-duration: 150ms;")?;
                }
                "opacity" => {
                    writeln!(buffer, "{indentation}transition-property: opacity;
{indentation}transition-timing-function: cubic-bezier(0.4, 0, 0);
{indentation}transition-duration: 150ms;")?;
                }
                "shadow" => {
                    writeln!(buffer, "{indentation}transition-property: box-shadow;
{indentation}transition-timing-function: cubic-bezier(0.4, 0, 0);
{indentation}transition-duration: 150ms;")?;
                }
                "transform" => {
                    writeln!(buffer, "{indentation}transition-property: transform;
{indentation}transition-timing-function: cubic-bezier(0.4, 0, 0);
{indentation}transition-duration: 150ms;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}transition-property: {value};")?;
            }
        }

        Ok(())
    }
}
