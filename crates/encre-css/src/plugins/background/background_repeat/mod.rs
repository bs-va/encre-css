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
                value: "repeat"
                    | "no-repeat"
                    | "repeat-x"
                    | "repeat-y"
                    | "repeat-round"
                    | "repeat-space",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "background-repeat: {};",
                match *value {
                    "repeat-round" => "round",
                    "repeat-space" => "space",
                    _ => value,
                }
            )),
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
