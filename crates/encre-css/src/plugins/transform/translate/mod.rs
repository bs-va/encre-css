#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

use std::borrow::Cow;

fn translate_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            spacing::is_matching_builtin_spacing(value) || *value == "auto" || *value == "full"
        }
        Modifier::Arbitrary { value, .. } => {
            is_matching_length(value) || is_matching_percentage(value)
        }
    }
}

fn translate_handle(css_prop: &str, context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => context.buffer.line(format_args!(
            "{}: {};",
            css_prop,
            if *value == "auto" {
                Cow::from("auto")
            } else if *value == "full" {
                Cow::from("100%")
            } else {
                spacing::get(value, *is_negative).unwrap()
            },
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
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        translate_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        translate_handle("--en-translate-x", context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        translate_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        translate_handle("--en-translate-y", context);
    }
}
