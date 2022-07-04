#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::indent,
};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "font"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["sans", "serif", "mono"].contains(&&**value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "generic-name"
                    || *hint == "family-name"
                    || (hint.is_empty()
                        && value.split(',').all(|v| v[..1].parse::<usize>().is_err()))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "sans" => writeln!(
                    context.buffer,
                    r#"font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";"#
                )?,
                "serif" => writeln!(
                    context.buffer,
                    r#"font-family: Georgia, Cambria, "Times New Roman", Times, serif;"#
                )?,
                "mono" => writeln!(
                    context.buffer,
                    r#"font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;"#
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "font-family: {};",
                to_css_value(value)
                    .split(',')
                    .map(|v| if v.trim().contains(' ') {
                        Cow::from(format!(r#""{}""#, v))
                    } else {
                        Cow::from(v)
                    })
                    .collect::<Vec<Cow<str>>>()
                    .join(","),
            )?,
        }

        Ok(())
    }
}
