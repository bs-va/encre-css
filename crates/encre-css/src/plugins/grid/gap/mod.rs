#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "gap"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty() && is_matching_length(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "gap: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &str {
        "gap-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "column-gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "column-gap: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &str {
        "gap-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "row-gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "row-gap: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
