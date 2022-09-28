#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "stretch" => writeln!(buffer, "{indentation}align-items: stretch;")?,
                "start" => writeln!(buffer, "{indentation}align-items: flex-start;")?,
                "center" => writeln!(buffer, "{indentation}align-items: center;")?,
                "end" => writeln!(buffer, "{indentation}align-items: flex-end;")?,
                "baseline" => writeln!(buffer, "{indentation}align-items: baseline;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
