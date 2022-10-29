#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                context.buffer.line(format_args!(
                    "text-indent: {};",
                    spacing::get(value, *is_negative).unwrap(),
                ));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("text-indent: {value};"));
            }
        }
    }
}
