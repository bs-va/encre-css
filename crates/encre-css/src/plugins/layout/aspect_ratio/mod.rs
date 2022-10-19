#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "aspect"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["auto", "square", "video"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => context.buffer.line("aspect-ratio: auto;"),
                "square" => context.buffer.line("aspect-ratio: 1 / 1;"),
                "video" => context.buffer.line("aspect-ratio: 16 / 9;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => context
                .buffer
                .line(format_args!("aspect-ratio: {};", value.replace('/', " / "),)),
        }
    }
}
