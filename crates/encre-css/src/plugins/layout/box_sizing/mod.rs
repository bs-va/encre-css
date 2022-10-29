#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["border", "content"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context
                    .buffer
                    .line(format_args!("box-sizing: {value}-box;"));
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
