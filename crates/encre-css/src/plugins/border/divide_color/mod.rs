#![doc = include_str!("README.md")]
use crate::{
    generator::{generate_at_rules, generate_class, ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{color, indent, value_matchers::is_matching_color},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "divide"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| {
                    indent(context.indentation, context.buffer)?;
                    match context.modifier {
                        Modifier::Builtin { value, .. } => {
                            let color =
                                color::get(context.config, value, Some("--en-divide-opacity"))
                                    .unwrap();
                            if color.contains("--en-divide-opacity") {
                                writeln!(context.buffer, "--en-divide-opacity: 1;")?;
                                indent(context.indentation, context.buffer)?;
                            }

                            writeln!(context.buffer, "border-color: {color};")?;
                        }
                        Modifier::Arbitrary { value, .. } => {
                            writeln!(context.buffer, "border-color: {value};")?;
                        }
                    }

                    Ok(())
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}
