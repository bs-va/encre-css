#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "hidden"
                    | "contents"
                    | "list-item"
                    | "block"
                    | "inline-block"
                    | "flex"
                    | "inline-flex"
                    | "inline"
                    | "table"
                    | "inline-table"
                    | "table-cell"
                    | "table-caption"
                    | "table-column"
                    | "table-column-group"
                    | "table-footer-group"
                    | "table-header-group"
                    | "table-row-group"
                    | "table-row"
                    | "flow-root"
                    | "grid"
                    | "inline-grid",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin {
            value: "hidden", ..
        } = context.modifier
        {
            context.buffer.line("display: none;");
        } else if let Modifier::Builtin { value, .. } = context.modifier {
            context.buffer.line(format_args!("display: {value};"));
        }
    }
}
