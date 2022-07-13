#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{color, indent, value_matchers::is_matching_color},
};

use std::fmt::{self, Write};

fn color_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => color::is_matching_builtin_color(context.config, value),
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "color" || (hint.is_empty() && is_matching_color(value))
        }
    }
}

fn color_handle(css_props: &[&str], context: &mut ContextHandle) -> fmt::Result {
    indent(context.indentation, context.buffer)?;
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            let color = color::get(context.config, value, Some("--en-border-opacity")).unwrap();
            if color.contains("--en-border-opacity") {
                writeln!(context.buffer, "--en-border-opacity: 1;")?;
                indent(context.indentation, context.buffer)?;
            }

            for css_prop in css_props {
                writeln!(context.buffer, "{css_prop}: {color};")?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_props {
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
        "border"
    }

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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        color_handle(&["border-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &str {
        "border-x"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        color_handle(&["border-left-color", "border-right-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &str {
        "border-y"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        color_handle(&["border-top-color", "border-bottom-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginLeftDefinition;

impl Plugin for PluginLeftDefinition {
    fn namespace(&self) -> &str {
        "border-l"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        color_handle(&["border-left-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginRightDefinition;

impl Plugin for PluginRightDefinition {
    fn namespace(&self) -> &str {
        "border-r"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        color_handle(&["border-right-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginTopDefinition;

impl Plugin for PluginTopDefinition {
    fn namespace(&self) -> &str {
        "border-t"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        color_handle(&["border-top-color"], context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginBottomDefinition;

impl Plugin for PluginBottomDefinition {
    fn namespace(&self) -> &str {
        "border-b"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        color_can_handle(&mut context)
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        color_handle(&["border-bottom-color"], context)
    }
}
