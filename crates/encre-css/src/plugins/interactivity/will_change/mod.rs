#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "will-change"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["auto", "scroll", "contents", "transform"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(buffer, "{indentation}will-change: auto;")?,
                "scroll" => writeln!(buffer, "{indentation}will-change: scroll-position;")?,
                "contents" => writeln!(buffer, "{indentation}will-change: contents;")?,
                "transform" => writeln!(buffer, "{indentation}will-change: transform;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "{indentation}will-change: {value};")?,
        }

        Ok(())
    }
}
