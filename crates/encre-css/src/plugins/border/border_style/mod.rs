#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => is_matching_line_style(value),
            Modifier::Arbitrary { value, .. } => value.split('_').all(is_matching_line_style),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context.buffer.line(format_args!("border-style: {value};"));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("border-style: {value};"));
            }
        }
    }
}
