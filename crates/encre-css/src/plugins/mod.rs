use crate::{config::Config, selector::Modifier};

use std::{borrow::Cow, fmt};

pub mod accessibility;
pub mod alignment;
pub mod background;
pub mod border;
pub mod effect;
pub mod filter;
pub mod flexbox;
pub mod grid;
pub mod interactivity;
pub mod layout;
pub mod sizing;
pub mod spacing;
pub mod svg;
pub mod table;
pub mod transform;
pub mod transition;
pub mod typography;

const WILL_BE_REPLACED_BY_UNDERSCORE: &str = "WILL-BE-REPLACED-BY-UNDERSCORE";

pub trait Plugin: fmt::Debug {
    /// Returns the namespace containing the plugin
    ///
    /// By default, the plugin does not belong to a namespace
    fn namespace(&self) -> &str {
        ""
    }

    /// Custom CSS written before the CSS rule
    ///
    /// NOTE: The CSS must end with two newlines
    fn css_before_rule(&self, _modifier: &Modifier, _buffer: &mut String) -> fmt::Result {
        Ok(())
    }

    /// Returns whether the plugin can handle a specific modifier
    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool;

    /// Get the CSS code from a modifier
    ///
    /// The CSS should end with a newline
    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result;
}

/// Convert an arbitrary value into a CSS value
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s or if prefixed by a backslash)
///  - Spaces are added around operators in the `calc` CSS function
pub fn to_css_value(value: &str) -> Cow<str> {
    let mut value = Cow::from(value);

    // Don't replace `_` if it is a URL
    if value.contains("url") {
        // For the `CursorPlugin`, `ContentPlugin` and `ImagePlugin` plugins, we need to keep underscores in URLs
        value = Cow::from(value.replace('_', WILL_BE_REPLACED_BY_UNDERSCORE));
    }

    // Replace `_` with ` ` (spaces) (if not prefixed by a backslash)
    if value.contains("\\_") {
        value = Cow::from(value.replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE));
    }

    if value.contains('_') {
        value = Cow::from(value.replace('_', " "));
    }

    if value.contains(WILL_BE_REPLACED_BY_UNDERSCORE) {
        value = Cow::from(value.replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_"));
    }

    // Add spaces around operators in the `calc` CSS function
    if value.contains("calc") {
        value = Cow::from(
            value
                .split(' ')
                .map(|v| {
                    if v.starts_with("calc(") {
                        Cow::from(
                            v.replace('-', " - ")
                                .replace('+', " + ")
                                .replace('/', " / ")
                                .replace('*', " * "),
                        )
                    } else {
                        Cow::from(v)
                    }
                })
                .collect::<Vec<Cow<str>>>()
                .join(" "),
        )
    }

    value
}
