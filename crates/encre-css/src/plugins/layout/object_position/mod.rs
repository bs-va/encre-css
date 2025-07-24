#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "bottom",
                "center",
                "left",
                "bottom-left",
                "top-left",
                "right",
                "bottom-right",
                "top-right",
                "top",
            ]
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "position" || (hint.is_empty() && is_matching_position(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "object-position: {};",
                value.replace('-', " ")
            )),
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("object-position: {value};"));
            }
        }
    }
}
