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

pub trait Plugin: fmt::Debug {
    /// Returns the namespace containing the plugin
    ///
    /// By default, the plugin does not belong to a namespace
    fn namespace(&self) -> &str {
        ""
    }

    /// Returns whether the plugin can handle a specific arbitrary value
    ///
    /// Used to distinguish plugins inside the same namespace
    ///
    /// By default, arbitrary values are disallowed
    ///
    /// The `hint` argument can be ignored, for example if the namespace contains a single plugin
    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        false
    }

    /// Get the template for an arbitrary associated with the plugin
    ///
    /// Returns whether the function handled the modifier
    ///
    /// NOTE: This function is called after [to_css_value], so, `_` (underscores) are already converted to ` ` (spaces)
    ///
    /// [to_css_value]: crate::generator::to_css_value
    fn css_template_value(&self, _val: &str, _css_content: &mut String) -> bool {
        false
    }

    /// Get the CSS code from a modifier
    ///
    /// Returns whether the function handled the modifier
    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        custom_css: &mut String,
    ) -> bool;
}
