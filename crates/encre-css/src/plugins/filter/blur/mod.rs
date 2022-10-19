#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "blur"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "3xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => context.buffer.line("--en-blur: blur(8px);"),
                "sm" => context.buffer.line("--en-blur: blur(4px);"),
                "md" => context.buffer.line("--en-blur: blur(12px);"),
                "lg" => context.buffer.line("--en-blur: blur(16px);"),
                "xl" => context.buffer.line("--en-blur: blur(24px);"),
                "2xl" => context.buffer.line("--en-blur: blur(40px);"),
                "3xl" => context.buffer.line("--en-blur: blur(64px);"),
                "none" => context.buffer.line("--en-blur: blur(0);"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("--en-blur: blur({value});"));
            }
        }

        context.buffer.line(CSS_FILTER);
    }
}
