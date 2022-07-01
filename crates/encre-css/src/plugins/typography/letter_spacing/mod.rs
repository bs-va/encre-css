#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_length},
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "tighter" => writeln!(context.buffer, "letter-spacing: -0.05em;")?,
                "tight" => writeln!(context.buffer, "letter-spacing: -0.025em;")?,
                "normal" => writeln!(context.buffer, "letter-spacing: 0;")?,
                "wide" => writeln!(context.buffer, "letter-spacing: 0.025em;")?,
                "wider" => writeln!(context.buffer, "letter-spacing: 0.05em;")?,
                "widest" => writeln!(context.buffer, "letter-spacing: 0.1em;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "letter-spacing: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
