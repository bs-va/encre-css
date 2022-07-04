#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
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
            let value = to_css_value(value);

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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        color_handle(&["border-color"], &mut context)
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        color_handle(&["border-left-color", "border-right-color"], &mut context)
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        color_handle(&["border-top-color", "border-bottom-color"], &mut context)
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        color_handle(&["border-left-color"], &mut context)
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        color_handle(&["border-right-color"], &mut context)
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        color_handle(&["border-top-color"], &mut context)
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

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        color_handle(&["border-bottom-color"], &mut context)
    }
}
