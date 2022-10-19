#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["uppercase", "lowercase", "capitalize", "normal-case"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "uppercase" => context.buffer.line("text-transform: uppercase;"),
                "lowercase" => context.buffer.line("text-transform: lowercase;"),
                "capitalize" => context.buffer.line("text-transform: capitalize;"),
                "normal-case" => context.buffer.line("text-transform: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
