#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "none",
                "gradient-to-t",
                "gradient-to-tr",
                "gradient-to-r",
                "gradient-to-br",
                "gradient-to-b",
                "gradient-to-bl",
                "gradient-to-l",
                "gradient-to-tl",
            ]
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "image" || *hint == "url" || (hint.is_empty() && is_matching_image(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "none" => context.buffer.line("background-image: none;"),
                "gradient-to-t" => context
                    .buffer
                    .line("background-image: linear-gradient(to top, var(--en-gradient-stops));"),
                "gradient-to-tr" => context.buffer.line(
                    "background-image: linear-gradient(to top right, var(--en-gradient-stops));",
                ),
                "gradient-to-r" => context
                    .buffer
                    .line("background-image: linear-gradient(to right, var(--en-gradient-stops));"),
                "gradient-to-br" => context.buffer.line(
                    "background-image: linear-gradient(to bottom right, var(--en-gradient-stops));",
                ),
                "gradient-to-b" => context.buffer.line(
                    "background-image: linear-gradient(to bottom, var(--en-gradient-stops));",
                ),
                "gradient-to-bl" => context.buffer.line(
                    "background-image: linear-gradient(to bottom left, var(--en-gradient-stops));",
                ),
                "gradient-to-l" => context
                    .buffer
                    .line("background-image: linear-gradient(to left, var(--en-gradient-stops));"),
                "gradient-to-tl" => context.buffer.line(
                    "background-image: linear-gradient(to top left, var(--en-gradient-stops));",
                ),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("background-image: {value};"));
            }
        }
    }
}
