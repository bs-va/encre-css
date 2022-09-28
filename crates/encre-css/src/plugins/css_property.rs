//! Define a plugin used to generate CSS properties quickly.
//!
//! Used for arbitrary CSS properties like `[mask-type:luminance]`.
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct CssPropertyPlugin;

impl Plugin for CssPropertyPlugin {
    fn can_handle(&self, _context: ContextCanHandle) -> bool {
        // NOTE: No need to implement it because we are manually calling the `handle` method in `selector.rs`
        unreachable!();
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { .. } => unreachable!(),
            Modifier::Arbitrary { value, .. } => {
                for line in value.lines() {
                    if let Some((prop, value)) = line.split_once(':') {
                        writeln!(buffer, "{indentation}{prop}: {value};")?;
                    }
                }
            }
        }

        Ok(())
    }
}
