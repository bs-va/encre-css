#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "overflow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "hidden",
                "x-hidden",
                "y-hidden",
                "visible",
                "x-visible",
                "y-visible",
                "scroll",
                "x-scroll",
                "y-scroll",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(buffer, "{indentation}overflow: auto;")?,
                "x-auto" => writeln!(buffer, "{indentation}overflow-x: auto;")?,
                "y-auto" => writeln!(buffer, "{indentation}overflow-y: auto;")?,
                "hidden" => writeln!(buffer, "{indentation}overflow: hidden;")?,
                "x-hidden" => writeln!(buffer, "{indentation}overflow-x: hidden;")?,
                "y-hidden" => writeln!(buffer, "{indentation}overflow-y: hidden;")?,
                "visible" => writeln!(buffer, "{indentation}overflow: visible;")?,
                "x-visible" => writeln!(buffer, "{indentation}overflow-x: visible;")?,
                "y-visible" => writeln!(buffer, "{indentation}overflow-y: visible;")?,
                "scroll" => writeln!(buffer, "{indentation}overflow: scroll;")?,
                "x-scroll" => writeln!(buffer, "{indentation}overflow-x: scroll;")?,
                "y-scroll" => writeln!(buffer, "{indentation}overflow-y: scroll;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
