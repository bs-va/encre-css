#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
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
                    || value.parse::<isize>().map_or(false, |v| v != 0)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "first" => return writeln!(context.buffer, "order: -9999;"),
                "last" => return writeln!(context.buffer, "order: 9999;"),
                "none" => return writeln!(context.buffer, "order: 0;"),
                _ => writeln!(context.buffer, "order: {};", value)?,
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
