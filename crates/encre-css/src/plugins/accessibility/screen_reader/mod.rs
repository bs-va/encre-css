#![doc = include_str!("README.md")]
#![doc(alias = "accessibility")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "sr-only" || *value == "not-sr-only",
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "sr-only" => writeln!(buffer, "{indentation}position: absolute;
{indentation}width: 1px;
{indentation}height: 1px;
{indentation}padding: 0;
{indentation}margin: -1px;
{indentation}overflow: hidden;
{indentation}clip: rect(0, 0, 0, 0);
{indentation}white-space: nowrap;
{indentation}border-width: 0;"),
                "not-sr-only" => writeln!(buffer, "{indentation}position: static;
{indentation}width: auto;
{indentation}height: auto;
{indentation}padding: 0;
{indentation}margin: 0;
{indentation}overflow: visible;
{indentation}clip: auto;
{indentation}white-space: normal;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
