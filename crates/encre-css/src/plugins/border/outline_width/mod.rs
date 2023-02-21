#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary {
                hint,
                value,
                prefix,
            } => {
                prefix.is_empty()
                    && (*hint == "length"
                        || *hint == "line-width"
                        || (hint.is_empty()
                            && (is_matching_length(value) || is_matching_line_width(value))))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                context
                    .buffer
                    .line(format_args!("outline-width: {value}px;"));
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("outline-width: {value};"));
            }
        }
    }
}
