#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "underline" | "overline" | "line-through" | "no-underline",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            let value = if *value == "no-underline" {
                "none"
            } else {
                value
            };

            context.buffer.lines([
                format_args!("-webkit-text-decoration-line: {value};"),
                format_args!("text-decoration-line: {value};"),
            ]);
        }
    }
}
