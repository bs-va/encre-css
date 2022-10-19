#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "origin"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "center",
                "top",
                "top-right",
                "right",
                "bottom-right",
                "bottom",
                "bottom-left",
                "left",
                "top-left",
            ]
            .contains(value),
            Modifier::Arbitrary { value, .. } => is_matching_position(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context
                    .buffer
                    .line(format_args!("transform-origin: {value};"));
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("transform-origin: {value};"));
            }
        }
    }
}
