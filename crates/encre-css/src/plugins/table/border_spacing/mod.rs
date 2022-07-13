#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

fn border_spacing_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

fn border_spacing_handle(css_props: &[&str], context: &mut ContextHandle) -> fmt::Result {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_props {
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "{}: {};",
                    css_prop,
                    spacing::get(value, *is_negative).unwrap(),
                )?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_props {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "{css_prop}: {value}")?;
            }
        }
    }

    indent(context.indentation, context.buffer)?;
    writeln!(
        context.buffer,
        "border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);"
    )?;
    Ok(())
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "border-spacing"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        border_spacing_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-x", "--en-border-spacing-y"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &'static str {
        "border-spacing-x"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        border_spacing_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-x"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &'static str {
        "border-spacing-y"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        border_spacing_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-y"], context)
    }
}
