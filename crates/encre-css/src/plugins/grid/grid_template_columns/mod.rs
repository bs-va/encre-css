#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::is_matching_all,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "grid-cols"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok() || *value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "none" {
                    return writeln!(buffer, "{indentation}grid-template-columns: none;");
                }

                writeln!(
                    buffer,
                    "{indentation}grid-template-columns: repeat({}, minmax(0, 1fr));",
                    value.parse::<usize>().unwrap(),
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}grid-template-columns: {value};")?;
            }
        }

        Ok(())
    }
}
