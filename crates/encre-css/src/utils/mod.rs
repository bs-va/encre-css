//! Define some utility functions for quickly doing things.
use crate::selector::parser::{ARBITRARY_END, ARBITRARY_START, ESCAPE, GROUP_END, GROUP_START};

use std::{
    fmt::{self, Write},
    str::CharIndices,
};

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

/// While <https://github.com/rust-lang/rust/issues/27721> is pending we need to define
/// our own minimal [`Pattern`] trait.
///
/// | Pattern type             | Match condition                           |
/// |--------------------------|-------------------------------------------|
/// | `&str`                   | is substring                              |
/// | `char`                   | is contained in string                    |
/// | `&[char]`                | any char in slice is contained in string  |
/// | `F: FnMut(char) -> bool` | `F` returns `true` for a char in string   |
/// | `&&str`                  | is substring                              |
/// | `&String`                | is substring                              |
///
/// [`Pattern`]: std::str::pattern::Pattern
pub trait Pattern {
    /// Returns whether the value is matching the pattern.
    fn is_matching(&self, val: &str) -> bool;
}

impl Pattern for char {
    fn is_matching(&self, val: &str) -> bool {
        val.contains(*self)
    }
}

impl Pattern for &str {
    fn is_matching(&self, val: &str) -> bool {
        val.contains(self)
    }
}

impl Pattern for &String {
    fn is_matching(&self, val: &str) -> bool {
        val.contains(*self)
    }
}

impl Pattern for String {
    fn is_matching(&self, val: &str) -> bool {
        val.contains(self)
    }
}

impl Pattern for &[char] {
    fn is_matching(&self, val: &str) -> bool {
        self.iter().any(|ch| val.contains(*ch))
    }
}

impl Pattern for &&str {
    fn is_matching(&self, val: &str) -> bool {
        val.contains(*self)
    }
}

impl<F: Fn(char) -> bool> Pattern for F {
    fn is_matching(&self, val: &str) -> bool {
        val.chars().any(self)
    }
}

/// An iterator ignoring values wrapped in parenthesis and brackets.
///
/// This structure is created by the [`split_ignore_arbitrary`] function. See its documentation for
/// more.
#[derive(Debug)]
pub struct SplitIgnoreArbitrary<'a, P: Pattern> {
    val: &'a str,
    iter: CharIndices<'a>,
    searched_pattern: P,
    is_next_escaped: bool,
    parenthesis_level: usize,
    bracket_level: usize,
    last_index: usize,
    seek_index: usize,
}

impl<'a, P: Pattern> Iterator for SplitIgnoreArbitrary<'a, P> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.is_next_escaped {
                let _ = self.iter.next()?;
                self.is_next_escaped = false;
                continue;
            }

            let ch = self.iter.next();

            if let Some(ch) = ch {
                match ch.1 {
                    ESCAPE => self.is_next_escaped = true,
                    GROUP_START => self.parenthesis_level += 1,
                    GROUP_END => {
                        if self.parenthesis_level > 0 {
                            self.parenthesis_level -= 1;
                            self.seek_index = ch.0 + 1;
                        } else {
                            // Parenthesis not opened, abort.
                            return None;
                        }
                    }
                    ARBITRARY_START => self.bracket_level += 1,
                    ARBITRARY_END => {
                        if self.bracket_level > 0 {
                            self.bracket_level -= 1;
                            self.seek_index = ch.0 + 1;
                        } else {
                            // Bracket not opened, abort.
                            return None;
                        }
                    }
                    _ => {
                        if self
                            .searched_pattern
                            .is_matching(&self.val[self.seek_index..ch.0 + ch.1.len_utf8()])
                            && self.parenthesis_level == 0
                            && self.bracket_level == 0
                        {
                            let last_index = self.last_index;
                            self.last_index = ch.0 + ch.1.len_utf8();
                            self.seek_index = self.last_index;
                            return Some(&self.val[last_index..ch.0]);
                        }
                    }
                }
            } else if self.last_index != self.val.len() {
                // The characters are all handled, return the last slice
                let last_index = self.last_index;
                self.last_index = self.val.len();
                return Some(&self.val[last_index..self.val.len()]);
            } else {
                // The characters are all handled, and the last slice was returned if no character is
                // searched, return `None`
                return None;
            }
        }
    }
}

/// Split a value while prevent splitting arbitrary values / variants and variant groups, by
/// ignoring values wrapped in parenthesis and brackets.
///
/// # Example
///
/// ```rust
/// use encre_css::utils::split_ignore_arbitrary;
///
/// let value = "bg-red-500 content-[wrapped in `[]`, will not be splitted] (words wrapped in parenthesis are not splitted too)";
/// assert_eq!(split_ignore_arbitrary(value, ' ').collect::<Vec<&str>>(), vec!["bg-red-500", "content-[wrapped in `[]`, will not be splitted]", "(words wrapped in parenthesis are not splitted too)"]);
/// ```
pub fn split_ignore_arbitrary<P: Pattern>(
    val: &str,
    searched_pattern: P,
) -> SplitIgnoreArbitrary<P> {
    SplitIgnoreArbitrary {
        val,
        iter: val.char_indices(),
        searched_pattern,
        is_next_escaped: false,
        parenthesis_level: 0,
        bracket_level: 0,
        last_index: 0,
        seek_index: 0,
    }
}
