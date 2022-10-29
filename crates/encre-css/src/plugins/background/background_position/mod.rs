#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
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
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                // https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
                *hint == "position"
                    || (hint.is_empty() && value.split(',').all(is_matching_position))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "bottom" => context.buffer.line("background-position: bottom;"),
                "center" => context.buffer.line("background-position: center;"),
                "left" => context.buffer.line("background-position: left;"),
                "left-bottom" => context.buffer.line("background-position: left-bottom;"),
                "left-top" => context.buffer.line("background-position: left-top;"),
                "right" => context.buffer.line("background-position: right;"),
                "right-bottom" => context.buffer.line("background-position: right-bottom;"),
                "right-top" => context.buffer.line("background-position: right-top;"),
                "top" => context.buffer.line("background-position: top;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("background-position: {value};"));
            }
        }
    }
}
