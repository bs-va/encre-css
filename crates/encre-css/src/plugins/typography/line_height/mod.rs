#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

use std::borrow::Cow;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["none", "tight", "snug", "normal", "relaxed", "loose"].contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => {
                *value == "normal"
                    || is_matching_number(value)
                    || is_matching_length(value)
                    || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => context.buffer.line(format_args!(
                "line-height: {};",
                match *value {
                    "none" => Cow::Borrowed("1"),
                    "tight" => Cow::Borrowed("1.25"),
                    "snug" => Cow::Borrowed("1.375"),
                    "normal" => Cow::Borrowed("1.5"),
                    "relaxed" => Cow::Borrowed("1.625"),
                    "loose" => Cow::Borrowed("2"),
                    _ => spacing::get(value, *is_negative).unwrap(),
                }
            )),
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("line-height: {value};"));
            }
        }
    }
}
