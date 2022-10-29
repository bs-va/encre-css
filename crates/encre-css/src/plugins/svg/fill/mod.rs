#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
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
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "fill: {};",
                color::get(context.config, value, None).unwrap()
            )),
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("fill: {value};"));
            }
        }
    }
}
