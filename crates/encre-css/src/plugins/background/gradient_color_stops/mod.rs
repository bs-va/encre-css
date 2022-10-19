#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;

use std::borrow::Cow;

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

    fn handle(&self, context: &mut ContextHandle) {
        let value = match context.modifier {
            Modifier::Builtin { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => value.clone(),
        };

        let default_to = if value == "inherit" || value == "currentColor" {
            Cow::from("rgb(255 255 255 / 0)")
        } else {
            let mut default = value.to_string();
            default.pop(); // Remove the last `)`
            default += "/ 0)";
            Cow::from(default)
        };

        context.buffer.line("--en-gradient-from: {value};");
        context.buffer.line(format_args!(
            "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, {default_to});"
        ));
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

    fn handle(&self, context: &mut ContextHandle) {
        let value = match context.modifier {
            Modifier::Builtin { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => value.clone(),
        };

        let default_to = if value == "inherit" || value == "currentColor" {
            Cow::from("rgb(255 255 255 / 0)")
        } else {
            let mut default = value.to_string();
            default.pop(); // Remove the last `)`
            default += "/ 0)";
            Cow::from(default)
        };

        context.buffer.line(format_args!(
            "--en-gradient-stops: var(--en-gradient-from), {}, var(--en-gradient-to, {});",
            value, default_to
        ));
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

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "--en-gradient-to: {};",
                color::get(context.config, value, None).unwrap()
            )),
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("--en-gradient-to: {value};"));
            }
        }
    }
}
