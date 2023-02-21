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
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "letter-spacing: {};",
                match *value {
                    "tighter" => "-0.05em",
                    "tight" => "-0.025em",
                    "normal" => "0",
                    "wide" => "0.025em",
                    "wider" => "0.05em",
                    "widest" => "0.1em",
                    _ => unreachable!(),
                }
            )),
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("letter-spacing: {value};"));
            }
        }
    }
}
