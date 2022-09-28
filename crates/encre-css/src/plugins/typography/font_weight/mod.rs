#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::{is_matching_integer, is_matching_var},
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "thin" => writeln!(buffer, "{indentation}font-weight: 100;")?,
                "extralight" => writeln!(buffer, "{indentation}font-weight: 200;")?,
                "light" => writeln!(buffer, "{indentation}font-weight: 300;")?,
                "normal" => writeln!(buffer, "{indentation}font-weight: 400;")?,
                "medium" => writeln!(buffer, "{indentation}font-weight: 500;")?,
                "semibold" => writeln!(buffer, "{indentation}font-weight: 600;")?,
                "bold" => writeln!(buffer, "{indentation}font-weight: 700;")?,
                "extrabold" => writeln!(buffer, "{indentation}font-weight: 800;")?,
                "black" => writeln!(buffer, "{indentation}font-weight: 900;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}font-weight: {value};")?;
            }
        }

        Ok(())
    }
}
