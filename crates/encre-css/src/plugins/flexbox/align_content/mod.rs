#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "content"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["start", "center", "end", "between", "around", "evenly"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "start" => writeln!(context.buffer, "align-content: flex-start;")?,
                "center" => writeln!(context.buffer, "align-content: center;")?,
                "end" => writeln!(context.buffer, "align-content: flex-end;")?,
                "between" => writeln!(context.buffer, "align-content: space-between;")?,
                "around" => writeln!(context.buffer, "align-content: space-around;")?,
                "evenly" => writeln!(context.buffer, "align-content: space-evenly;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
