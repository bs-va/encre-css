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
        "items"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["stretch", "start", "center", "end", "baseline"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "stretch" => writeln!(context.buffer, "align-items: stretch;")?,
                "start" => writeln!(context.buffer, "align-items: flex-start;")?,
                "center" => writeln!(context.buffer, "align-items: center;")?,
                "end" => writeln!(context.buffer, "align-items: flex-end;")?,
                "baseline" => writeln!(context.buffer, "align-items: baseline;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
