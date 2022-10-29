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
                let color = color::get(context.config, value, Some("--en-text-opacity")).unwrap();
                if color.contains("--en-text-opacity") {
                    context.buffer.line("--en-text-opacity: 1;");
                }

                context.buffer.line(format_args!("color: {color};"));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("color: {value};"));
            }
        }
    }
}
