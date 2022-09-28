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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "start" => writeln!(buffer, "{indentation}align-content: flex-start;")?,
                "center" => writeln!(buffer, "{indentation}align-content: center;")?,
                "end" => writeln!(buffer, "{indentation}align-content: flex-end;")?,
                "between" => writeln!(buffer, "{indentation}align-content: space-between;")?,
                "around" => writeln!(buffer, "{indentation}align-content: space-around;")?,
                "evenly" => writeln!(buffer, "{indentation}align-content: space-evenly;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
