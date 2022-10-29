#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["row", "row-reverse", "col", "col-reverse"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "row" => context.buffer.line("flex-direction: row;"),
                "row-reverse" => context.buffer.line("flex-direction: row-reverse;"),
                "col" => context.buffer.line("flex-direction: column;"),
                "col-reverse" => context.buffer.line("flex-direction: column-reverse;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
