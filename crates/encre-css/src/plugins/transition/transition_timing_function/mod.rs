#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["linear", "in", "out", "in-out"].contains(value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "linear" => context.buffer.line("transition-timing-function: linear;"),
                "in" => context
                    .buffer
                    .line("transition-timing-function: cubic-bezier(0.4, 0, 1, 1);"),
                "out" => context
                    .buffer
                    .line("transition-timing-function: cubic-bezier(0, 0, 0.2, 1);"),
                "in-out" => context
                    .buffer
                    .line("transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("transition-timing-function: {value};"));
            }
        }
    }
}
