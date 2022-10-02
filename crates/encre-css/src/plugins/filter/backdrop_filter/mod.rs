#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::{CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2};
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "backdrop-filter"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => {
                    writeln!(buffer, "{indentation}{}\n{indentation}{}", CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2)?;
                }
                "none" => {
                    writeln!(buffer, "{indentation}-webkit-backdrop-filter: none;\n{indentation}backdrop-filter: none;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
