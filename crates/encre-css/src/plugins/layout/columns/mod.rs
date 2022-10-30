#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value, .. } if value.parse::<usize>().map_or(false, |v| v <= 12)
            || [
                "auto", "3xs", "2xs", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl",
                "5xl", "6xl", "7xl",
            ]
            .contains(value))
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            match *value {
                "3xs" => context.buffer.line("columns: 16rem;"),
                "2xs" => context.buffer.line("columns: 18rem;"),
                "xs" => context.buffer.line("columns: 20rem;"),
                "sm" => context.buffer.line("columns: 24rem;"),
                "md" => context.buffer.line("columns: 28rem;"),
                "lg" => context.buffer.line("columns: 32rem;"),
                "xl" => context.buffer.line("columns: 36rem;"),
                "2xl" => context.buffer.line("columns: 42rem;"),
                "3xl" => context.buffer.line("columns: 48rem;"),
                "4xl" => context.buffer.line("columns: 56rem;"),
                "5xl" => context.buffer.line("columns: 64rem;"),
                "6xl" => context.buffer.line("columns: 72rem;"),
                "7xl" => context.buffer.line("columns: 80rem;"),
                _ => context.buffer.line(format_args!("columns: {value};")),
            }
        }
    }
}
