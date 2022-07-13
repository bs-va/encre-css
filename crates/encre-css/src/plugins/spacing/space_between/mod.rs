#![doc = include_str!("README.md")]
use crate::{
    generator::{generate_at_rules, generate_class, ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &str {
        "space-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                *value == "reverse" || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
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
                        Modifier::Builtin { is_negative, value } => {
                            if *value == "reverse" {
                                return writeln!(context.buffer, "--en-space-x-reverse: 1;");
                            }

                            let length = spacing::get(value, *is_negative).unwrap();
                            writeln!(context.buffer, "--en-space-x-reverse: 0;")?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                                context.buffer,
                                "margin-right: calc({length} * var(--en-space-x-reverse));"
                            )?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                        context.buffer,
                        "margin-left: calc({length} * calc(1 - var(--en-space-x-reverse)));"
                    )?;
                        }
                        Modifier::Arbitrary { value, .. } => {
                            writeln!(context.buffer, "--en-space-x-reverse: 0;")?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                                context.buffer,
                                "margin-right: calc({value} * var(--en-space-x-reverse));"
                            )?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                                context.buffer,
                                "margin-left: calc({value} * calc(1 - var(--en-space-x-reverse)));"
                            )?;
                        }
                    }

                    Ok(())
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &str {
        "space-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                *value == "reverse" || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
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
                        Modifier::Builtin { is_negative, value } => {
                            if *value == "reverse" {
                                return writeln!(context.buffer, "--en-space-y-reverse: 1;");
                            }

                            let length = spacing::get(value, *is_negative).unwrap();
                            writeln!(context.buffer, "--en-space-y-reverse: 0;")?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                                context.buffer,
                                "margin-top: calc({length} * calc(1 - var(--en-space-y-reverse)));"
                            )?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                                context.buffer,
                                "margin-bottom: calc({length} * var(--en-space-y-reverse));"
                            )?;
                        }
                        Modifier::Arbitrary { value, .. } => {
                            writeln!(context.buffer, "--en-space-y-reverse: 0;")?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                                context.buffer,
                                "margin-top: calc({value} * calc(1 - var(--en-space-y-reverse)));"
                            )?;
                            indent(context.indentation, context.buffer)?;
                            writeln!(
                                context.buffer,
                                "margin-bottom: calc({value} * var(--en-space-y-reverse));"
                            )?;
                        }
                    }

                    Ok(())
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}
