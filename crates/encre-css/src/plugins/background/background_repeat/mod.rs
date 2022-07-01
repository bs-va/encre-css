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
        "bg"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "repeat",
                "no-repeat",
                "repeat-x",
                "repeat-y",
                "repeat-round",
                "repeat-space",
            ]
            .contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "repeat" => writeln!(context.buffer, "background-repeat: repeat;")?,
                "no-repeat" => writeln!(context.buffer, "background-repeat: no-repeat;")?,
                "repeat-x" => writeln!(context.buffer, "background-repeat: repeat-x;")?,
                "repeat-y" => writeln!(context.buffer, "background-repeat: repeat-y;")?,
                "repeat-round" => writeln!(context.buffer, "background-repeat: round;")?,
                "repeat-space" => writeln!(context.buffer, "background-repeat: space;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
