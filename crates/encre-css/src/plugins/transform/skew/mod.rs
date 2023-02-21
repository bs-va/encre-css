#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

fn skew_can_handle(context: &ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => value.parse::<usize>().map_or(false, |v| v <= 360),
        Modifier::Arbitrary { value, .. } => is_matching_angle(value),
    }
}

fn skew_handle(css_prop: &str, context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => context.buffer.line(format_args!(
            "{}: {}{value}deg;",
            css_prop,
            format_negative(is_negative),
        )),
        Modifier::Arbitrary { value, .. } => {
            context.buffer.line(format_args!("{css_prop}: {value};"));
        }
    }

    context.buffer.line(CSS_TRANSFORM);
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        skew_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        skew_handle("--en-skew-x", context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        skew_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        skew_handle("--en-skew-y", context);
    }
}
