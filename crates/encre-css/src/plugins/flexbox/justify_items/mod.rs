#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["stretch", "start", "center", "end", "auto"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "stretch" => context.buffer.line("justify-items: stretch;"),
                "start" => context.buffer.line("justify-items: start;"),
                "center" => context.buffer.line("justify-items: center;"),
                "end" => context.buffer.line("justify-items: end;"),
                "auto" => context.buffer.line("justify-items: auto;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
