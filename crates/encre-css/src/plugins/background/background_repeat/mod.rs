#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "repeat",
                "no-repeat",
                "repeat-x",
                "repeat-y",
                "repeat-round",
                "repeat-space",
            ]
            .contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "repeat" => context.buffer.line("background-repeat: repeat;"),
                "no-repeat" => context.buffer.line("background-repeat: no-repeat;"),
                "repeat-x" => context.buffer.line("background-repeat: repeat-x;"),
                "repeat-y" => context.buffer.line("background-repeat: repeat-y;"),
                "repeat-round" => context.buffer.line("background-repeat: round;"),
                "repeat-space" => context.buffer.line("background-repeat: space;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
