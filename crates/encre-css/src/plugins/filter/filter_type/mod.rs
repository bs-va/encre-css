#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "" | "none",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value: "", .. } = context.modifier {
            context.buffer.line(CSS_FILTER);
        } else {
            context.buffer.line("filter: none;");
        }
    }
}
