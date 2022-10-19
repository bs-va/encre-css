#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "hidden",
                "contents",
                "list-item",
                "block",
                "inline-block",
                "flex",
                "inline-flex",
                "inline",
                "table",
                "inline-table",
                "table-cell",
                "table-caption",
                "table-column",
                "table-column-group",
                "table-footer-group",
                "table-header-group",
                "table-row-group",
                "table-row",
                "flow-root",
                "grid",
                "inline-grid",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "hidden" => context.buffer.line("display: none;"),
                "contents" => context.buffer.line("display: contents;"),
                "list-item" => context.buffer.line("display: list-item;"),
                "block" => context.buffer.line("display: block;"),
                "inline-block" => context.buffer.line("display: inline-block;"),
                "flex" => context.buffer.line("display: flex;"),
                "inline-flex" => context.buffer.line("display: inline-flex;"),
                "inline" => context.buffer.line("display: inline;"),
                "table" => context.buffer.line("display: table;"),
                "inline-table" => context.buffer.line("display: inline-table;"),
                "table-cell" => context.buffer.line("display: table-cell;"),
                "table-caption" => context.buffer.line("display: table-caption;"),
                "table-column" => context.buffer.line("display: table-column;"),
                "table-column-group" => context.buffer.line("display: table-column-group;"),
                "table-footer-group" => context.buffer.line("display: table-footer-group;"),
                "table-header-group" => context.buffer.line("display: table-header-group;"),
                "table-row-group" => context.buffer.line("display: table-row-group;"),
                "table-row" => context.buffer.line("display: table-row;"),
                "flow-root" => context.buffer.line("display: flow-root;"),
                "grid" => context.buffer.line("display: grid;"),
                "inline-grid" => context.buffer.line("display: inline-grid;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
