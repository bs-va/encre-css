#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_image},
};
use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "none",
                "gradient-to-t",
                "gradient-to-tr",
                "gradient-to-r",
                "gradient-to-br",
                "gradient-to-b",
                "gradient-to-bl",
                "gradient-to-l",
                "gradient-to-tl",
            ]
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "image" || *hint == "url" || (hint.is_empty() && is_matching_image(value))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "none" => writeln!(context.buffer, "background-image: none;")?,
                "gradient-to-t" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to top, var(--en-gradient-stops));"
                )?,
                "gradient-to-tr" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to top right, var(--en-gradient-stops));"
                )?,
                "gradient-to-r" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to right, var(--en-gradient-stops));"
                )?,
                "gradient-to-br" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to bottom right, var(--en-gradient-stops));"
                )?,
                "gradient-to-b" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to bottom, var(--en-gradient-stops));"
                )?,
                "gradient-to-bl" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to bottom left, var(--en-gradient-stops));"
                )?,
                "gradient-to-l" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to left, var(--en-gradient-stops));"
                )?,
                "gradient-to-tl" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to top left, var(--en-gradient-stops));"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "background-image: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
