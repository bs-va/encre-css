#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "overscroll"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "contain",
                "x-contain",
                "y-contain",
                "none",
                "x-none",
                "y-none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(buffer, "{indentation}overscroll-behavior: auto;")?,
                "x-auto" => writeln!(buffer, "{indentation}overscroll-behavior-x: auto;")?,
                "y-auto" => writeln!(buffer, "{indentation}overscroll-behavior-y: auto;")?,
                "contain" => writeln!(buffer, "{indentation}overscroll-behavior: contain;")?,
                "x-contain" => writeln!(buffer, "{indentation}overscroll-behavior-x: contain;")?,
                "y-contain" => writeln!(buffer, "{indentation}overscroll-behavior-y: contain;")?,
                "none" => writeln!(buffer, "{indentation}overscroll-behavior: none;")?,
                "x-none" => writeln!(buffer, "{indentation}overscroll-behavior-x: none;")?,
                "y-none" => writeln!(buffer, "{indentation}overscroll-behavior-y: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
