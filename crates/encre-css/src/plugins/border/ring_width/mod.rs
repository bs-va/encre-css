#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                value.is_empty() || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary {
                hint,
                value,
                prefix,
            } => {
                prefix.is_empty()
                    && (*hint == "length" || (hint.is_empty() && is_matching_length(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context.buffer.line(format_args!("--en-ring-shadow: 0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color);", if value.is_empty() { "1" } else { value }));
            }
            Modifier::Arbitrary { value, .. } => context.buffer.line(format_args!("--en-ring-shadow: var(--en-ring-inset) 0 0 0 calc({value} + var(--en-ring-offset-width)) var(--en-ring-color);")),
        }

        context.buffer.line("box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);");
    }
}

#[derive(Debug)]
pub(crate) struct PluginInsetDefinition;

impl Plugin for PluginInsetDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                value.is_empty() || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary {
                hint,
                value,
                prefix,
            } => {
                prefix.is_empty()
                    && (*hint == "length" || (hint.is_empty() && is_matching_length(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context.buffer.line(format_args!("--en-inset-ring-shadow: inset 0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color);", if value.is_empty() { "1" } else { value }));
            }
            Modifier::Arbitrary { value, .. } => context.buffer.line(format_args!("--en-inset-ring-shadow: inset 0 0 0 calc({value} + var(--en-ring-offset-width)) var(--en-ring-color);")),
        }

        context.buffer.line("box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);");
    }
}
