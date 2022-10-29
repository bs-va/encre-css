#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "hidden",
                "x-hidden",
                "y-hidden",
                "visible",
                "x-visible",
                "y-visible",
                "scroll",
                "x-scroll",
                "y-scroll",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => context.buffer.line("overflow: auto;"),
                "x-auto" => context.buffer.line("overflow-x: auto;"),
                "y-auto" => context.buffer.line("overflow-y: auto;"),
                "hidden" => context.buffer.line("overflow: hidden;"),
                "x-hidden" => context.buffer.line("overflow-x: hidden;"),
                "y-hidden" => context.buffer.line("overflow-y: hidden;"),
                "visible" => context.buffer.line("overflow: visible;"),
                "x-visible" => context.buffer.line("overflow-x: visible;"),
                "y-visible" => context.buffer.line("overflow-y: visible;"),
                "scroll" => context.buffer.line("overflow: scroll;"),
                "x-scroll" => context.buffer.line("overflow-x: scroll;"),
                "y-scroll" => context.buffer.line("overflow-y: scroll;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
