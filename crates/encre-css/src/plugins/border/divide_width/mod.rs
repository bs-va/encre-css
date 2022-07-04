#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{
        indent,
        value_matchers::{is_matching_length, is_matching_line_width},
    },
};

use std::fmt::{self, Write};

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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "reverse" {
                    return writeln!(context.buffer, "--en-divide-x-reverse: 1;");
                }

                writeln!(context.buffer, "--en-divide-x-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;

                // TODO: class with `> :not([hidden]) ~ :not([hidden])`
                if is_matching_line_width(value) {
                    writeln!(context.buffer, "border-left-width: {value};")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "border-right-width: {value};")?;
                } else {
                    writeln!(
                        context.buffer,
                        "border-left-width: calc({}px * calc(1 - var(--en-divide-x-reverse)));",
                        if value.is_empty() { "1" } else { value }
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "border-right-width: calc({}px * var(--en-divide-x-reverse));",
                        if value.is_empty() { "1" } else { value }
                    )?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-divide-x-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;

                if is_matching_line_width(value) {
                    let value = to_css_value(value);
                    writeln!(context.buffer, "border-left-width: {value};")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "border-right-width: {value};")?;
                } else {
                    let value = to_css_value(value);
                    writeln!(
                        context.buffer,
                        "border-left-width: calc({value} * calc(1 - var(--en-divide-x-reverse)));"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "border-right-width: calc({value} * var(--en-divide-x-reverse));"
                    )?;
                }
            }
        }

        Ok(())
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "reverse" {
                    return writeln!(context.buffer, "--en-divide-y-reverse: 1;");
                }

                writeln!(context.buffer, "--en-divide-y-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;

                // TODO: class with `> :not([hidden]) ~ :not([hidden])`
                if is_matching_line_width(value) {
                    writeln!(context.buffer, "border-top-width: {value};")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "border-bottom-width: {value};")?;
                } else {
                    writeln!(
                        context.buffer,
                        "border-top-width: calc({} * calc(1 - var(--en-divide-y-reverse)));",
                        if value.is_empty() { "1px" } else { value }
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "border-bottom-width: calc({} * var(--en-divide-y-reverse));",
                        if value.is_empty() { "1px" } else { value }
                    )?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-divide-y-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;

                if is_matching_line_width(value) {
                    let value = to_css_value(value);
                    writeln!(context.buffer, "border-top-width: {value};")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "border-bottom-width: {value};")?;
                } else {
                    let value = to_css_value(value);
                    writeln!(
                        context.buffer,
                        "border-top-width: calc({value} * calc(1 - var(--en-divide-y-reverse)));"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "border-bottom-width: calc({value} * var(--en-divide-y-reverse));"
                    )?;
                }
            }
        }

        Ok(())
    }
}
