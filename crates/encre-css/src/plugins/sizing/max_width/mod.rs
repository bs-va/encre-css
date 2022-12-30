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
                    "xs",
                    "sm",
                    "md",
                    "lg",
                    "xl",
                    "2xl",
                    "3xl",
                    "4xl",
                    "5xl",
                    "6xl",
                    "7xl",
                    "full",
                    "min",
                    "max",
                    "fit",
                    "prose",
                    "screen",
                    "screen-sm",
                    "screen-md",
                    "screen-lg",
                    "screen-lg",
                    "screen-xl",
                    "screen-2xl",
                    "none",
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
                "none" => context.buffer.line("max-width: none;"),
                "xs" => context.buffer.line("max-width: 20rem;"),
                "sm" => context.buffer.line("max-width: 24rem;"),
                "md" => context.buffer.line("max-width: 28rem;"),
                "lg" => context.buffer.line("max-width: 32rem;"),
                "xl" => context.buffer.line("max-width: 36rem;"),
                "2xl" => context.buffer.line("max-width: 42rem;"),
                "3xl" => context.buffer.line("max-width: 48rem;"),
                "4xl" => context.buffer.line("max-width: 56rem;"),
                "5xl" => context.buffer.line("max-width: 64rem;"),
                "6xl" => context.buffer.line("max-width: 72rem;"),
                "7xl" => context.buffer.line("max-width: 80rem;"),
                "full" => context.buffer.line("max-width: 100%;"),
                "min" => context.buffer.line("max-width: min-content;"),
                "max" => context.buffer.line("max-width: max-content;"),
                "fit" => context.buffer.line("max-width: fit-content;"),
                "prose" => context.buffer.line("max-width: 65ch;"),
                "screen" => context.buffer.line("max-width: 100vw;"),
                "screen-sm" => context.buffer.line("max-width: 640px;"),
                "screen-md" => context.buffer.line("max-width: 768px;"),
                "screen-lg" => context.buffer.line("max-width: 1024px;"),
                "screen-xl" => context.buffer.line("max-width: 1280px;"),
                "screen-2xl" => context.buffer.line("max-width: 1536px;"),
                _ => context.buffer.line(format_args!(
                    "max-width: {};",
                    spacing::get(value, *is_negative).unwrap()
                )),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("max-width: {value};"));
            }
        }
    }
}
