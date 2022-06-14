use crate::{config::Config, selector::Modifier};

use std::fmt;

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

pub trait Plugin {
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
