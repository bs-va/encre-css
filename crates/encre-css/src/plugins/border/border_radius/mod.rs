#![doc = include_str!("README.md")]
#![doc(alias("border", "rounded"))]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::{is_matching_length, is_matching_percentage},
};

use std::fmt::{self, Write};

fn radius_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            value.is_empty()
                || ["sm", "md", "lg", "xl", "2xl", "3xl", "full", "none"].contains(&&**value)
        }
        Modifier::Arbitrary { value, .. } => value
            .split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v)),
    }
}

fn radius_handle(css_properties: &[&str], ContextHandle { modifier, indentation, buffer, .. }: &mut ContextHandle) -> fmt::Result {
    match modifier {
        Modifier::Builtin { value, .. } => {
            for css_prop in css_properties {
                writeln!(
                    buffer,
                    "{indentation}{}: {};",
                    css_prop,
                    match *value {
                        "" => "0.25rem",
                        "none" => "0",
                        "sm" => "0.125rem",
                        "md" => "0.375rem",
                        "lg" => "0.5rem",
                        "xl" => "0.75rem",
                        "2xl" => "1rem",
                        "3xl" => "1.5rem",
                        "full" => "9999px",
                        _ => unreachable!(),
                    }
                )?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_properties {
                writeln!(buffer, "{indentation}{css_prop}: {value};")?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "rounded"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                value.is_empty()
                    || ["sm", "md", "lg", "xl", "2xl", "3xl", "full", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty()
                    && value
                        .split('_')
                        .all(|v| is_matching_length(v) || is_matching_percentage(v))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(&["border-radius"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopRightDefinition;

impl Plugin for PluginTopRightDefinition {
    fn namespace(&self) -> &str {
        "rounded-tr"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(&["border-top-right-radius"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopLeftDefinition;

impl Plugin for PluginTopLeftDefinition {
    fn namespace(&self) -> &str {
        "rounded-tl"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(&["border-top-left-radius"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomRightDefinition;

impl Plugin for PluginBottomRightDefinition {
    fn namespace(&self) -> &str {
        "rounded-br"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(&["border-bottom-right-radius"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomLeftDefinition;

impl Plugin for PluginBottomLeftDefinition {
    fn namespace(&self) -> &str {
        "rounded-bl"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(&["border-bottom-left-radius"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn namespace(&self) -> &str {
        "rounded-t"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(
            &["border-top-left-radius", "border-top-right-radius"],
            context,
        )
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn namespace(&self) -> &str {
        "rounded-b"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(
            &["border-bottom-left-radius", "border-bottom-right-radius"],
            context,
        )
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn namespace(&self) -> &str {
        "rounded-l"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(
            &["border-top-left-radius", "border-bottom-left-radius"],
            context,
        )
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn namespace(&self) -> &str {
        "rounded-r"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        radius_handle(
            &["border-top-right-radius", "border-bottom-right-radius"],
            context,
        )
    }
}
