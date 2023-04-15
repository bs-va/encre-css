//! A [`Plugin`] is a handler used to convert utility classes into CSS declarations.
//!
//! A lot of plugins are built in (like the ones from Tailwind CSS) and some others live in
//! their own crates and need to be imported manually. They usually define a `register` function taking
//! a mutable reference to a [`Config`] structure.
//!
//! # Example (with `encre-css-typography`)
//!
//! ```ignore
//! use encre_css::{Config, generate};
//!
//! # fn main() -> encre_css::Result<()> {
//! let mut config = Config::from_file("encre-css.toml")?;
//! // Or let mut config = Config::default();
//!
//! encre_css_typography::register(&mut config);
//!
//! let _css = generate([r#"<div class="prose prose-headings:text-blue-500 prose-slate lg:prose-lg dark:prose-invert"></div>"#], &config);
//! // Do something with the CSS
//! # Ok(())
//! # }
//! ```
//!
//! # Official plugins
//!
//! - [`encre-css-typography`](https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css-typography): used to define beautiful typographic defaults for HTML you don't control.
//! - [`encre-css-icons`](https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css-icons): used to quickly add pure CSS icons to your website.
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
/// Each plugin consists of two methods:
/// - [`Plugin::can_handle`] to check if it will be able to generate CSS for a specific modifier;
/// - [`Plugin::handle`] to generate the CSS needed.
///
/// The [`Plugin::can_handle`] method takes a [`ContextCanHandle`] structure containing the
/// modifier and the current configuration.
///
/// The [`Plugin::handle`] method takes a [`ContextHandle`] structure containing the modifier,
/// the current configuration and a buffer containing the whole CSS
/// currently generated. You can use the [`Buffer`] structure (especially the [`Buffer::line`]
/// and [`Buffer::lines`] functions) to push CSS declarations to it, they will be automatically
/// indented.
///
/// It is common to use the [`unreachable!`] macro if the [`Plugin::handle`] method cannot be
/// called because you are sure that [`Plugin::can_handle`] returned `false`.
///
/// # Example (defines the `stroke-width` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// #[derive(Debug)]
/// struct StrokeWidth;
///
/// impl Plugin for StrokeWidth {
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
///     fn handle(&self, context: &mut ContextHandle) {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 context.buffer.line(format_args!("stroke-width: {value}px;"));
///             }
///             Modifier::Arbitrary { value, .. } => {
///                 context.buffer.line(format_args!("stroke-width: {value};"));
///             }
///         }
///     }
/// }
/// ```
///
/// # Release a plugin as a crate
///
/// If you want to release your custom plugins as a crate, you can export a `register` function
/// taking a mutable reference to a [`Config`] structure and use the [`Config::register_plugin`]
/// function to register them. The first argument is the namespace prefixing all the
/// utility classes handled by the plugin.
///
/// ```ignore
/// pub fn register(config: &mut Config) {
///     config.register_plugin("stroke", &StrokeWidth);
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
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// #[derive(Debug)]
/// struct PluginDefinition;
///
/// impl Plugin for PluginDefinition {
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
///     fn handle(&self, context: &mut ContextHandle) {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 let animation = match *value {
///                     "none" => "none",
///                     "spin" => {
///                         context.buffer.lines([
///                             "@keyframes spin",
///                             "...",
///                         ]);
///                         "spin 1s linear infinite"
///                     }
///                     "ping" => {
///                         context.buffer.lines([
///                             "@keyframes ping",
///                             "...",
///                         ]);
///                         "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"
///                     }
///                     "pulse" => {
///                         context.buffer.lines([
///                             "@keyframes pulse",
///                             "...",
///                         ]);
///                         "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"
///                     }
///                     "bounce" => {
///                         context.buffer.lines([
///                             "@keyframes bounce",
///                             "...",
///                         ]);
///                         "bounce 1s infinite"
///                     }
///                     _ => unreachable!(),
///                 };
///
///                 generate_wrapper(context, |context| {
///                     context.buffer.line(format_args!("animation: {animation};"));
///                 })
///             }
///             Modifier::Arbitrary { value, .. } => generate_wrapper(context, |context| {
///                 context.buffer.line(format_args!("animation: {value};"));
///             }),
///         }
///     }
/// }
/// ```
///
/// Have a look at <https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css/src/plugins>
/// for more examples.
///
/// [`Buffer`]: crate::utils::buffer::Buffer
/// [`Buffer::line`]: crate::utils::buffer::Buffer::line
/// [`Buffer::lines`]: crate::utils::buffer::Buffer::lines
/// [`Config::register_plugin`]: crate::Config::register_plugin
/// [`Config`]: crate::Config
/// [`needs_wrapping`]: Plugin::needs_wrapping
/// [`generator::generate_at_rules`]: crate::generator::generate_at_rules
/// [`generator::generate_class`]: crate::generator::generate_class
/// [`generator::generate_wrapper`]: crate::generator::generate_wrapper
pub trait Plugin: fmt::Debug {
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
    /// Various notes:
    /// - The CSS returned should end with a newline;
    /// - Arbitrary values are already normalized (e.g. underscores are replaced by spaces);
    /// - This function is guaranteed to be called only once per selector.
    fn handle(&self, _context: &mut ContextHandle);
}
