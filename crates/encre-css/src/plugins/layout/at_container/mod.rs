#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.is_empty(),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { .. } => {
                context
                    .buffer
                    .line(format_args!("container-type: inline-size;"));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("container-type: {value};"));
            }
        }
    }
}
