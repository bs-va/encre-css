#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "auto"
                    | "x-auto"
                    | "y-auto"
                    | "contain"
                    | "x-contain"
                    | "y-contain"
                    | "none"
                    | "x-none"
                    | "y-none",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => context.buffer.line("overscroll-behavior: auto;"),
                "x-auto" => context.buffer.line("overscroll-behavior-x: auto;"),
                "y-auto" => context.buffer.line("overscroll-behavior-y: auto;"),
                "contain" => context.buffer.line("overscroll-behavior: contain;"),
                "x-contain" => context.buffer.line("overscroll-behavior-x: contain;"),
                "y-contain" => context.buffer.line("overscroll-behavior-y: contain;"),
                "none" => context.buffer.line("overscroll-behavior: none;"),
                "x-none" => context.buffer.line("overscroll-behavior-x: none;"),
                "y-none" => context.buffer.line("overscroll-behavior-y: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
