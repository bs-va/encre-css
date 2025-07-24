#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["2xs", "xs", "sm", "md", "lg", "xl", "2xl", "inner", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "shadow" || (hint.is_empty() && is_matching_shadow(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "2xs" => {
                    context.buffer.lines([
                        "text-shadow: 0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.15));",
                    ]);
                }
                "xs" => {
                    context.buffer.lines([
                        "text-shadow: 0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.2));",
                    ]);
                }
                "sm" => {
                    context.buffer.lines([
                        "text-shadow: 0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 2px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.075));",
                    ]);
                }
                "md" => {
                    context.buffer.lines([
                        "text-shadow: 0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 2px 4px var(--en-text-shadow-color, rgb(0 0 0 / 0.1));",
                    ]);
                }
                "lg" => {
                    context.buffer.lines([
                        "text-shadow: 0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 3px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 4px 8px var(--en-text-shadow-color, rgb(0 0 0 / 0.1));",
                    ]);
                }
                "none" => return context.buffer.line("text-shadow: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                let mut shadow = shadow::ShadowList::parse(value).unwrap();
                shadow.replace_all_colors("var(--en-text-shadow-color, {})");
                context
                    .buffer
                    .line(format_args!("text-shadow: {shadow};"));
            }
        }
    }
}
