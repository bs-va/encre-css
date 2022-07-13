#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "gap: {value};")?;
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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "column-gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "column-gap: {value};")?;
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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                context.buffer,
                "row-gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "row-gap: {value};")?;
            }
        }

        Ok(())
    }
}
