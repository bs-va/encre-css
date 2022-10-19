#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg"
    }

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
                let color = color::get(context.config, value, Some("--en-bg-opacity")).unwrap();
                if color.contains("--en-bg-opacity") {
                    context.buffer.line("--en-bg-opacity: 1;");
                }

                context
                    .buffer
                    .line(format_args!("background-color: {color};"));
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("background-color: {value};"));
            }
        }
    }
}
