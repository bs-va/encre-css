#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
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
        "line-clamp"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "none" || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "none" {
                    writeln!(buffer, "{indentation}-webkit-line-clamp: unset;")
                } else {
                    writeln!(buffer, "{indentation}overflow: hidden;
{indentation}display: -webkit-box;
{indentation}-webkit-box-orient: vertical;
{indentation}-webkit-line-clamp: {value};")
                }
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
