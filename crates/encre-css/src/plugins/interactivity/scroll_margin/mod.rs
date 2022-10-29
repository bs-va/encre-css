#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

fn scroll_margin_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

fn scroll_margin_handle(css_properties: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
                context.buffer.line(format_args!(
                    "{}: {};",
                    css_prop,
                    spacing::get(value, *is_negative).unwrap(),
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
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scroll_margin_handle(&["scroll-margin"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scroll_margin_handle(&["scroll-margin-left", "scroll-margin-right"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scroll_margin_handle(&["scroll-margin-top", "scroll-margin-bottom"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scroll_margin_handle(&["scroll-margin-left"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scroll_margin_handle(&["scroll-margin-right"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scroll_margin_handle(&["scroll-margin-top"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        scroll_margin_handle(&["scroll-margin-bottom"], context)
    }
}
