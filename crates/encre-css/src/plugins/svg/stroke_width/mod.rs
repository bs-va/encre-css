#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "number"
                    || *hint == "percentage"
                    || (hint.is_empty()
                        && (is_matching_length(value) || is_matching_percentage(value)))
            }
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                writeln!(buffer, "{indentation}stroke-width: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}stroke-width: {value};")?;
            }
        }

        Ok(())
    }
}
