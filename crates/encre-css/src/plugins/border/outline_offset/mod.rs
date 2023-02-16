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
                *hint == "length"
                    || *hint == "number"
                    || *hint == "percentage"
                    || (hint.is_empty() && is_matching_length(value)
                        || is_matching_number(value)
                        || is_matching_percentage(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context
                    .buffer
                    .line(format_args!("outline-offset: {value}px;"));
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("outline-offset: {value};"));
            }
        }
    }
}
