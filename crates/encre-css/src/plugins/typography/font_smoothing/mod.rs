#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "antialised" | "subpixel-antialised",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            match *value {
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
            }
        }
    }
}
