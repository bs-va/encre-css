#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length" || (hint.is_empty() && is_matching_length(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        context.buffer.line("--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);");

        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context
                    .buffer
                    .line(format_args!("--en-ring-offset-width: {value}px;"));
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("--en-ring-offset-width: {value};"));
            }
        }
    }
}
