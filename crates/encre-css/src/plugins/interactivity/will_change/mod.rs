#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "scroll", "contents", "transform"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => context.buffer.line("will-change: auto;"),
                "scroll" => context.buffer.line("will-change: scroll-position;"),
                "contents" => context.buffer.line("will-change: contents;"),
                "transform" => context.buffer.line("will-change: transform;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("will-change: {value};"));
            }
        }
    }
}
