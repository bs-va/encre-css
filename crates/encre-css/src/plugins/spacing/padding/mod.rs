#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

use std::borrow::Cow;

fn padding_can_handle(context: &ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            *value == "auto" || spacing::is_matching_builtin_spacing(value)
        }
        Modifier::Arbitrary { value, prefix, .. } => {
            prefix.is_empty() && (is_matching_length(value) || is_matching_percentage(value))
        }
    }
}

fn padding_handle(css_properties: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
                context.buffer.line(format_args!(
                    "{}: {};",
                    css_prop,
                    if *value == "auto" {
                        Cow::from("auto")
                    } else {
                        spacing::get(value, *is_negative).unwrap()
                    },
                ));
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_properties {
                context.buffer.line(format_args!("{css_prop}: {value};"));
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-left", "padding-right"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-top", "padding-bottom"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginStartDefinition;

impl Plugin for PluginStartDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-inline-start"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginEndDefinition;

impl Plugin for PluginEndDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-inline-end"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-top"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-bottom"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-left"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        padding_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        padding_handle(&["padding-right"], context);
    }
}
