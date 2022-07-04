#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

fn padding_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            *value == "auto" || spacing::is_matching_builtin_spacing(value)
        }
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

fn padding_handle(css_properties: &[&str], context: &mut ContextHandle) -> fmt::Result {
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
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "p"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                *value == "auto" || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty() && is_matching_length(value)
            }
        }
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        padding_handle(&["padding"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &str {
        "px"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        padding_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        padding_handle(&["padding-left", "padding-right"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &str {
        "py"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        padding_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        padding_handle(&["padding-top", "padding-bottom"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn namespace(&self) -> &str {
        "pt"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        padding_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        padding_handle(&["padding-top"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn namespace(&self) -> &str {
        "pb"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        padding_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        padding_handle(&["padding-bottom"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn namespace(&self) -> &str {
        "pl"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        padding_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        padding_handle(&["padding-left"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn namespace(&self) -> &str {
        "pr"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        padding_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        padding_handle(&["padding-right"], &mut context)
    }
}
