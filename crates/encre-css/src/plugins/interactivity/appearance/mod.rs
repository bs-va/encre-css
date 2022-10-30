#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value: "none", .. })
    }

    fn handle(&self, context: &mut ContextHandle) {
        context.buffer.lines([
            "-webkit-appearance: none;",
            "-moz-appearance: none;",
            "appearance: none;",
        ]);
    }
}
