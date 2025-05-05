#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["xs", "sm", "md", "lg", "xl", "2xl", "3xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "xs" => context.buffer.line("--en-backdrop-blur: blur(4px);"),
                "sm" => context.buffer.line("--en-backdrop-blur: blur(8px);"),
                "md" => context.buffer.line("--en-backdrop-blur: blur(12px);"),
                "lg" => context.buffer.line("--en-backdrop-blur: blur(16px);"),
                "xl" => context.buffer.line("--en-backdrop-blur: blur(24px);"),
                "2xl" => context.buffer.line("--en-backdrop-blur: blur(40px);"),
                "3xl" => context.buffer.line("--en-backdrop-blur: blur(64px);"),
                "none" => context.buffer.line("--en-backdrop-blur: blur(0);"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("--en-backdrop-blur: blur({value});"));
            }
        }

        context.buffer.lines(CSS_BACKDROP_FILTER);
    }
}
