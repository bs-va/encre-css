#![doc = include_str!("README.md")]
use crate::{
    plugins::Plugin,
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

/// Utilities for controlling whether an element should explicitly create a new stacking context.
///
/// <table style="display: table;">
///     <thead>
///         <tr>
///             <th style="text-align: center;">Class</th>
///             <th style="text-align: center;">Properties</th>
///         </tr>
///     </thead>
///     <tbody>
///         <tr><td>isolate</td><td>isolation: isolate;</td></tr>
///         <tr><td>isolation-auto</td><td>isolation: auto;</td></tr>
///     </tbody>
/// </table>
///
/// [Tailwind reference](https://tailwindcss.com/docs/isolation)
#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["isolate", "isolation-auto"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "isolate" => writeln!(context.buffer, "isolation: isolate;")?,
                "isolation-auto" => writeln!(context.buffer, "isolation: auto;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
