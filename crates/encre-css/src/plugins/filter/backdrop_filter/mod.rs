#![doc = include_str!("README.md")]
use super::{CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2};
use crate::{
    plugins::Plugin,
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "backdrop-filter"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => {
                    writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;
                }
                "none" => {
                    writeln!(context.buffer, "-webkit-backdrop-filter: none;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "backdrop-filter: none;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
