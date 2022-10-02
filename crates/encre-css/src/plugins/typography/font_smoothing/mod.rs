#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["antialised", "subpixel-antialised"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "antialised" => {
                    writeln!(buffer, "{indentation}-webkit-font-smoothing: antialiased;\n{indentation}-moz-osx-font-smoothing: grayscale;")?;
                }
                "subpixel-antialised" => {
                    writeln!(buffer, "{indentation}-webkit-font-smoothing: auto;\n{indentation}-moz-osx-font-smoothing: auto;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
