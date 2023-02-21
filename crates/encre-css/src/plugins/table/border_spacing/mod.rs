#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

fn border_spacing_can_handle(context: &ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
        Modifier::Arbitrary { value, prefix, .. } => prefix.is_empty() && is_matching_length(value),
    }
}

fn border_spacing_handle(css_props: &[&str], context: &mut ContextHandle) {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_props {
                context.buffer.line(format_args!(
                    "{}: {};",
                    css_prop,
                    spacing::get(value, *is_negative).unwrap(),
                ));
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_props {
                context.buffer.line(format_args!("{css_prop}: {value};"));
            }
        }
    }

    context
        .buffer
        .line("border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);");
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        border_spacing_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        border_spacing_handle(&["--en-border-spacing-x", "--en-border-spacing-y"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        border_spacing_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        border_spacing_handle(&["--en-border-spacing-x"], context);
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        border_spacing_can_handle(&context)
    }

    fn handle(&self, context: &mut ContextHandle) {
        border_spacing_handle(&["--en-border-spacing-y"], context);
    }
}
