//! Define several structures used to provide context to plugin handlers.
use super::{
    config::Config,
    selector::{Modifier, Selector},
    variant::Variant,
};

use std::{borrow::Cow, collections::BTreeMap};

/// The context used in the [`Plugin::can_handle`] method.
///
/// [`Plugin::can_handle`]: crate::plugins::Plugin::can_handle
#[derive(Debug)]
pub struct ContextCanHandle<'a, 'b, 'c> {
    /// The generator's configuration
    pub config: &'a Config,

    /// The modifier which will be checked
    pub modifier: &'b Modifier<'c>,
}

/// The context used in the [`Plugin::handle`] method.
///
/// [`Plugin::handle`]: crate::plugins::Plugin::handle
#[derive(Debug)]
pub struct ContextHandle<'a, 'b, 'c, 'd> {
    /// The generator's configuration
    pub config: &'a Config,

    /// The modifier which will have its CSS generated
    pub modifier: &'b Modifier<'c>,

    /// The current indentation of the CSS rule
    pub indentation: usize,

    /// The buffer containing the whole generated CSS
    pub buffer: &'d mut String,
}

/// The context used in the [`Plugin::css_before_rule`] method.
///
/// [`Plugin::css_before_rule`]: crate::plugins::Plugin::css_before_rule
#[derive(Debug)]
pub struct ContextBeforeRule<'a, 'b, 'c, 'd> {
    /// The generator's configuration
    pub config: &'a Config,

    /// The selector which was checked (and is correct) and will have its CSS generated
    pub selector: &'b Selector<'c>,

    /// The buffer containing the whole generated CSS
    pub buffer: &'d mut String,
}

/// The context used in the [`Plugin::css_after_rule`] method.
///
/// [`Plugin::css_after_rule`]: crate::plugins::Plugin::css_after_rule
#[derive(Debug)]
pub struct ContextAfterRule<'a, 'b, 'c, 'd, 'e, 'f> {
    /// The generator's configuration
    pub config: &'a Config,

    /// The selector which was checked (and is correct) and will have its CSS generated
    pub selector: &'b Selector<'c>,

    /// The buffer containing the whole generated CSS
    pub buffer: &'d mut String,

    /// The list of all custom variants derived from the configuration
    pub custom_variants: &'e BTreeMap<Cow<'f, str>, Variant>,
}
