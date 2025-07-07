#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "start" | "end" | "left" | "right" | "both" | "none",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            if *value == "start" {
                context.buffer.line(format_args!("clear: inline-start;"));
            } else if *value == "end" {
                context.buffer.line(format_args!("clear: inline-end;"));
            } else {
                context.buffer.line(format_args!("clear: {value};"));
            }
        }
    }
}
