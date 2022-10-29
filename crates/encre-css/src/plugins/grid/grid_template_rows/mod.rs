#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok() || *value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "none" {
                    context.buffer.line("grid-template-rows: none;");
                    return;
                }

                context.buffer.line(format_args!(
                    "grid-template-rows: repeat({}, minmax(0, 1fr));",
                    value.parse::<usize>().unwrap(),
                ));
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("grid-template-rows: {value};"));
            }
        }
    }
}
