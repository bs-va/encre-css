#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

fn color_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => color::is_matching_builtin_color(context.config, value),
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "color" || (hint.is_empty() && is_matching_color(value))
        }
    }
}

fn color_handle(css_props: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            let color = color::get(context.config, value, Some("--en-border-opacity")).unwrap();
            if color.contains("--en-border-opacity") {
                context.buffer.line("--en-border-opacity: 1;");
            }

            for css_prop in css_props {
                context.buffer.line(format_args!("{css_prop}: {color};"));
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_props {
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
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary {
                prefix,
                hint,
                value,
                ..
            } => {
                prefix.is_empty()
                    && (*hint == "color" || (hint.is_empty() && is_matching_color(value)))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        color_handle(&["border-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        color_handle(&["border-left-color", "border-right-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        color_handle(&["border-top-color", "border-bottom-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        color_handle(&["border-left-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        color_handle(&["border-right-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        color_handle(&["border-top-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        color_handle(&["border-bottom-color"], context)
    }
}
