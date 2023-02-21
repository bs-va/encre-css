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
                    || [
                        "xs",
                        "sm",
                        "md",
                        "lg",
                        "xl",
                        "2xl",
                        "3xl",
                        "4xl",
                        "5xl",
                        "6xl",
                        "7xl",
                        "full",
                        "min",
                        "max",
                        "fit",
                        "prose",
                        "screen",
                        "screen-sm",
                        "screen-md",
                        "screen-lg",
                        "screen-lg",
                        "screen-xl",
                        "screen-2xl",
                        "none",
                    ]
                    .contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => {
                is_matching_length(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => context.buffer.line(format_args!(
                "max-width: {};",
                match *value {
                    "none" => Cow::Borrowed("none"),
                    "xs" => Cow::Borrowed("20rem"),
                    "sm" => Cow::Borrowed("24rem"),
                    "md" => Cow::Borrowed("28rem"),
                    "lg" => Cow::Borrowed("32rem"),
                    "xl" => Cow::Borrowed("36rem"),
                    "2xl" => Cow::Borrowed("42rem"),
                    "3xl" => Cow::Borrowed("48rem"),
                    "4xl" => Cow::Borrowed("56rem"),
                    "5xl" => Cow::Borrowed("64rem"),
                    "6xl" => Cow::Borrowed("72rem"),
                    "7xl" => Cow::Borrowed("80rem"),
                    "full" => Cow::Borrowed("100%"),
                    "min" => Cow::Borrowed("min-content"),
                    "max" => Cow::Borrowed("max-content"),
                    "fit" => Cow::Borrowed("fit-content"),
                    "prose" => Cow::Borrowed("65ch"),
                    "screen" => Cow::Borrowed("100vw"),
                    "screen-sm" => Cow::Borrowed("640px"),
                    "screen-md" => Cow::Borrowed("768px"),
                    "screen-lg" => Cow::Borrowed("1024px"),
                    "screen-xl" => Cow::Borrowed("1280px"),
                    "screen-2xl" => Cow::Borrowed("1536px"),
                    _ => spacing::get(value, *is_negative).unwrap(),
                }
            )),
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("max-width: {value};"));
            }
        }
    }
}
