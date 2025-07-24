#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

fn scale_can_handle(context: &ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { value, .. } => {
            is_matching_percentage(value) || is_matching_number(value)
        }
    }
}

fn scale_handle(css_properties: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
                #[allow(clippy::cast_precision_loss)]
                context.buffer.line(format_args!(
                    "{}: {}{};",
                    css_prop,
                    format_negative(is_negative),
                    value.parse::<usize>().unwrap() as f32 / 100.,
                ));
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_properties {
                context
                    .buffer
                    .line(format_args!("{}: {};", css_prop, value));
            }
        }
    }

    context.buffer.line(CSS_TRANSFORM);
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scale_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scale_handle(&["--en-scale-x", "--en-scale-y"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scale_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scale_handle(&["--en-scale-x"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scale_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scale_handle(&["--en-scale-y"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginZDefinition;

impl Plugin for PluginZDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scale_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scale_handle(&["--en-scale-z"], context);
    }
}
