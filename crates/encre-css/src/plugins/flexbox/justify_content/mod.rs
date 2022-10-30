#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "start" | "center" | "end" | "between" | "around" | "evenly",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "start" => context.buffer.line("justify-content: flex-start;"),
                "center" => context.buffer.line("justify-content: center;"),
                "end" => context.buffer.line("justify-content: flex-end;"),
                "between" => context.buffer.line("justify-content: space-between;"),
                "around" => context.buffer.line("justify-content: space-around;"),
                "evenly" => context.buffer.line("justify-content: space-evenly;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
