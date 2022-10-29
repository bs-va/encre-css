#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "dashed", "dotted", "double", "hidden", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => context.buffer.line("outline-style: solid;"),
                "none" => {
                    context.buffer.line("outline: 2px solid transparent;");
                    context.buffer.line("outline-offset: 2px;");
                }
                "dashed" => context.buffer.line("outline-style: dashed;"),
                "dotted" => context.buffer.line("outline-style: dotted;"),
                "double" => context.buffer.line("outline-style: double;"),
                "hidden" => context.buffer.line("outline-style: hidden;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
