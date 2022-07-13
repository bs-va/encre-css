#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

fn scroll_margin_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

fn scroll_margin_handle(css_properties: &[&str], context: &mut ContextHandle) -> fmt::Result {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
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
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "{css_prop}: {value};")?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "scroll-m"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        scroll_margin_handle(&["scroll-margin"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &str {
        "scroll-mx"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        scroll_margin_handle(&["scroll-margin-left", "scroll-margin-right"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &str {
        "scroll-my"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        scroll_margin_handle(&["scroll-margin-top", "scroll-margin-bottom"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn namespace(&self) -> &str {
        "scroll-ml"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        scroll_margin_handle(&["scroll-margin-left"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn namespace(&self) -> &str {
        "scroll-mr"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        scroll_margin_handle(&["scroll-margin-right"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn namespace(&self) -> &str {
        "scroll-mt"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        scroll_margin_handle(&["scroll-margin-top"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn namespace(&self) -> &str {
        "scroll-mb"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scroll_margin_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        scroll_margin_handle(&["scroll-margin-bottom"], context)
    }
}
