#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

fn divide_width_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            value.is_empty() || *value == "reverse" || value.parse::<usize>().is_ok()
        }
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "length"
                || *hint == "line-width"
                || (hint.is_empty() && (is_matching_length(value) || is_matching_line_width(value)))
        }
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &str {
        "divide-x"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        divide_width_can_handle(&mut context)
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |ContextHandle { modifier, indentation, buffer, .. }| {
                    match modifier {
                        Modifier::Builtin { value, .. } => {
                            if *value == "reverse" {
                                return writeln!(buffer, "{indentation}--en-divide-x-reverse: 1;");
                            }

                            writeln!(buffer, "{indentation}--en-divide-x-reverse: 0;")?;

                            if is_matching_line_width(value) {
                                writeln!(buffer, "{indentation}border-right-width: {value};\n{indentation}border-left-width: {value};")?;
                            } else {
                                writeln!(
                                    buffer,
                                    "{indentation}border-right-width: calc({value}px * var(--en-divide-x-reverse));\n{indentation}border-left-width: calc({value}px * calc(1 - var(--en-divide-x-reverse)));",
                                    value = if value.is_empty() { "1" } else { value }
                                )?;
                            }
                        }
                        Modifier::Arbitrary { value, .. } => {
                            writeln!(buffer, "{indentation}--en-divide-x-reverse: 0;")?;

                            if is_matching_line_width(value) {
                                writeln!(buffer, "{indentation}border-right-width: {value};\n{indentation}border-left-width: {value};")?;
                            } else {
                                writeln!(
                            buffer,
                            "{indentation}border-right-width: calc({value} * var(--en-divide-x-reverse));\n{indentation}border-left-width: calc({value} * calc(1 - var(--en-divide-x-reverse)));"
                        )?;
                            }
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
        "divide-y"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        divide_width_can_handle(&mut context)
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |ContextHandle { modifier, indentation, buffer, .. }| {
                    match modifier {
                        Modifier::Builtin { value, .. } => {
                            if *value == "reverse" {
                                return writeln!(buffer, "{indentation}--en-divide-y-reverse: 1;");
                            }

                            writeln!(buffer, "{indentation}--en-divide-y-reverse: 0;")?;

                            if is_matching_line_width(value) {
                                writeln!(buffer, "{indentation}border-top-width: {value};\n{indentation}border-bottom-width: {value};")?;
                            } else {
                                writeln!(
                            buffer,
                            "{indentation}border-top-width: calc({value}px * calc(1 - var(--en-divide-y-reverse)));\n{indentation}border-bottom-width: calc({value}px * var(--en-divide-y-reverse));",
                            value = if value.is_empty() { "1" } else { value }
                        )?;
                            }
                        }
                        Modifier::Arbitrary { value, .. } => {
                            writeln!(buffer, "{indentation}--en-divide-y-reverse: 0;")?;

                            if is_matching_line_width(value) {
                                writeln!(buffer, "{indentation}border-top-width: {value};\n{indentation}border-bottom-width: {value};")?;
                            } else {
                                writeln!(
                            buffer,
                            "{indentation}border-top-width: calc({value} * calc(1 - var(--en-divide-y-reverse)));\n{indentation}border-bottom-width: calc({value} * var(--en-divide-y-reverse));"
                        )?;
                            }
                        }
                    }

                    Ok(())
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}
