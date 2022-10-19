#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "min-w"
    }

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
                "full" => context.buffer.line("min-width: 100%;"),
                "min" => context.buffer.line("min-width: min-content;"),
                "max" => context.buffer.line("min-width: max-content;"),
                "fit" => context.buffer.line("min-width: fit-content;"),
                "screen" => context.buffer.line("min-width: 100vw;"),
                _ => context.buffer.line(format_args!(
                    "min-width: {};",
                    spacing::get(value, *is_negative).unwrap()
                )),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("min-width: {value};"));
            }
        }
    }
}
