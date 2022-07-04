#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
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
                writeln!(context.buffer, "{}: {};", css_prop, to_css_value(value))?;
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        border_spacing_handle(
            &["--en-border-spacing-x", "--en-border-spacing-y"],
            &mut context,
        )
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-x"], &mut context)
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-y"], &mut context)
    }
}
