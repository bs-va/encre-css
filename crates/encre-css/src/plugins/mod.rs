//! Define the [`Plugin`] trait used to generate styles from classes.
use crate::generator::{ContextCanHandle, ContextHandle};

use std::fmt;

pub mod accessibility;
pub mod background;
pub mod border;
pub mod css_property;
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

    /// Returns whether the plugin should be wrapped inside a CSS rule or if it will manually
    /// generate it
    fn needs_wrapping(&self) -> bool {
        true
    }

    /// Get the CSS code from a modifier.
    ///
    ///
    /// The [`Plugin::can_handle`] method **must be** called before to know if it can handle
    /// the modifier, otherwise this function **will panic**.
    ///
    /// Note: the CSS should end with a newline.
    /// Note: arbitrary values are already normalized (e.g. underscores are replaced by spaces).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Format`] if writing to the buffer failed.
    ///
    /// [`Error::Format`]: crate::Error::Format
    fn handle(&self, _context: &mut ContextHandle) -> fmt::Result;
}
