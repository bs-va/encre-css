#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

fn width_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => value.is_empty() || value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "length"
                || (hint.is_empty() && (is_matching_length(value) || is_matching_line_width(value)))
        }
    }
}

fn width_handle(css_properties: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            for css_prop in css_properties {
                context.buffer.line(format_args!(
                    "{}: {}px;",
                    css_prop,
                    if value.is_empty() { "1" } else { value }
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
            Modifier::Builtin { value, .. } => value.is_empty() || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary {
                prefix,
                hint,
                value,
                ..
            } => {
                prefix.is_empty()
                    && (*hint == "length"
                        || *hint == "line-width"
                        || (hint.is_empty()
                            && (is_matching_length(value) || is_matching_line_width(value))))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        width_handle(&["border-width"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        width_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        width_handle(&["border-top-width"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        width_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        width_handle(&["border-bottom-width"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        width_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        width_handle(&["border-left-width"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        width_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        width_handle(&["border-right-width"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        width_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        width_handle(&["border-left-width", "border-right-width"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        width_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        width_handle(&["border-top-width", "border-bottom-width"], context);
    }
}
