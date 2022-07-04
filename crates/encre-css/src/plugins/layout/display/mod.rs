#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "hidden" => writeln!(context.buffer, "display: none;")?,
                "contents" => writeln!(context.buffer, "display: contents;")?,
                "list-item" => writeln!(context.buffer, "display: list-item;")?,
                "block" => writeln!(context.buffer, "display: block;")?,
                "inline-block" => writeln!(context.buffer, "display: inline-block;")?,
                "flex" => writeln!(context.buffer, "display: flex;")?,
                "inline-flex" => writeln!(context.buffer, "display: inline-flex;")?,
                "inline" => writeln!(context.buffer, "display: inline;")?,
                "table" => writeln!(context.buffer, "display: table;")?,
                "inline-table" => writeln!(context.buffer, "display: inline-table;")?,
                "table-cell" => writeln!(context.buffer, "display: table-cell;")?,
                "table-caption" => writeln!(context.buffer, "display: table-caption;")?,
                "table-column" => writeln!(context.buffer, "display: table-column;")?,
                "table-column-group" => writeln!(context.buffer, "display: table-column-group;")?,
                "table-footer-group" => writeln!(context.buffer, "display: table-footer-group;")?,
                "table-header-group" => writeln!(context.buffer, "display: table-header-group;")?,
                "table-row-group" => writeln!(context.buffer, "display: table-row-group;")?,
                "table-row" => writeln!(context.buffer, "display: table-row;")?,
                "flow-root" => writeln!(context.buffer, "display: flow-root;")?,
                "grid" => writeln!(context.buffer, "display: grid;")?,
                "inline-grid" => writeln!(context.buffer, "display: inline-grid;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
