//! Define a structure used to scan content.
use std::collections::BTreeSet;

use crate::utils::split_ignore_arbitrary;

/// A structure responsible for scanning some content and returning a list of possible classes.
///
/// By default, it splits the content by spaces, double quotes, single quotes and backticks and
/// ignores arbitrary values / variants and variant groups by using [`split_ignore_arbitrary`].
/// It is recommended to use this function when splitting classes with characters which can be
/// included inside arbitrary strings.
///
/// # Example
///
/// The following code snippet defines a scanner for extracting classes listed in the `data-en`
/// HTML attribute.
///
/// ```rust
/// use encre_css::{EncreGenerator, Config, Scanner, utils::split_ignore_arbitrary};
/// use std::collections::BTreeSet;
///
/// let mut config = Config::default();
/// config.scanner = Scanner::from_fn(|content| content.split(r#"data-en=""#)
///     .filter_map(|v| v.split_once("\"").map(|(classes, _)| classes.split_whitespace()))
///     .flatten()
///     .collect::<BTreeSet<&str>>());
///
/// let mut generator = EncreGenerator::from_config(config);
/// generator.scan(r#"<h1 data-en="underline"></h1><p data-en="bg-red-200 text-blue-300"></p>"#);
///
/// assert!(generator.generate().expect("failed to generate the CSS").contains(r#".bg-red-200 {
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
/// }"#));
/// ```
///
/// [`utils::split_ignore_arbitray`]: crate::utils::split_ignore_arbitrary
#[allow(missing_debug_implementations)]
pub struct Scanner {
    scan_fn: Box<dyn Fn(&str) -> BTreeSet<&str> + Send + Sync>,
}

impl Scanner {
    /// Build an [`Scanner`] from a closure taking some content and returning a list of possible
    /// classes.
    pub fn from_fn<T: 'static + Fn(&str) -> BTreeSet<&str> + Send + Sync>(scan_fn: T) -> Self {
        Self {
            scan_fn: Box::new(scan_fn),
        }
    }

    pub(crate) fn scan<'a>(&self, val: &'a str) -> BTreeSet<&'a str> {
        (self.scan_fn)(val)
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            scan_fn: Box::new(|val| {
                split_ignore_arbitrary(val, |ch| ch == ' ' || ch == '"' || ch == '\'' || ch == '`')
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
            Scanner::default().scan("test bg-red-500 'hello'"),
            BTreeSet::from(["", "test", "bg-red-500", "hello"])
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
}
