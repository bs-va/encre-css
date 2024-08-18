#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

use std::borrow::Cow;

fn placement_can_handle(context: &ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            spacing::is_matching_builtin_spacing(value) || *value == "auto" || *value == "full"
        }
        Modifier::Arbitrary { value, prefix, .. } => {
            prefix.is_empty()
                && (is_matching_length(value) || is_matching_percentage(value) || *value == "auto")
        }
    }
}

fn placement_handle(css_properties: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
                context.buffer.line(format_args!(
                    "{}: {};",
                    css_prop,
                    if *value == "auto" {
                        Cow::from("auto")
                    } else if *value == "full" && *is_negative {
                        Cow::from("-100%")
                    } else if *value == "full" {
                        Cow::from("100%")
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
pub(crate) struct PluginInsetDefinition;

impl Plugin for PluginInsetDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["top", "right", "bottom", "left"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginInsetXDefinition;

impl Plugin for PluginInsetXDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["left", "right"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginInsetYDefinition;

impl Plugin for PluginInsetYDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["top", "bottom"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginStartDefinition;

impl Plugin for PluginStartDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["inset-inline-start"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginEndDefinition;

impl Plugin for PluginEndDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["inset-inline-end"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["top"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["bottom"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["left"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        placement_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        placement_handle(&["right"], context);
    }
}
