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
        "overflow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "hidden",
                "x-hidden",
                "y-hidden",
                "visible",
                "x-visible",
                "y-visible",
                "scroll",
                "x-scroll",
                "y-scroll",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "overflow: auto;")?,
                "x-auto" => writeln!(context.buffer, "overflow-x: auto;")?,
                "y-auto" => writeln!(context.buffer, "overflow-y: auto;")?,
                "hidden" => writeln!(context.buffer, "overflow: hidden;")?,
                "x-hidden" => writeln!(context.buffer, "overflow-x: hidden;")?,
                "y-hidden" => writeln!(context.buffer, "overflow-y: hidden;")?,
                "visible" => writeln!(context.buffer, "overflow: visible;")?,
                "x-visible" => writeln!(context.buffer, "overflow-x: visible;")?,
                "y-visible" => writeln!(context.buffer, "overflow-y: visible;")?,
                "scroll" => writeln!(context.buffer, "overflow: scroll;")?,
                "x-scroll" => writeln!(context.buffer, "overflow-x: scroll;")?,
                "y-scroll" => writeln!(context.buffer, "overflow-y: scroll;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
