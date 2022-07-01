//! Define some utility functions for quickly doing things.
use std::fmt::{self, Write};

pub mod color;
pub mod shadow;
pub mod spacing;
pub mod value_matchers;

const INDENTATION_SIZE: usize = 2;

/// Indent a line using two spaces.
///
/// # Errors
///
/// Returns [`Error::Format`] if writing to the buffer failed.
///
/// [`Error::Format`]: crate::Error::Format
pub fn indent(num: usize, buffer: &mut String) -> fmt::Result {
    write!(buffer, "{:indent$}", "", indent = num * INDENTATION_SIZE)
}

/// Quickly format a negative value (returns "-" if true or "" otherwise).
pub fn format_negative(is_negative: &bool) -> &'static str {
    if *is_negative {
        "-"
    } else {
        ""
    }
}
