#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["x", "y", "both", "mandatory", "proximity", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "none" => {
                    context
                        .buffer
                        .lines(["-ms-scroll-snap-type: none;", "scroll-snap-type: none;"]);
                }
                "x" => {
                    context.buffer.lines([
                        "-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);",
                        "scroll-snap-type: x var(--en-scroll-snap-strictness);",
                    ]);
                }
                "y" => {
                    context.buffer.lines([
                        "-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);",
                        "scroll-snap-type: y var(--en-scroll-snap-strictness);",
                    ]);
                }
                "both" => {
                    context.buffer.lines([
                        "-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);",
                        "scroll-snap-type: both var(--en-scroll-snap-strictness);",
                    ]);
                }
                "mandatory" => context
                    .buffer
                    .line("--en-scroll-snap-strictness: mandatory;"),
                "proximity" => context
                    .buffer
                    .line("--en-scroll-snap-strictness: proximity;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
