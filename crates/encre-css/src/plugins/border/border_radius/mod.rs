#![doc = include_str!("README.md")]
#![doc(alias("border", "rounded"))]
use crate::prelude::build_plugin::*;

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

fn radius_handle(css_properties: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            for css_prop in css_properties {
                context.buffer.line(format_args!(
                    "{}: {};",
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

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(&["border-radius"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopRightDefinition;

impl Plugin for PluginTopRightDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(&["border-top-right-radius"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopLeftDefinition;

impl Plugin for PluginTopLeftDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(&["border-top-left-radius"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomRightDefinition;

impl Plugin for PluginBottomRightDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(&["border-bottom-right-radius"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomLeftDefinition;

impl Plugin for PluginBottomLeftDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(&["border-bottom-left-radius"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(
            &["border-top-left-radius", "border-top-right-radius"],
            context,
        );
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(
            &["border-bottom-left-radius", "border-bottom-right-radius"],
            context,
        );
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(
            &["border-top-left-radius", "border-bottom-left-radius"],
            context,
        );
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        radius_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        radius_handle(
            &["border-top-right-radius", "border-bottom-right-radius"],
            context,
        );
    }
}
