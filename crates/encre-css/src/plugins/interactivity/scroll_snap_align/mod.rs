#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["start", "end", "center", "align-none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "start" => context.buffer.line("scroll-snap-align: start;"),
                "end" => context.buffer.line("scroll-snap-align: end;"),
                "center" => context.buffer.line("scroll-snap-align: center;"),
                "align-none" => context.buffer.line("scroll-snap-align: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
