#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["tighter", "tight", "normal", "wide", "wider", "widest"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => *value == "normal" || is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "tighter" => context.buffer.line("letter-spacing: -0.05em;"),
                "tight" => context.buffer.line("letter-spacing: -0.025em;"),
                "normal" => context.buffer.line("letter-spacing: 0;"),
                "wide" => context.buffer.line("letter-spacing: 0.025em;"),
                "wider" => context.buffer.line("letter-spacing: 0.05em;"),
                "widest" => context.buffer.line("letter-spacing: 0.1em;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("letter-spacing: {value};"));
            }
        }
    }
}
