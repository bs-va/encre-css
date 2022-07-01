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
        "overscroll"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "contain",
                "x-contain",
                "y-contain",
                "none",
                "x-none",
                "y-none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "overscroll-behavior: auto;")?,
                "x-auto" => writeln!(context.buffer, "overscroll-behavior-x: auto;")?,
                "y-auto" => writeln!(context.buffer, "overscroll-behavior-y: auto;")?,
                "contain" => writeln!(context.buffer, "overscroll-behavior: contain;")?,
                "x-contain" => writeln!(context.buffer, "overscroll-behavior-x: contain;")?,
                "y-contain" => writeln!(context.buffer, "overscroll-behavior-y: contain;")?,
                "none" => writeln!(context.buffer, "overscroll-behavior: none;")?,
                "x-none" => writeln!(context.buffer, "overscroll-behavior-x: none;")?,
                "y-none" => writeln!(context.buffer, "overscroll-behavior-y: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
