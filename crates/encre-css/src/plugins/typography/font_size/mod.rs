#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{
        indent,
        value_matchers::{
            is_matching_absolute_size, is_matching_length, is_matching_percentage,
            is_matching_relative_size,
        },
    },
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "text"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl",
                "9xl",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "percentage"
                    || *hint == "absolute-size"
                    || *hint == "relative-size"
                    || (hint.is_empty()
                        && (is_matching_length(value)
                            || is_matching_percentage(value)
                            || is_matching_absolute_size(value)
                            || is_matching_relative_size(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "xs" => {
                    writeln!(context.buffer, "font-size: 0.75rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1rem;")?;
                }
                "sm" => {
                    writeln!(context.buffer, "font-size: 0.875rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.25rem;")?;
                }
                "base" => {
                    writeln!(context.buffer, "font-size: 1rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.5rem;")?;
                }
                "lg" => {
                    writeln!(context.buffer, "font-size: 1.125rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.75rem;")?;
                }
                "xl" => {
                    writeln!(context.buffer, "font-size: 1.25rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.75rem;")?;
                }
                "2xl" => {
                    writeln!(context.buffer, "font-size: 1.5rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 2rem;")?;
                }
                "3xl" => {
                    writeln!(context.buffer, "font-size: 1.875rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 2.25rem;")?;
                }
                "4xl" => {
                    writeln!(context.buffer, "font-size: 2.25rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 2.5rem;")?;
                }
                "5xl" => {
                    writeln!(context.buffer, "font-size: 3rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "6xl" => {
                    writeln!(context.buffer, "font-size: 3.75rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "7xl" => {
                    writeln!(context.buffer, "font-size: 4.5rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "8xl" => {
                    writeln!(context.buffer, "font-size: 6rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "9xl" => {
                    writeln!(context.buffer, "font-size: 8rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "font-size: {value};")?;
            }
        }

        Ok(())
    }
}
