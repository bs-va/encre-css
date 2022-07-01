#![doc = include_str!("README.md")]
use crate::{
    plugins::Plugin,
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "self"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "start", "center", "end", "stretch"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "align-self: auto;")?,
                "start" => writeln!(context.buffer, "align-self: flex-start;")?,
                "center" => writeln!(context.buffer, "align-self: center;")?,
                "end" => writeln!(context.buffer, "align-self: flex-end;")?,
                "stretch" => writeln!(context.buffer, "align-self: stretch;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
