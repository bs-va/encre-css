#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

fn scale_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { .. } => false,
    }
}

fn scale_handle(css_properties: &[&str], context: &mut ContextHandle) {
    if let Modifier::Builtin { is_negative, value } = context.modifier {
        for css_prop in css_properties {
            #[allow(clippy::cast_precision_loss)]
            context.buffer.line(format_args!(
                "{}: {}{};",
                css_prop,
                format_negative(is_negative),
                value.parse::<usize>().unwrap() as f32 / 100.,
            ));
        }

        context.buffer.line(CSS_TRANSFORM);
    }
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scale_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scale_handle(&["--en-scale-x", "--en-scale-y"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scale_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scale_handle(&["--en-scale-x"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scale_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scale_handle(&["--en-scale-y"], context);
    }
}
