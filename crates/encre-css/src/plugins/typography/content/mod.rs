#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "content"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { .. } => {
                context.buffer.lines([
                    format_args!("--en-content: none;"),
                    format_args!("content: var(--en-content);"),
                ]);
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("--en-content: {value};"));
            }
        }
    }
}
