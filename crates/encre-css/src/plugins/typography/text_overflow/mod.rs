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
                value: "truncate" | "text-ellipsis" | "text-clip",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            match *value {
                "truncate" => {
                    context.buffer.lines([
                        "overflow: hidden;",
                        "text-overflow: ellipsis;",
                        "white-space: nowrap;",
                    ]);
                }
                "text-ellipsis" => context.buffer.line("text-overflow: ellipsis;"),
                "text-clip" => context.buffer.line("text-overflow: clip;"),
                _ => unreachable!(),
            }
        }
    }
}
