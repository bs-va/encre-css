#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["1", "auto", "initial", "none"].contains(value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "1" => context.buffer.line("flex: 1 1 0%;"),
                "auto" => context.buffer.line("flex: 1 1 auto;"),
                "initial" => context.buffer.line("flex: 0 1 auto;"),
                "none" => context.buffer.line("flex: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("flex: {value};"));
            }
        }
    }
}
