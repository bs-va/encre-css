#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_position},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "object"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "bottom",
                "center",
                "left",
                "left-bottom",
                "left-top",
                "right",
                "right-bottom",
                "right-top",
                "top",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_position(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "bottom" => writeln!(context.buffer, "object-position: bottom;")?,
                "center" => writeln!(context.buffer, "object-position: center;")?,
                "left" => writeln!(context.buffer, "object-position: left;")?,
                "left-bottom" => writeln!(context.buffer, "object-position: left bottom;")?,
                "left-top" => writeln!(context.buffer, "object-position: left top;")?,
                "right" => writeln!(context.buffer, "object-position: right;")?,
                "right-bottom" => writeln!(context.buffer, "object-position: right bottom;")?,
                "right-top" => writeln!(context.buffer, "object-position: right top;")?,
                "top" => writeln!(context.buffer, "object-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "object-position: {value};")?;
            }
        }

        Ok(())
    }
}
