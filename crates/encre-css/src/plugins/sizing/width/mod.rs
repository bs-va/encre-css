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
                    || ["full", "screen", "min", "max", "fit", "auto", "svw", "lvw", "dvw"].contains(value)
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
                    "width: {};",
                    match *value {
                        "auto" => Cow::Borrowed("auto"),
                        "full" => Cow::Borrowed("100%"),
                        "screen" => Cow::Borrowed("100vw"),
                        "min" => Cow::Borrowed("min-content"),
                        "max" => Cow::Borrowed("max-content"),
                        "fit" => Cow::Borrowed("fit-content"),
                        "svw" => Cow::Borrowed("100svw"),
                        "lvw" => Cow::Borrowed("100lvw"),
                        "dvw" => Cow::Borrowed("100dvw"),
                        _ => spacing::get(value, *is_negative).unwrap(),
                    },
                ));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("width: {value};"));
            }
        }
    }
}
