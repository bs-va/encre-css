#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl",
                "9xl",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "percentage"
                    || *hint == "absolute-size"
                    || *hint == "relative-size"
                    || (hint.is_empty()
                        && (is_matching_length(value)
                            || is_matching_percentage(value)
                            || is_matching_absolute_size(value)
                            || is_matching_relative_size(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "xs" => {
                    context
                        .buffer
                        .lines(["font-size: 0.75rem;", "line-height: 1rem;"]);
                }
                "sm" => {
                    context
                        .buffer
                        .lines(["font-size: 0.875rem;", "line-height: 1.25rem;"]);
                }
                "base" => {
                    context
                        .buffer
                        .lines(["font-size: 1rem;", "line-height: 1.5rem;"]);
                }
                "lg" => {
                    context
                        .buffer
                        .lines(["font-size: 1.125rem;", "line-height: 1.75rem;"]);
                }
                "xl" => {
                    context
                        .buffer
                        .lines(["font-size: 1.25rem;", "line-height: 1.75rem;"]);
                }
                "2xl" => {
                    context
                        .buffer
                        .lines(["font-size: 1.5rem;", "line-height: 2rem;"]);
                }
                "3xl" => {
                    context
                        .buffer
                        .lines(["font-size: 1.875rem;", "line-height: 2.25rem;"]);
                }
                "4xl" => {
                    context
                        .buffer
                        .lines(["font-size: 2.25rem;", "line-height: 2.5rem;"]);
                }
                "5xl" => {
                    context
                        .buffer
                        .lines(["font-size: 3rem;", "line-height: 1;"]);
                }
                "6xl" => {
                    context
                        .buffer
                        .lines(["font-size: 3.75rem;", "line-height: 1;"]);
                }
                "7xl" => {
                    context
                        .buffer
                        .lines(["font-size: 4.5rem;", "line-height: 1;"]);
                }
                "8xl" => {
                    context
                        .buffer
                        .lines(["font-size: 6rem;", "line-height: 1;"]);
                }
                "9xl" => {
                    context
                        .buffer
                        .lines(["font-size: 8rem;", "line-height: 1;"]);
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("font-size: {value};"));
            }
        }
    }
}
