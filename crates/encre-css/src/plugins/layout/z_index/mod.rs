#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value, .. } if value.parse::<usize>().is_ok() || *value == "auto")
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, is_negative } = context.modifier {
            context.buffer.line(format_args!(
                "z-index: {}{value};",
                format_negative(is_negative)
            ));
        }
    }
}
