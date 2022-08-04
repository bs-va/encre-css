//! A [`Plugin`] is a handler used to convert utility classes into CSS declarations.
//!
//! A lot of plugins are built in (like the ones from Tailwind CSS) and some others live in
//! their own crates and need to be imported manually. They usually define a `register` function taking
//! a mutable reference to a [`Config`] structure.
//!
//! # Example (with `encre-css-typography`)
//!
//! ```rust,ignore
//! use encre_css::{Config, EncreGenerator};
//!
//! let mut config = Config::from_file("encre-css.toml");
//! // Or let mut config = Config::default();
//! encre_css_typography::register(&mut config);
//!
//! let mut generator = EncreGenerator::from_config(config);
//! generator.scan(r#"<div class="prose prose-headings:text-blue-500 prose-slate lg:prose-lg dark:prose-invert"></div>"#);
//! let css = generator.generate();
//! // Do something with the CSS
//! ```
//!
//! # Official plugins
//!
//! - [`encre-css-typography`](https://gitlab.com/encre-css/encre-css/tree/main/crates/encre-css-typography): used to define beautiful typographic defaults for HTML you don't control.
//!
//! If you want to write your own plugins, see [`Plugin`].
//!
//! [`Config`]: crate::Config
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
///
/// Each plugin consists of three methods:
/// - [`Plugin::namespace`] (optional) to give a prefix to all modifiers;
/// - [`Plugin::can_handle`] to check if it will be able to generate CSS for a specific modifier;
/// - [`Plugin::handle`] to generate the CSS needed.
///
/// The [`Plugin::can_handle`] method takes a [`ContextCanHandle`] structure containing the
/// modifier and the current configuration.
///
/// The [`Plugin::handle`] method takes a [`ContextHandle`] structure containing the modifier,
/// the current configuration, the current indentation (each `handle` needs to take account of the
/// indentation using the [`utils::indent`] function) and a buffer containing the whole CSS
/// currently generated. You can use the [`writeln!`] macro to push CSS declarations to it (the CSS
/// pushed **should** end with a newline) and the [`fmt::Result`] can be discarded using `?`.
///
/// It is common to use the [`unreachable!`] macro if the [`Plugin::handle`] method cannot be
/// called because [`Plugin::can_handle`] returned `false`.
///
/// # Example (defines the `stroke-width` plugin)
///
/// ```rust
/// use encre_css::{
///    generator::{ContextCanHandle, ContextHandle},
///    plugins::Plugin,
///    selector::Modifier,
///    utils::{
///        indent,
///        value_matchers::{is_matching_length, is_matching_percentage},
///    },
/// };
/// use std::fmt::{self, Write};
///
/// #[derive(Debug)]
/// pub struct StrokeWidth;
///
/// impl Plugin for StrokeWidth {
///     fn namespace(&self) -> &str {
///         "stroke"
///     }
///
///     fn can_handle(&self, context: ContextCanHandle) -> bool {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
///             Modifier::Arbitrary { hint, value, .. } => {
///                 *hint == "length"
///                     || *hint == "number"
///                     || *hint == "percentage"
///                     || (hint.is_empty()
///                         && (is_matching_length(value) || is_matching_percentage(value)))
///             }
///         }
///     }
///
///     fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
///         indent(context.indentation, context.buffer)?;
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 writeln!(context.buffer, "stroke-width: {value}px;")?;
///             }
///             Modifier::Arbitrary { value, .. } => {
///                 writeln!(context.buffer, "stroke-width: {value};")?;
///             }
///         }
///
///         Ok(())
///     }
/// }
/// ```
///
/// # Release a plugin as a crate
///
/// If you want to release your custom plugins as a crate, you can export a `register` function
/// taking a mutable reference to a [`Config`] structure and use the [`Config::register_plugin`]
/// function to register them.
///
/// ```rust,ignore
/// pub fn register(config: &mut Config) {
///     config.register_plugin(&StrokeWidth);
/// }
/// ```
///
/// # More powerful usage
///
/// If you need to have full control over the CSS **rule**, you can create a [`needs_wrapping`]
/// method returning false and use [`generator::generate_at_rules`], [`generator::generate_class`]
/// and [`generator::generate_wrapper`] to generate some CSS boilerplate.
///
/// ### Example (roughly defines the `animation` plugin)
///
/// ```rust
/// # use encre_css::{
/// #     generator::generate_wrapper,
/// #     generator::{ContextCanHandle, ContextHandle},
/// #     plugins::Plugin,
/// #     selector::Modifier,
/// #     utils::{indent, value_matchers::is_matching_all},
/// # };
/// # use std::{fmt::{self, Write}};
/// #[derive(Debug)]
/// pub(crate) struct PluginDefinition;
///
/// impl Plugin for PluginDefinition {
///     fn namespace(&self) -> &str {
///         "animate"
///     }
///
///     fn needs_wrapping(&self) -> bool {
///         false
///     }
///
///     fn can_handle(&self, context: ContextCanHandle) -> bool {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 ["spin", "ping", "pulse", "bounce", "none"].contains(value)
///             }
///             Modifier::Arbitrary { value, .. } => is_matching_all(value),
///         }
///     }
///
///     fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 let animation = match *value {
///                     "none" => "none",
///                     "spin" => {
///                         writeln!(context.buffer, "@keyframes spin...")?;
///                         "spin 1s linear infinite"
///                     }
///                     "ping" => {
///                         writeln!(context.buffer, "@keyframes ping...")?;
///                         "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"
///                     }
///                     "pulse" => {
///                         writeln!(context.buffer, "@keyframes pulse...")?;
///                         "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"
///                     }
///                     "bounce" => {
///                         writeln!(context.buffer, "@keyframes bounce...")?;
///                         "bounce 1s infinite"
///                     }
///                     _ => unreachable!(),
///                 };
///
///                 generate_wrapper(context, |context| {
///                     indent(context.indentation, context.buffer)?;
///                     writeln!(context.buffer, "animation: {animation};")
///                 })
///             }
///             Modifier::Arbitrary { value, .. } => generate_wrapper(context, |context| {
///                 indent(context.indentation, context.buffer)?;
///                 writeln!(context.buffer, "animation: {value};")
///             }),
///         }
///     }
/// }
/// ```
///
/// Have a look at <https://gitlab.com/encre-css/encre-css/tree/main/crates/encre-css/src/plugins>
/// for more examples.
///
/// [`utils::indent`]: crate::utils::indent
/// [`writeln!`]: std::writeln
/// [`Config::register_plugin`]: crate::Config::register_plugin
/// [`Config`]: crate::Config
/// [`needs_wrapping`]: Plugin::needs_wrapping
/// [`generator::generate_at_rules`]: crate::generator::generate_at_rules
/// [`generator::generate_class`]: crate::generator::generate_class
/// [`generator::generate_wrapper`]: crate::generator::generate_wrapper
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
    ///
    /// Note: arbitrary values are already normalized (e.g. underscores are replaced by spaces).
    ///
    /// # Errors
    ///
    /// Returns [`fmt::Result`] if writing to the buffer failed.
    fn handle(&self, _context: &mut ContextHandle) -> fmt::Result;
}
