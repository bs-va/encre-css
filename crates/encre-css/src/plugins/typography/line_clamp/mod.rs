#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value, .. } if *value == "none" || value.parse::<usize>().is_ok())
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            if *value == "none" {
                context.buffer.line("-webkit-line-clamp: unset;");
            } else {
                context.buffer.lines([
                    format_args!("overflow: hidden;"),
                    format_args!("display: -webkit-box;"),
                    format_args!("-webkit-box-orient: vertical;"),
                    format_args!("-webkit-line-clamp: {value};"),
                ]);
            }
        }
    }
}
