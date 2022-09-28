#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
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
        "snap"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["start", "end", "center", "align-none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "start" => writeln!(buffer, "{indentation}scroll-snap-align: start;")?,
                "end" => writeln!(buffer, "{indentation}scroll-snap-align: end;")?,
                "center" => writeln!(buffer, "{indentation}scroll-snap-align: center;")?,
                "align-none" => writeln!(buffer, "{indentation}scroll-snap-align: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
