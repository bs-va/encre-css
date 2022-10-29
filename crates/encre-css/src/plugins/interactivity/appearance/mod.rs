#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "none",
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { .. } => {
                context.buffer.lines([
                    "-webkit-appearance: none;",
                    "-moz-appearance: none;",
                    "appearance: none;",
                ]);
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
