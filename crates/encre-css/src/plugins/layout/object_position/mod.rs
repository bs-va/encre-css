#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "bottom",
                "center",
                "left",
                "left-bottom",
                "left-top",
                "right",
                "right-bottom",
                "right-top",
                "top",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_position(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "bottom" => context.buffer.line("object-position: bottom;"),
                "center" => context.buffer.line("object-position: center;"),
                "left" => context.buffer.line("object-position: left;"),
                "left-bottom" => context.buffer.line("object-position: left bottom;"),
                "left-top" => context.buffer.line("object-position: left top;"),
                "right" => context.buffer.line("object-position: right;"),
                "right-bottom" => context.buffer.line("object-position: right bottom;"),
                "right-top" => context.buffer.line("object-position: right top;"),
                "top" => context.buffer.line("object-position: top;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("object-position: {value};"));
            }
        }
    }
}
