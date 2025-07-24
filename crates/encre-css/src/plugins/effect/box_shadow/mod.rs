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
                        "--en-shadow: 0 1px var(--en-shadow-color, rgb(0 0 0 / 0.05));",
                    ]);
                }
                "xs" => {
                    context.buffer.lines([
                        "--en-shadow: 0 1px 2px 0 var(--en-shadow-color, rgb(0 0 0 / 0.05));",
                    ]);
                }
                "sm" => {
                    context.buffer.lines([
                        "--en-shadow: 0 1px 3px 0 var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 1px 2px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
                    ]);
                }
                "md" => {
                    context.buffer.lines([
                        "--en-shadow: 0 4px 6px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 2px 4px -2px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
                    ]);
                }
                "lg" => {
                    context.buffer.lines([
                        "--en-shadow: 0 10px 15px -3px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 4px 6px -4px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
                    ]);
                }
                "xl" => {
                    context.buffer.lines([
                        "--en-shadow: 0 20px 25px -5px var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 8px 10px -6px var(--en-shadow-color, rgb(0 0 0 / 0.1));",
                    ]);
                }
                "2xl" => {
                    context.buffer.lines([
                        "--en-shadow: 0 25px 50px -12px var(--en-shadow-color, rgb(0 0 0 / 0.25));",
                    ]);
                }
                "inner" => {
                    context.buffer.lines([
                        "--en-shadow: inset 0 2px 4px 0 var(--en-shadow-color, rgb(0 0 0 / 0.05));",
                    ]);
                }
                "none" => return context.buffer.line("box-shadow: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                let mut shadow = shadow::ShadowList::parse(value).unwrap();
                shadow.replace_all_colors("var(--en-shadow-color, {})");
                context
                    .buffer
                    .line(format_args!("--en-shadow: {shadow};"));
            }
        }

        context.buffer.line("box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);");
    }
}
