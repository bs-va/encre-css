#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "decoration"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "from-font"].contains(&&**value) || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "percentage"
                    || (hint.is_empty()
                        && (["auto", "from-font"].contains(&&**value)
                            || is_matching_length(value)
                            || is_matching_percentage(value)))
            }
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                if ["auto", "from-font"].contains(&&**value) {
                    return writeln!(buffer, "{indentation}text-decoration-thickness: {value};");
                }

                writeln!(buffer, "{indentation}text-decoration-thickness: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}text-decoration-thickness: {value};")?;
            }
        }

        Ok(())
    }
}
