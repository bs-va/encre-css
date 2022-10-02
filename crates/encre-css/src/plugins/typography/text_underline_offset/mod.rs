#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "underline-offset"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok() || *value == "auto",
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "percentage"
                    || (hint.is_empty()
                        && (*value == "auto"
                            || is_matching_length(value)
                            || is_matching_percentage(value)))
            }
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "auto" {
                    return writeln!(buffer, "{indentation}text-underline-offset: auto;");
                }

                writeln!(buffer, "{indentation}text-underline-offset: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}text-underline-offset: {value};")?;
            }
        }

        Ok(())
    }
}
