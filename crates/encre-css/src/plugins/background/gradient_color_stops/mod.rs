#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{color, indent, value_matchers::is_matching_color},
};
use std::{
    borrow::Cow,
    fmt::{self, Write},
};

#[derive(Debug)]
pub(crate) struct PluginFromDefinition;

impl Plugin for PluginFromDefinition {
    fn namespace(&self) -> &str {
        "from"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        let value = match context.modifier {
            Modifier::Builtin { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        let default_to = if value == "inherit" || value == "currentColor" {
            Cow::from("rgb(255 255 255 / 0)")
        } else {
            let mut default = value.to_string();
            default.pop(); // Remove the last `)`
            default += "/ 0)";
            Cow::from(default)
        };

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "--en-gradient-from: {value};")?;
        indent(context.indentation, context.buffer)?;
        writeln!(
            context.buffer,
            "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, {default_to});"
        )?;

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PluginViaDefinition;

impl Plugin for PluginViaDefinition {
    fn namespace(&self) -> &str {
        "via"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        let value = match context.modifier {
            Modifier::Builtin { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        let default_to = if value == "inherit" || value == "currentColor" {
            Cow::from("rgb(255 255 255 / 0)")
        } else {
            let mut default = value.to_string();
            default.pop(); // Remove the last `)`
            default += "/ 0)";
            Cow::from(default)
        };

        indent(context.indentation, context.buffer)?;
        writeln!(
            context.buffer,
            "--en-gradient-stops: var(--en-gradient-from), {}, var(--en-gradient-to, {});",
            value, default_to
        )?;

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PluginToDefinition;

impl Plugin for PluginToDefinition {
    fn namespace(&self) -> &str {
        "to"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        let value = match context.modifier {
            Modifier::Builtin { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "--en-gradient-to: {value};")?;

        Ok(())
    }
}
