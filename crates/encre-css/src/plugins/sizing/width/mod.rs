#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::{borrow::Cow, fmt::{self, Write}};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "w"
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                if *value == "screen" {
                    return writeln!(context.buffer, "width: 100vw;");
                }

                writeln!(
                    context.buffer,
                    "width: {};",
                    match *value {
                        "auto" => Cow::from("auto"),
                        "full" => Cow::from("100%"),
                        "screen" => Cow::from("100vw"),
                        "min" => Cow::from("min-content"),
                        "max" => Cow::from("max-content"),
                        "fit" => Cow::from("fit-content"),
                        _ => spacing::get(value, *is_negative).unwrap(),
                    },
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "width: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
