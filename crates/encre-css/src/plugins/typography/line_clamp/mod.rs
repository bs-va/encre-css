#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "line-clamp"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "none" || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "none" {
                    context.buffer.line("-webkit-line-clamp: unset;");
                } else {
                    context.buffer.lines([
                        format_args!("overflow: hidden;"),
                        format_args!("display: -webkit-box;"),
                        format_args!("-webkit-box-orient: vertical;"),
                        format_args!("-webkit-line-clamp: {value};"),
                    ]);
                }
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
