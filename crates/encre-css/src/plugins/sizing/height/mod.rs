#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "h"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                spacing::is_matching_builtin_spacing(value)
                    || ["full", "screen", "min", "max", "fit", "auto"].contains(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                writeln!(
                    context.buffer,
                    "height: {};",
                    match *value {
                        "auto" => Cow::from("auto"),
                        "full" => Cow::from("100%"),
                        "screen" => Cow::from("100vh"),
                        "min" => Cow::from("min-content"),
                        "max" => Cow::from("max-content"),
                        "fit" => Cow::from("fit-content"),
                        _ => spacing::get(value, *is_negative).unwrap(),
                    },
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "height: {value};")?;
            }
        }

        Ok(())
    }
}
