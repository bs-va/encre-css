//! Define a structure used to scan content.
use std::{collections::BTreeSet, sync::Arc};

/// A structure responsible for scanning some content and returning a list of possible classes.
///
/// By default, it splits the content by spaces, double quotes, single quotes, backticks and new
/// lines, while ignoring arbitrary the content inside values/variants and variant groups
/// by using [`split_ignore_arbitrary`].
/// It is recommended to use this function when splitting classes with characters which can be
/// included inside arbitrary strings.
///
/// # Example
///
/// The following code snippet defines a scanner for extracting classes listed in the `data-en`
/// HTML attribute.
///
/// ```
/// use encre_css::{Config, Scanner, utils::split_ignore_arbitrary};
/// use std::collections::BTreeSet;
///
/// let mut config = Config::default();
/// config.scanner = Scanner::from_fn(|content| content.split(r#"data-en=""#)
///     .filter_map(|v| v.split_once("\"").map(|(classes, _)| classes.split_whitespace()))
///     .flatten()
///     .collect::<BTreeSet<&str>>());
///
/// let generated = encre_css::generate(
///     [r#"<h1 data-en="underline"></h1><p data-en="bg-red-200 text-blue-300"></p>"#],
///     &config,
/// );
///
/// assert!(generated.ends_with(".bg-red-200 {
///   --en-bg-opacity: 1;
///   background-color: rgb(254 202 202 / var(--en-bg-opacity));
/// }
///
/// .text-blue-300 {
///   --en-text-opacity: 1;
///   color: rgb(147 197 253 / var(--en-text-opacity));
/// }
///
/// .underline {
///   -webkit-text-decoration-line: underline;
///   text-decoration-line: underline;
/// }"));
/// ```
///
/// [`split_ignore_arbitrary`]: crate::utils::split_ignore_arbitrary
#[allow(missing_debug_implementations)]
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct Scanner {
    scan_fn: Arc<dyn Fn(&str) -> BTreeSet<&str> + Send + Sync>,
}

impl Scanner {
    /// Build an [`Scanner`] from a closure taking some content and returning a list of possible
    /// classes.
    pub fn from_fn<T: 'static + Fn(&str) -> BTreeSet<&str> + Send + Sync>(scan_fn: T) -> Self {
        Self {
            scan_fn: Arc::new(scan_fn),
        }
    }

    pub(crate) fn scan<'a>(&self, val: &'a str) -> BTreeSet<&'a str> {
        (self.scan_fn)(val)
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            scan_fn: Arc::new(|val| {
                let mut is_dashed = false;
                let mut is_arbitrary = false;

                val.split(|ch| {
                    // Escape all characters in arbitrary values prefixed by a dash (used to avoid
                    // ignoring values in, for example, JS arrays, given that they are defined
                    // using square brackets)
                    match ch {
                        '-' => {
                            is_dashed = true;
                            false
                        }
                        '[' if is_dashed => {
                            is_arbitrary = true;
                            is_dashed = false;
                            false
                        }
                        ']' => {
                            is_arbitrary = false;
                            is_dashed = false;
                            false
                        }
                        _ => {
                            is_dashed = false;
                            ch == ' '
                                || (!is_arbitrary
                                    && (ch == '\'' || ch == '"' || ch == '`' || ch == '\n'))
                        }
                    }
                })
                .collect::<BTreeSet<&str>>()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    #[test]
    fn default_scanner_test() {
        assert_eq!(
            Scanner::default().scan("test bg-red-500 'hello' content-[some_[_square]_brackets] foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"),
            BTreeSet::from([
                "",
                "test",
                "bg-red-500",
                "hello",
                "content-[some_[_square]_brackets]",
                "foo-bar",
                "sm:focus:ring",
                "hover:bg-black",
                "border-[#333]",
                "text-[color:var(--hello)]",
            ])
        );
    }

    #[test]
    fn custom_scanner_test() {
        let scanner = Scanner::from_fn(|val| val.split(|ch| ch == '|').collect::<BTreeSet<&str>>());

        assert_eq!(
            scanner.scan("test|bg-red-500|'hello'"),
            BTreeSet::from(["test", "bg-red-500", "'hello'"])
        );
    }

    #[test]
    fn utf8_scan() {
        assert_eq!(
            Scanner::default().scan("<div class=\"before:content-[J\u{e4}s\u{f8}n_Doe] content-[\u{2192}]\">\u{306}</div>"),
            BTreeSet::from([
                "<div",
                ">\u{306}</div>",
                "before:content-[J\u{e4}s\u{f8}n_Doe]",
                "class=",
                "content-[\u{2192}]",
            ])
        );
    }

    #[test]
    fn scan_prevent_splitting_arbitrary_values() {
        assert_eq!(
            Scanner::default().scan(r#"<div class="bg-red-300 content-['hello:>"']"></div>"#),
            BTreeSet::from([
                "<div",
                "></div>",
                "bg-red-300",
                "class=",
                "content-['hello:>\"']",
            ])
        );
    }
}
