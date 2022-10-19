#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "leading"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["none", "tight", "snug", "normal", "relaxed", "loose"].contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height#values
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
            Modifier::Builtin { is_negative, value } => match *value {
                "none" => context.buffer.line("line-height: 1;"),
                "tight" => context.buffer.line("line-height: 1.25;"),
                "snug" => context.buffer.line("line-height: 1.375;"),
                "normal" => context.buffer.line("line-height: 1.5;"),
                "relaxed" => context.buffer.line("line-height: 1.625;"),
                "loose" => context.buffer.line("line-height: 2;"),
                _ => context.buffer.line(format_args!(
                    "line-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("line-height: {value};"));
            }
        }
    }
}
