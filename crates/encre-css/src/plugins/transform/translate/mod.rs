#![doc = include_str!("README.md")]
use super::CSS_TRANSFORM;
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

fn translate_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            spacing::is_matching_builtin_spacing(value) || *value == "auto" || *value == "full"
        }
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

fn translate_handle(css_prop: &str, context: &mut ContextHandle) -> fmt::Result {
    indent(context.indentation, context.buffer)?;
    match context.modifier {
        Modifier::Builtin { is_negative, value } => writeln!(
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
        "translate-x"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        translate_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        translate_handle("--en-translate-x", &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &'static str {
        "translate-y"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        translate_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        translate_handle("--en-translate-y", &mut context)
    }
}
