#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["normal", "always"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "normal" => context.buffer.line("scroll-snap-stop: normal;"),
                "always" => context.buffer.line("scroll-snap-stop: always;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
