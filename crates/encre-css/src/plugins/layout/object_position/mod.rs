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
                value: "bottom"
                    | "center"
                    | "left"
                    | "left-bottom"
                    | "left-top"
                    | "right"
                    | "right-bottom"
                    | "right-top"
                    | "top",
                ..
            }
        )
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
