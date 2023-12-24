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
                    || ["full", "screen", "min", "max", "fit", "auto", "svh", "lvh", "dvh"].contains(value)
            }
            Modifier::Arbitrary { value, .. } => {
                is_matching_length(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                context.buffer.line(format_args!(
                    "height: {};",
                    match *value {
                        "auto" => Cow::Borrowed("auto"),
                        "full" => Cow::Borrowed("100%"),
                        "screen" => Cow::Borrowed("100vh"),
                        "min" => Cow::Borrowed("min-content"),
                        "max" => Cow::Borrowed("max-content"),
                        "fit" => Cow::Borrowed("fit-content"),
                        "svh" => Cow::Borrowed("100svh"),
                        "lvh" => Cow::Borrowed("100lvh"),
                        "dvh" => Cow::Borrowed("100dvh"),
                        _ => spacing::get(value, *is_negative).unwrap(),
                    },
                ));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("height: {value};"));
            }
        }
    }
}
