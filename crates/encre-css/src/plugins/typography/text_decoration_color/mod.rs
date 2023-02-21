#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let value = color::get(context.config, value, None).unwrap();
                context.buffer.lines([
                    format_args!("-webkit-text-decoration-color: {value};"),
                    format_args!("text-decoration-color: {value};"),
                ]);
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.lines([
                    format_args!("-webkit-text-decoration-color: {value};"),
                    format_args!("text-decoration-color: {value};"),
                ]);
            }
        }
    }
}
