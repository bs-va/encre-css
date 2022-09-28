#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::{
        is_matching_absolute_size, is_matching_length, is_matching_percentage,
        is_matching_relative_size,
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "xs" => {
                    writeln!(buffer, "{indentation}font-size: 0.75rem;\n{indentation}line-height: 1rem;")?;
                }
                "sm" => {
                    writeln!(buffer, "{indentation}font-size: 0.875rem;\n{indentation}line-height: 1.25rem;")?;
                }
                "base" => {
                    writeln!(buffer, "{indentation}font-size: 1rem;\n{indentation}line-height: 1.5rem;")?;
                }
                "lg" => {
                    writeln!(buffer, "{indentation}font-size: 1.125rem;\n{indentation}line-height: 1.75rem;")?;
                }
                "xl" => {
                    writeln!(buffer, "{indentation}font-size: 1.25rem;\n{indentation}line-height: 1.75rem;")?;
                }
                "2xl" => {
                    writeln!(buffer, "{indentation}font-size: 1.5rem;\n{indentation}line-height: 2rem;")?;
                }
                "3xl" => {
                    writeln!(buffer, "{indentation}font-size: 1.875rem;\n{indentation}line-height: 2.25rem;")?;
                }
                "4xl" => {
                    writeln!(buffer, "{indentation}font-size: 2.25rem;\n{indentation}line-height: 2.5rem;")?;
                }
                "5xl" => {
                    writeln!(buffer, "{indentation}font-size: 3rem;\n{indentation}line-height: 1;")?;
                }
                "6xl" => {
                    writeln!(buffer, "{indentation}font-size: 3.75rem;\n{indentation}line-height: 1;")?;
                }
                "7xl" => {
                    writeln!(buffer, "{indentation}font-size: 4.5rem;\n{indentation}line-height: 1;")?;
                }
                "8xl" => {
                    writeln!(buffer, "{indentation}font-size: 6rem;\n{indentation}line-height: 1;")?;
                }
                "9xl" => {
                    writeln!(buffer, "{indentation}font-size: 8rem;\n{indentation}line-height: 1;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}font-size: {value};")?;
            }
        }

        Ok(())
    }
}
