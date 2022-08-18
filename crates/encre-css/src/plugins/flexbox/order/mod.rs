#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{format_negative, indent},
};

use std::fmt::{self, Write};

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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin {
                is_negative, value, ..
            } => match *value {
                "first" => return writeln!(context.buffer, "order: -9999;"),
                "last" => return writeln!(context.buffer, "order: 9999;"),
                "none" => return writeln!(context.buffer, "order: 0;"),
                _ => writeln!(
                    context.buffer,
                    "order: {}{value};",
                    format_negative(is_negative)
                )?,
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
