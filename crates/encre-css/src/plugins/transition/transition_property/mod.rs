#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "transition"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "",
                "none",
                "all",
                "colors",
                "opacity",
                "shadow",
                "transform",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => {
                    writeln!(context.buffer, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "none" => writeln!(context.buffer, "transition-property: none;")?,
                "all" => {
                    writeln!(context.buffer, "transition-property: all;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "colors" => {
                    writeln!(context.buffer, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "opacity" => {
                    writeln!(context.buffer, "transition-property: opacity;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "shadow" => {
                    writeln!(context.buffer, "transition-property: box-shadow;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "transform" => {
                    writeln!(context.buffer, "transition-property: transform;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "transition-property: {};",
                to_css_value(value)
            )?,
        }

        Ok(())
    }
}
