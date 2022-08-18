#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, shadow::ShadowList, value_matchers::is_matching_shadow},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "shadow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "inner", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "shadow" || (hint.is_empty() && is_matching_shadow(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => {
                    writeln!(context.buffer, "--en-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 1px 3px 0 var(--en-shadow-color), 0 1px 2px -1px var(--en-shadow-color);")?;
                }
                "sm" => {
                    writeln!(
                        context.buffer,
                        "--en-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "--en-shadow-colored: 0 1px 2px 0 var(--en-shadow-color);"
                    )?;
                }
                "md" => {
                    writeln!(context.buffer, "--en-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 4px 6px -1px var(--en-shadow-color), 0 2px 4px -2px var(--en-shadow-color);")?;
                }
                "lg" => {
                    writeln!(context.buffer, "--en-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 10px 15px -3px var(--en-shadow-color), 0 4px 6px -4px var(--en-shadow-color);")?;
                }
                "xl" => {
                    writeln!(context.buffer, "--en-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 20px 25px -5px var(--en-shadow-color), 0 8px 10px -6px var(--en-shadow-color);")?;
                }
                "2xl" => {
                    writeln!(
                        context.buffer,
                        "--en-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "--en-shadow-colored: 0 25px 50px -12px var(--en-shadow-color);"
                    )?;
                }
                "inner" => {
                    writeln!(
                        context.buffer,
                        "--en-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "--en-shadow-colored: inset 0 2px 4px 0 var(--en-shadow-color);"
                    )?;
                }
                "none" => writeln!(context.buffer, "box-shadow: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-shadow: {value};")?;
                indent(context.indentation, context.buffer)?;
                let mut shadow = ShadowList::parse(value).unwrap();
                shadow.replace_all_colors("var(--en-shadow-color)");
                writeln!(context.buffer, "--en-shadow-colored: {};", shadow)?;
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);")?;

        Ok(())
    }
}
