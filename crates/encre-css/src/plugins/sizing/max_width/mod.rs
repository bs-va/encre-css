#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "max-w"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                [
                    "xs",
                    "sm",
                    "md",
                    "lg",
                    "xl",
                    "2xl",
                    "3xl",
                    "4xl",
                    "5xl",
                    "6xl",
                    "7xl",
                    "full",
                    "min",
                    "max",
                    "fit",
                    "prose",
                    "screen",
                    "screen-sm",
                    "screen-md",
                    "screen-lg",
                    "screen-lg",
                    "screen-xl",
                    "screen-2xl",
                    "none",
                ]
                .contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "none" => writeln!(context.buffer, "max-width: none;")?,
                "xs" => writeln!(context.buffer, "max-width: 20rem;")?,
                "sm" => writeln!(context.buffer, "max-width: 24rem;")?,
                "md" => writeln!(context.buffer, "max-width: 28rem;")?,
                "lg" => writeln!(context.buffer, "max-width: 32rem;")?,
                "xl" => writeln!(context.buffer, "max-width: 36rem;")?,
                "2xl" => writeln!(context.buffer, "max-width: 42rem;")?,
                "3xl" => writeln!(context.buffer, "max-width: 48rem;")?,
                "4xl" => writeln!(context.buffer, "max-width: 56rem;")?,
                "5xl" => writeln!(context.buffer, "max-width: 64rem;")?,
                "6xl" => writeln!(context.buffer, "max-width: 72rem;")?,
                "7xl" => writeln!(context.buffer, "max-width: 80rem;")?,
                "full" => writeln!(context.buffer, "max-width: 100%;")?,
                "min" => writeln!(context.buffer, "max-width: min-content;")?,
                "max" => writeln!(context.buffer, "max-width: max-content;")?,
                "fit" => writeln!(context.buffer, "max-width: fit-content;")?,
                "prose" => writeln!(context.buffer, "max-width: 65ch;")?,
                "screen" => writeln!(context.buffer, "max-width: 100vw;")?,
                "screen-sm" => writeln!(context.buffer, "max-width: 640px;")?,
                "screen-md" => writeln!(context.buffer, "max-width: 768px;")?,
                "screen-lg" => writeln!(context.buffer, "max-width: 1024px;")?,
                "screen-xl" => writeln!(context.buffer, "max-width: 1280px;")?,
                "screen-2xl" => writeln!(context.buffer, "max-width: 1536px;")?,
                _ => writeln!(
                    context.buffer,
                    "min-width: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "max-width: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
