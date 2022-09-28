#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "hidden" => writeln!(buffer, "{indentation}display: none;")?,
                "contents" => writeln!(buffer, "{indentation}display: contents;")?,
                "list-item" => writeln!(buffer, "{indentation}display: list-item;")?,
                "block" => writeln!(buffer, "{indentation}display: block;")?,
                "inline-block" => writeln!(buffer, "{indentation}display: inline-block;")?,
                "flex" => writeln!(buffer, "{indentation}display: flex;")?,
                "inline-flex" => writeln!(buffer, "{indentation}display: inline-flex;")?,
                "inline" => writeln!(buffer, "{indentation}display: inline;")?,
                "table" => writeln!(buffer, "{indentation}display: table;")?,
                "inline-table" => writeln!(buffer, "{indentation}display: inline-table;")?,
                "table-cell" => writeln!(buffer, "{indentation}display: table-cell;")?,
                "table-caption" => writeln!(buffer, "{indentation}display: table-caption;")?,
                "table-column" => writeln!(buffer, "{indentation}display: table-column;")?,
                "table-column-group" => writeln!(buffer, "{indentation}display: table-column-group;")?,
                "table-footer-group" => writeln!(buffer, "{indentation}display: table-footer-group;")?,
                "table-header-group" => writeln!(buffer, "{indentation}display: table-header-group;")?,
                "table-row-group" => writeln!(buffer, "{indentation}display: table-row-group;")?,
                "table-row" => writeln!(buffer, "{indentation}display: table-row;")?,
                "flow-root" => writeln!(buffer, "{indentation}display: flow-root;")?,
                "grid" => writeln!(buffer, "{indentation}display: grid;")?,
                "inline-grid" => writeln!(buffer, "{indentation}display: inline-grid;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
