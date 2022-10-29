#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["nowrap", "wrap", "wrap-reverse"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "nowrap" => context.buffer.line("flex-wrap: nowrap;"),
                "wrap" => context.buffer.line("flex-wrap: wrap;"),
                "wrap-reverse" => context.buffer.line("flex-wrap: wrap-reverse;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
