#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::is_matching_length,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "tracking"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["tighter", "tight", "normal", "wide", "wider", "widest"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => *value == "normal" || is_matching_length(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "tighter" => writeln!(buffer, "{indentation}letter-spacing: -0.05em;")?,
                "tight" => writeln!(buffer, "{indentation}letter-spacing: -0.025em;")?,
                "normal" => writeln!(buffer, "{indentation}letter-spacing: 0;")?,
                "wide" => writeln!(buffer, "{indentation}letter-spacing: 0.025em;")?,
                "wider" => writeln!(buffer, "{indentation}letter-spacing: 0.05em;")?,
                "widest" => writeln!(buffer, "{indentation}letter-spacing: 0.1em;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}letter-spacing: {value};")?;
            }
        }

        Ok(())
    }
}
