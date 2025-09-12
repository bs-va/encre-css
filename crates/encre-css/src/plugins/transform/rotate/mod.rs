#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

fn rotate_can_handle(context: &ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { value, .. } => is_matching_angle(value),
    }
}

fn rotate_handle(css_properties: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
                context.buffer.line(format_args!(
                    "{}: {}{}deg;",
                    css_prop,
                    format_negative(is_negative),
                    value.parse::<usize>().unwrap(),
                ));
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_properties {
                context
                    .buffer
                    .line(format_args!("{css_prop}: {value};"));
            }
        }
    }

    context.buffer.line(CSS_TRANSFORM);
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        rotate_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        rotate_handle(&["--en-rotate-x", "--en-rotate-y"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        rotate_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        rotate_handle(&["--en-rotate-x"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        rotate_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        rotate_handle(&["--en-rotate-y"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginZDefinition;

impl Plugin for PluginZDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        rotate_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        rotate_handle(&["--en-rotate-z"], context);
    }
}
