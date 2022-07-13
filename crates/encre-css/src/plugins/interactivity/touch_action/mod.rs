#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "touch"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "auto",
                "pan-x",
                "pan-left",
                "pan-right",
                "pan-y",
                "pan-up",
                "pan-down",
                "pinch-zoom",
                "manipulation",
                "none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => writeln!(context.buffer, "touch-action: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
