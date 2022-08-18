#![doc = include_str!("README.md")]
#![doc(alias = "accessibility")]

use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "sr-only" || *value == "not-sr-only",
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "sr-only" => "position: absolute;
width: 1px;
height: 1px;
padding: 0;
margin: -1px;
overflow: hidden;
clip: rect(0, 0, 0, 0);
white-space: nowrap;
border-width: 0;"
                    .lines()
                    .try_for_each(|line| {
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "{line}")
                    }),
                "not-sr-only" => "position: static;
width: auto;
height: auto;
padding: 0;
margin: 0;
overflow: visible;
clip: auto;
white-space: normal;"
                    .lines()
                    .try_for_each(|line| {
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "{line}")
                    }),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
