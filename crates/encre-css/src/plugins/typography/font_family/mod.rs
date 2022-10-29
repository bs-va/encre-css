#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["sans", "serif", "mono"].contains(&&**value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "generic-name"
                    || *hint == "family-name"
                    || (hint.is_empty()
                        && value
                            .split(',')
                            .all(|v| v.is_empty() || v[..1].parse::<usize>().is_err()))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "sans" => context.buffer.line(
                    r#"font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";"#
                ),
                "serif" => context.buffer.line(
                    r#"font-family: Georgia, Cambria, "Times New Roman", Times, serif;"#
                ),
                "mono" => context.buffer.line(
                    r#"font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;"#
                ),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("font-family: {value};"));
            }
        }
    }
}
