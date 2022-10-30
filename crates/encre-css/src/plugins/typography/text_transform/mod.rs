#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "uppercase" | "lowercase" | "capitalize" | "normal-case",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            match *value {
                "uppercase" => context.buffer.line("text-transform: uppercase;"),
                "lowercase" => context.buffer.line("text-transform: lowercase;"),
                "capitalize" => context.buffer.line("text-transform: capitalize;"),
                "normal-case" => context.buffer.line("text-transform: none;"),
                _ => unreachable!(),
            }
        }
    }
}
