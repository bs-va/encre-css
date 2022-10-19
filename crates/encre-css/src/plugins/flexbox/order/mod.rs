#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "order"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["first", "last", "none"].contains(&&**value)
                    || value.parse::<usize>().map_or(false, |v| v != 0)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin {
                is_negative, value, ..
            } => match *value {
                "first" => return context.buffer.line("order: -9999;"),
                "last" => return context.buffer.line("order: 9999;"),
                "none" => return context.buffer.line("order: 0;"),
                _ => context.buffer.line(format_args!(
                    "order: {}{value};",
                    format_negative(is_negative)
                )),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
