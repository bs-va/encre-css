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

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "antialised" => {
                    context.buffer.lines([
                        "-webkit-font-smoothing: antialiased;",
                        "-moz-osx-font-smoothing: grayscale;",
                    ]);
                }
                "subpixel-antialised" => {
                    context.buffer.lines([
                        "-webkit-font-smoothing: auto;",
                        "-moz-osx-font-smoothing: auto;",
                    ]);
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
