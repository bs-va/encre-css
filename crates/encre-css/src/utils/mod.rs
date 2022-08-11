//! Define some utility functions for quickly doing things.
use crate::{
    config::Config,
    selector::{
        parser::{parse, ARBITRARY_END, ARBITRARY_START, ESCAPE, GROUP_END, GROUP_START},
        Selector,
    },
};

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
    ignore_parenthesis: bool,
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
                    GROUP_START if self.ignore_parenthesis => self.parenthesis_level += 1,
                    GROUP_END if self.ignore_parenthesis => {
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
                            && self.bracket_level == 0
                            && !(self.ignore_parenthesis && self.parenthesis_level > 0)
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
/// ignoring values wrapped in brackets.
///
/// The last argument indicates whether parenthesis are also ignored.
///
/// # Example
///
/// ```rust
/// use encre_css::utils::split_ignore_arbitrary;
///
/// let value = "bg-red-500 content-[wrapped in `[]`, will not be split] (words wrapped in parenthesis are not split too)";
/// assert_eq!(split_ignore_arbitrary(value, ' ', true).collect::<Vec<&str>>(), vec!["bg-red-500", "content-[wrapped in `[]`, will not be split]", "(words wrapped in parenthesis are not split too)"]);
/// ```
pub fn split_ignore_arbitrary<P: Pattern>(
    val: &str,
    searched_pattern: P,
    ignore_parenthesis: bool,
) -> impl Iterator<Item = &str> {
    SplitIgnoreArbitrary {
        val,
        iter: val.char_indices(),
        searched_pattern,
        ignore_parenthesis,
        is_next_escaped: false,
        parenthesis_level: 0,
        bracket_level: 0,
        last_index: 0,
        seek_index: 0,
    }
}

/// Sort a list of selectors (separated by spaces) according to `encre-css` rules.
///
/// Note: selectors are also deduplicated.
pub fn sort_selectors(val: &str, config: &Config) -> String {
    let mut selectors = val
        .split_whitespace()
        .filter_map(|v| parse(v.trim(), config))
        .flatten()
        .collect::<Vec<Selector>>();

    // Deduplicate selectors belonging to a variant group
    selectors.sort_unstable();
    selectors.dedup_by_key(|s| s.full);

    selectors
        .iter()
        .map(|s| s.full)
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_selectors_test() {
        assert_eq!(
            sort_selectors(
                "text-white px-4 sm:px-8 py-2 sm:py-3 bg-sky-700 hover:bg-sky-800",
                &Config::default()
            ),
            "bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3".to_string()
        );
    }

    #[test]
    fn sort_selectors_with_variant_groups() {
        assert_eq!(
            sort_selectors(
                "hover:(text-white,bg-sky-800) focus-within:bg-red-100 text-blue-500 md:flex",
                &Config::default()
            ),
            "text-blue-500 focus-within:bg-red-100 hover:(text-white,bg-sky-800) md:flex"
                .to_string()
        );
    }

    #[test]
    fn sort_selectors_selectors_are_deduplicated() {
        assert_eq!(sort_selectors("text-blue-100 text-blue-100 md:flex lg:block md:flex focus:(hover:md:flex,lg:flex)", &Config::default()), "text-blue-100 focus:(hover:md:flex,lg:flex) md:flex focus:(hover:md:flex,lg:flex) lg:block".to_string());
    }
}
