#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

use std::borrow::Cow;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                spacing::is_matching_builtin_spacing(value)
                    || ["none", "full", "screen", "min", "max", "fit", "auto"].contains(value)
            }
            Modifier::Arbitrary { value, .. } => {
                is_matching_length(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => context.buffer.line(format_args!(
                "max-height: {};",
                match *value {
                    "none" => Cow::Borrowed("none"),
                    "auto" => Cow::Borrowed("auto"),
                    "full" => Cow::Borrowed("100%"),
                    "screen" => Cow::Borrowed("100vh"),
                    "min" => Cow::Borrowed("min-content"),
                    "max" => Cow::Borrowed("max-content"),
                    "fit" => Cow::Borrowed("fit-content"),
                    _ => spacing::get(value, *is_negative).unwrap(),
                }
            )),
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("max-height: {value};"));
            }
        }
    }
}
