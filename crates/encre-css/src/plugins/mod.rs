//! Define the [`Plugin`] trait used to generate styles from classes.
use crate::context::{ContextAfterRule, ContextBeforeRule, ContextCanHandle, ContextHandle};

use std::{borrow::Cow, fmt};

pub mod accessibility;
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

/// A plugin is a structure capable of generating CSS styles from a modifier (contained in a
/// context structure).
pub trait Plugin: fmt::Debug {
    /// Returns the namespace containing the plugin.
    ///
    /// By default, the plugin does not belong to a namespace.
    fn namespace(&self) -> &str {
        ""
    }

    /// Returns whether the plugin can handle a specific modifier.
    fn can_handle(&self, _context: ContextCanHandle) -> bool;

    /// Custom CSS written before the CSS rule.
    ///
    /// Note: the CSS must end with two newlines.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Format`] if writing to the buffer failed.
    ///
    /// [`Error::Format`]: crate::Error::Format
    fn css_before_rule(&self, _context: ContextBeforeRule) -> fmt::Result {
        Ok(())
    }

    /// Custom CSS written after the CSS rule.
    ///
    /// Note: the CSS must start with two newlines.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Format`] if writing to the buffer failed.
    ///
    /// [`Error::Format`]: crate::Error::Format
    fn css_after_rule(&self, _context: ContextAfterRule) -> fmt::Result {
        Ok(())
    }

    /// Get the CSS code from a modifier.
    ///
    /// The [`Plugin::can_handle`] method **must be** called before to know if it can handle
    /// the modifier, otherwise this function **will panic**.
    ///
    /// Note: the CSS should end with a newline.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Format`] if writing to the buffer failed.
    ///
    /// [`Error::Format`]: crate::Error::Format
    fn handle(&self, _context: ContextHandle) -> fmt::Result;
}

/// Convert an arbitrary value into a CSS value.
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s or if prefixed by a backslash);
///  - Spaces are added around operators in the `calc` CSS function.
pub(crate) fn to_css_value(value: &str) -> Cow<str> {
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
        );
    }

    value
}
