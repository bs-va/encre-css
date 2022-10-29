#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "auto",
                "pan-x",
                "pan-left",
                "pan-right",
                "pan-y",
                "pan-up",
                "pan-down",
                "pinch-zoom",
                "manipulation",
                "none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context.buffer.line(format_args!("touch-action: {value};"))
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
