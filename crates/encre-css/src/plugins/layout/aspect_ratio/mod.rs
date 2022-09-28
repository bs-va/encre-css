#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::is_matching_all,
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
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(buffer, "{indentation}aspect-ratio: auto;")?,
                "square" => writeln!(buffer, "{indentation}aspect-ratio: 1 / 1;")?,
                "video" => writeln!(buffer, "{indentation}aspect-ratio: 16 / 9;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                buffer,
                "{indentation}aspect-ratio: {};",
                value.replace('/', " / "),
            )?,
        }

        Ok(())
    }
}
