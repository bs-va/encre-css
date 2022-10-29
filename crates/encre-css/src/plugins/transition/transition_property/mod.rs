#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "",
                "none",
                "all",
                "colors",
                "opacity",
                "shadow",
                "transform",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => {
                    context.buffer.lines([
                        "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;",
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
                        "transition-duration: 150ms;",
                    ]);
                }
                "none" => context.buffer.line("transition-property: none;"),
                "all" => {
                    context.buffer.lines([
                        "transition-property: all;",
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
                        "transition-duration: 150ms;",
                    ]);
                }
                "colors" => {
                    context.buffer.lines([
                        "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;",
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);",
                        "transition-duration: 150ms;",
                    ]);
                }
                "opacity" => {
                    context.buffer.lines([
                        "transition-property: opacity;",
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);",
                        "transition-duration: 150ms;",
                    ]);
                }
                "shadow" => {
                    context.buffer.lines([
                        "transition-property: box-shadow;",
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);",
                        "transition-duration: 150ms;",
                    ]);
                }
                "transform" => {
                    context.buffer.lines([
                        "transition-property: transform;",
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);",
                        "transition-duration: 150ms;",
                    ]);
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("transition-property: {value};"));
            }
        }
    }
}
