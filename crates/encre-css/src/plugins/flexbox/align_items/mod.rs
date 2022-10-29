#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["stretch", "start", "center", "end", "baseline"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "stretch" => context.buffer.line("align-items: stretch;"),
                "start" => context.buffer.line("align-items: flex-start;"),
                "center" => context.buffer.line("align-items: center;"),
                "end" => context.buffer.line("align-items: flex-end;"),
                "baseline" => context.buffer.line("align-items: baseline;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
