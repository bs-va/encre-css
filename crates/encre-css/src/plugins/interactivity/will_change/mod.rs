#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "will-change"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "scroll", "contents", "transform"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "will-change: auto;")?,
                "scroll" => writeln!(context.buffer, "will-change: scroll-position;")?,
                "contents" => writeln!(context.buffer, "will-change: contents;")?,
                "transform" => writeln!(context.buffer, "will-change: transfrom;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(context.buffer, "will-change: {value};")?,
        }

        Ok(())
    }
}
