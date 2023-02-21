#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "border" | "padding" | "content" | "text",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "background-clip: {};",
                match *value {
                    "border" => "border-box",
                    "padding" => "padding-box",
                    "content" => "content-box",
                    _ => value,
                }
            )),
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
