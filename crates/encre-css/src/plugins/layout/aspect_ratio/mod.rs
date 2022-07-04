#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "aspect"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["auto", "square", "video"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "aspect-ratio: auto;")?,
                "square" => writeln!(context.buffer, "aspect-ratio: 1 / 1;")?,
                "video" => writeln!(context.buffer, "aspect-ratio: 16 / 9;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "aspect-ratio: {};",
                to_css_value(&value.replace('/', " / "))
            )?,
        }

        Ok(())
    }
}
