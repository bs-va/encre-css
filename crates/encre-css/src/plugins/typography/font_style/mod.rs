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
                value: "italic" | "not-italic",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            match *value {
                "italic" => context.buffer.line("font-style: italic;"),
                "not-italic" => context.buffer.line("font-style: normal;"),
                _ => unreachable!(),
            }
        }
    }
}
