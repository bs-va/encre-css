#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["border", "padding", "content", "text"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "border" => context.buffer.line("background-clip: border-box;"),
                "padding" => context.buffer.line("background-clip: padding-box;"),
                "content" => context.buffer.line("background-clip: content-box;"),
                "text" => context.buffer.line("background-clip: text;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
