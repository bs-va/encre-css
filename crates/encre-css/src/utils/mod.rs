use std::fmt::{self, Write};

pub mod color;
pub mod length;
pub mod shadow;
pub mod value_matchers;

pub const INDENTATION_SIZE: usize = 2;

pub fn indent(num: usize, buffer: &mut String) -> fmt::Result {
    write!(buffer, "{:indent$}", "", indent = num * INDENTATION_SIZE)
}

pub fn format_negative(is_negative: &bool) -> &'static str {
    if *is_negative {
        "-"
    } else {
        ""
    }
}
