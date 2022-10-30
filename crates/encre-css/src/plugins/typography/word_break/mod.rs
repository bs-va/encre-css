#![doc = include_str!("README.md")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "normal" | "words" | "all",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            match *value {
                "normal" => {
                    context
                        .buffer
                        .lines(["overflow-wrap: normal;", "word-break: normal;"]);
                }
                "words" => context.buffer.line("overflow-wrap: break-word;"),
                "all" => context.buffer.line("word-break: break-all;"),
                _ => unreachable!(),
            }
        }
    }
}
