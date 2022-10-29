#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => context.buffer.line(format_args!(
                "--en-backdrop-hue-rotate: hue-rotate({}{}deg);",
                format_negative(is_negative),
                value
            )),
            Modifier::Arbitrary { value, .. } => context.buffer.line(format_args!(
                "--en-backdrop-hue-rotate: hue-rotate({value});",
            )),
        }

        context.buffer.lines(CSS_BACKDROP_FILTER);
    }
}
