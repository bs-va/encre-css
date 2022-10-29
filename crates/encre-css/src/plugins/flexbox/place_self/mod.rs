#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "start", "center", "end", "stretch"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => context.buffer.line("place-self: auto;"),
                "start" => context.buffer.line("place-self: start;"),
                "center" => context.buffer.line("place-self: center;"),
                "end" => context.buffer.line("place-self: end;"),
                "stretch" => context.buffer.line("place-self: stretch;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
