//! Define a structure used to scan content.
use std::collections::BTreeSet;

/// A structure responsible for scanning some content and returning a list of possible classes.
///
/// By default, it splits the content by spaces, double quotes, single quotes and backticks.
///
/// # Example
///
/// ```rust
/// use encre_css::{EncreGenerator, Config, Extractor};
/// use std::collections::BTreeSet;
///
/// let mut config = Config::default();
/// config.extractor = Extractor::from_fn(|content| content.split(r#"data-en=""#)
///     .filter_map(|v| v.split_once("\"").map(|(classes, _)| classes.split_whitespace()))
///     .flatten()
///     .collect::<BTreeSet<&str>>());
///
/// let mut generator = EncreGenerator::from_config(config);
/// generator.scan(r#"<h1 data-en="underline"></h1><p data-en="bg-red-200 text-blue-300"></p>"#);
///
/// assert!(generator.generate().expect("failed to generate the CSS").contains(r#"
/// .bg-red-200 {
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
#[allow(missing_debug_implementations)]
pub struct Extractor {
    extract_fn: Box<dyn Fn(&str) -> BTreeSet<&str> + Send + Sync>,
}

impl Extractor {
    /// Build an [`Extractor`] from a closure taking some content and returning a list of possible
    /// classes.
    pub fn from_fn<T: 'static + Fn(&str) -> BTreeSet<&str> + Send + Sync>(extract_fn: T) -> Self {
        Self {
            extract_fn: Box::new(extract_fn),
        }
    }

    pub(crate) fn extract<'a>(&self, val: &'a str) -> BTreeSet<&'a str> {
        (self.extract_fn)(val)
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self {
            extract_fn: Box::new(|val| {
                val.split(|ch| ch == ' ' || ch == '"' || ch == '\'' || ch == '`')
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
    fn default_extractor_test() {
        assert_eq!(
            Extractor::default().extract("test bg-red-500 'hello'"),
            BTreeSet::from(["", "test", "bg-red-500", "hello"])
        );
    }

    #[test]
    fn custom_extractor_test() {
        let extractor =
            Extractor::from_fn(|val| val.split(|ch| ch == '|').collect::<BTreeSet<&str>>());

        assert_eq!(
            extractor.extract("test|bg-red-500|'hello'"),
            BTreeSet::from(["test", "bg-red-500", "'hello'"])
        );
    }
}
