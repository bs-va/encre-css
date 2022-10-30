#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "auto"
                    | "pan-x"
                    | "pan-left"
                    | "pan-right"
                    | "pan-y"
                    | "pan-up"
                    | "pan-down"
                    | "pinch-zoom"
                    | "manipulation"
                    | "none",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            context.buffer.line(format_args!("touch-action: {value};"));
        }
    }
}
