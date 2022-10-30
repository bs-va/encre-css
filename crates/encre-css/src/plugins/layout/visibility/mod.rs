#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "visible" | "invisible",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin {
            value: "visible", ..
        } = context.modifier
        {
            context.buffer.line("visibility: visible;");
        } else if let Modifier::Builtin {
            value: "invisible", ..
        } = context.modifier
        {
            context.buffer.line("visibility: hidden;");
        }
    }
}
