#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{
        indent, spacing,
        value_matchers::{is_matching_length, is_matching_percentage},
    },
};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

fn placement_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            spacing::is_matching_builtin_spacing(value) || *value == "auto" || *value == "full"
        }
        Modifier::Arbitrary { value, .. } => {
            is_matching_length(value) || is_matching_percentage(value) || *value == "auto"
        }
    }
}

fn placement_handle(css_properties: &[&str], context: &mut ContextHandle) -> fmt::Result {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "{}: {};",
                    css_prop,
                    if *value == "auto" {
                        Cow::from("auto")
                    } else if *value == "full" {
                        Cow::from("100%")
                    } else {
                        spacing::get(value, *is_negative).unwrap()
                    },
                )?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            let value = to_css_value(value);
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "{}: {};", css_prop, value)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub(crate) struct PluginInsetDefinition;

impl Plugin for PluginInsetDefinition {
    fn namespace(&self) -> &str {
        "inset"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                spacing::is_matching_builtin_spacing(value) || *value == "auto" || *value == "full"
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty()
                    && (is_matching_length(value)
                        || is_matching_percentage(value)
                        || *value == "auto")
            }
        }
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        placement_handle(&["top", "bottom", "left", "right"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginInsetXDefinition;

impl Plugin for PluginInsetXDefinition {
    fn namespace(&self) -> &str {
        "inset-x"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        placement_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        placement_handle(&["left", "right"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginInsetYDefinition;

impl Plugin for PluginInsetYDefinition {
    fn namespace(&self) -> &str {
        "inset-y"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        placement_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        placement_handle(&["top", "bottom"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn namespace(&self) -> &str {
        "top"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        placement_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        placement_handle(&["top"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn namespace(&self) -> &str {
        "bottom"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        placement_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        placement_handle(&["bottom"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn namespace(&self) -> &str {
        "left"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        placement_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        placement_handle(&["left"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn namespace(&self) -> &str {
        "right"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        placement_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        placement_handle(&["right"], &mut context)
    }
}
