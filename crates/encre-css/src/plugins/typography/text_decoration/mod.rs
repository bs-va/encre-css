#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["underline", "overline", "line-through", "no-underline"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context.buffer.lines([
                    format_args!("-webkit-text-decoration-line: {value};"),
                    format_args!("text-decoration-line: {value};"),
                ]);
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
