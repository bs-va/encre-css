#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

use std::borrow::Cow;

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

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                context.buffer.line(format_args!(
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
                ));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("height: {value};"));
            }
        }
    }
}
