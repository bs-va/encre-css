#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

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
                        && value
                            .split(',')
                            .all(|v| v.is_empty() || v[..1].parse::<usize>().is_err()))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
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
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "font-family: {value};")?;
            }
        }

        Ok(())
    }
}
