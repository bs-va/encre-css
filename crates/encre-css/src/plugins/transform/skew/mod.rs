#![doc = include_str!("README.md")]
use super::CSS_TRANSFORM;
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{format_negative, indent, value_matchers::is_matching_angle},
};

use std::fmt::{self, Write};

fn skew_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => value.parse::<usize>().map_or(false, |v| v <= 360),
        Modifier::Arbitrary { value, .. } => is_matching_angle(value),
    }
}

fn skew_handle(css_prop: &str, context: &mut ContextHandle) -> fmt::Result {
    indent(context.indentation, context.buffer)?;
    match context.modifier {
        Modifier::Builtin { is_negative, value } => writeln!(
            context.buffer,
            "{}: {}{value}deg;",
            css_prop,
            format_negative(is_negative),
        )?,
        Modifier::Arbitrary { value, .. } => {
            writeln!(context.buffer, "{}: {};", css_prop, to_css_value(value))?;
        }
    }

    indent(context.indentation, context.buffer)?;
    writeln!(context.buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &'static str {
        "skew-x"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        skew_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        skew_handle("--en-skew-x", &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &'static str {
        "skew-y"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        skew_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        skew_handle("--en-skew-y", &mut context)
    }
}
