#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["full", "min", "max", "fit", "screen"].contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "full" => context.buffer.line("min-height: 100%;"),
                "min" => context.buffer.line("min-height: min-content;"),
                "max" => context.buffer.line("min-height: max-content;"),
                "fit" => context.buffer.line("min-height: fit-content;"),
                "screen" => context.buffer.line("min-height: 100vh;"),
                _ => context.buffer.line(format_args!(
                    "min-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("min-height: {value};"));
            }
        }
    }
}
