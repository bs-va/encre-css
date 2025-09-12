#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["dramatic", "near", "normal", "midrange", "distant", "none"].contains(value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length" || (hint.is_empty() && is_matching_length(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let value = match *value {
                    "dramatic" => "100px",
                    "near" => "300px",
                    "normal" => "500px",
                    "midrange" => "800px",
                    "distant" => "1200px",
                    "none" => "none",
                    _ => unreachable!(),
                };
                context.buffer.line(format_args!("perspective: {value};"));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("perspective: {value};"));
            }
        }
    }
}
