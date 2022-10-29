#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                [
                    "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "full",
                    "min", "max", "screen", "fit", "none",
                ]
                .contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "none" => context.buffer.line("max-height: none;"),
                "xs" => context.buffer.line("max-height: 20rem;"),
                "sm" => context.buffer.line("max-height: 24rem;"),
                "md" => context.buffer.line("max-height: 28rem;"),
                "lg" => context.buffer.line("max-height: 32rem;"),
                "xl" => context.buffer.line("max-height: 36rem;"),
                "2xl" => context.buffer.line("max-height: 42rem;"),
                "3xl" => context.buffer.line("max-height: 48rem;"),
                "4xl" => context.buffer.line("max-height: 56rem;"),
                "5xl" => context.buffer.line("max-height: 64rem;"),
                "6xl" => context.buffer.line("max-height: 72rem;"),
                "7xl" => context.buffer.line("max-height: 80rem;"),
                "full" => context.buffer.line("max-height: 100%;"),
                "min" => context.buffer.line("max-height: min-content;"),
                "max" => context.buffer.line("max-height: max-content;"),
                "screen" => context.buffer.line("max-height: 100vh;"),
                "fit" => context.buffer.line("max-height: fit-content;"),
                _ => context.buffer.line(format_args!(
                    "max-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("max-height: {value};"));
            }
        }
    }
}
