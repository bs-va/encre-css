#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "font"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "thin",
                "extralight",
                "light",
                "normal",
                "medium",
                "semibold",
                "bold",
                "extrabold",
                "black",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "number"
                    && (hint.is_empty()
                        && (["normal", "bold", "lighter", "bolder"].contains(&&**value)
                            || is_matching_integer(value)
                            || is_matching_var(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "thin" => context.buffer.line("font-weight: 100;"),
                "extralight" => context.buffer.line("font-weight: 200;"),
                "light" => context.buffer.line("font-weight: 300;"),
                "normal" => context.buffer.line("font-weight: 400;"),
                "medium" => context.buffer.line("font-weight: 500;"),
                "semibold" => context.buffer.line("font-weight: 600;"),
                "bold" => context.buffer.line("font-weight: 700;"),
                "extrabold" => context.buffer.line("font-weight: 800;"),
                "black" => context.buffer.line("font-weight: 900;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("font-weight: {value};"));
            }
        }
    }
}
