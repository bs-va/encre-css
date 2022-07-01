#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, value_matchers::{is_matching_integer, is_matching_var}},
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
            Modifier::Builtin { value, .. } => [
                "thin",
                "extralight",
                "light",
                "normal",
                "medium",
                "semibold",
                "bold",
                "extrabold",
                "black",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "number"
                    && (hint.is_empty()
                        && (["normal", "bold", "lighter", "bolder"].contains(&&**value)
                            || is_matching_integer(value)
                            || is_matching_var(value)))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "thin" => writeln!(context.buffer, "font-weight: 100;")?,
                "extralight" => writeln!(context.buffer, "font-weight: 200;")?,
                "light" => writeln!(context.buffer, "font-weight: 300;")?,
                "normal" => writeln!(context.buffer, "font-weight: 400;")?,
                "medium" => writeln!(context.buffer, "font-weight: 500;")?,
                "semibold" => writeln!(context.buffer, "font-weight: 600;")?,
                "bold" => writeln!(context.buffer, "font-weight: 700;")?,
                "extrabold" => writeln!(context.buffer, "font-weight: 800;")?,
                "black" => writeln!(context.buffer, "font-weight: 900;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "font-weight: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
