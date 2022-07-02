//! Define the [`Selector`] structure used to parse scanned classes.
//!
//! ### Some vocabulary
//!
//! <p style="font-family: sans-serif; color: black;"><b><span style="padding: 0.1rem 0 0.1rem 0.2rem; background-color: hsl(45, 100%, 85%);">hover:xl</span><span style="padding: 0.1rem 0; background-image: linear-gradient(to right, hsl(45, 100%, 85%), hsl(180, 100%, 85%));">:</span><span style="padding: 0.1rem 0; background-color: hsl(180, 100%, 85%);">bg</span><span style="padding: 0.1rem 0; background-image: linear-gradient(to right, hsl(180, 100%, 85%), hsl(315, 100%, 85%));">-</span><span style="padding: 0.1rem 0.2rem 0.1rem 0; background-color: hsl(315, 100%, 85%);">red-500</span></b></p>
//!
//! 1. The <a href="../variant/index.html" style="text-decoration: underline; text-underline-offset: 2px; text-decoration-color: black;">
//!    <span style="background-color: hsl(45, 100%, 85%); color: black; padding: 0 0.2rem;"><b>variants</b></span></a>
//!    (used to add pseudo-selectors, pseudo-elements, pseudo classes, media queries), in this case
//!    the class will be applied only on a screen larger than 1280px (see [`BUILTIN_SCREENS`]) and
//!    if hovered;
//! 2. The <span style="background-color: hsl(180, 100%, 85%); color: black; padding: 0 0.2rem;"><b>namespace</b></span> (basically the name of the plugin), in this case `bg` for changing the background;
//! 3. The <a href="enum.Modifier.html" style="text-decoration: underline; text-underline-offset: 2px; text-decoration-color: black;"><span style="background-color: hsl(315, 100%, 85%); color: black; padding: 0 0.2rem;"><b>modifier</b></span></a> (used to clarify the CSS needed to be generated), in this case the
//!       background color will become `rgb(239 68 68)` (see [`BUILTIN_COLORS`]).
//!
//! <p style="font-family: sans-serif; color: black;"><b><span style="padding: 0.1rem 0 0.1rem 0.2rem; background-color: hsl(180, 100%, 85%);">bg</span><span style="padding: 0.1rem 0; background-image: linear-gradient(to right, hsl(180, 100%, 85%), hsl(135, 100%, 85%));">-</span><span style="padding: 0.1rem 0.2rem 0.1rem 0; background-color: hsl(135, 100%, 85%);">[rgb(12_12_12)]</span></b></p>
//!
//! 4. The <a href="enum.Modifier.html#variant.Arbitrary" style="text-decoration: underline; text-underline-offset: 2px; text-decoration-color: black;"><span style="background-color: hsl(135, 100%, 85%); color: black; padding: 0 0.2rem;"><b>arbitrary value</b></span></a>
//!    (used to specify a value not included in your design system), in this case the background
//!    color will become `rgb(12 12 12)` (spaces need to be replaced with underscores in arbitrary
//!    values).
//!
//! As you can see, by default variants are separated by `:`, modifiers by `-` and arbitrary values
//! are surrounded by `[]`.
//!
//! [`BUILTIN_SCREENS`]: crate::config::BUILTIN_SCREENS
//! [`BUILTIN_COLORS`]: crate::config::BUILTIN_COLORS
use crate::{
    config::{Config, BUILTIN_PLUGINS, BUILTIN_VARIANTS},
    context::ContextCanHandle,
    plugins::Plugin,
    variant::Variant,
};

use std::{borrow::Cow, cmp::Ordering, collections::BTreeMap};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const VALID_PLUGIN_HINT: [&str; 13] = [
    "color",
    "length",
    "line-width",
    "image",
    "url",
    "position",
    "percentage",
    "number",
    "generic-name",
    "family-name",
    "absolute-size",
    "relative-size",
    "shadow",
];

pub(crate) const VARIANT_SEPARATOR: char = ':';
const MODIFIER_SEPARATOR: char = '-';
const ARBITRARY_SEPARATOR_START: char = '[';
const ARBITRARY_SEPARATOR_END: char = ']';
const HINT_SEPARATOR: char = ':';

/// The modifier is the rest of the selector after the namespace, it is used to clarify the
/// CSS needed to be generated.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Modifier<'a> {
    /// A builtin static modifier (e.g. `bg-red-500`).
    Builtin {
        /// Whether the value is negative (e.g. `-translate-2` is negative).
        is_negative: bool,

        /// The inner value of the modifier.
        value: &'a str,
    },

    /// A dynamic modifier capable of automatically generating a rule from a CSS value
    /// (e.g. `bg-[rgb(12_12_12)]`).
    ///
    /// Sometimes the value is ambiguous, for example `bg-[var(--foo)]` can be handled by either
    /// the [`background color`](crate::plugins::background::background_color) or the
    /// [`background size`](crate::plugins::background::background_size) utility. In this case,
    /// you need to provide a [CSS type](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types)
    /// hint (see the list of hints below) before the arbitrary value. For example
    /// `bg-[length:var(--foo)]` will generate `background-size: var(--foo);` (using the
    /// [`background size`] utility).
    ///
    /// List of all type hints:
    /// - `color`
    /// - `length`
    /// - `line-width`
    /// - `image`
    /// - `url`
    /// - `position`
    /// - `percentage`
    /// - `number`
    /// - `generic-name`
    /// - `family-name`
    /// - `absolute-size`
    /// - `relative-size`
    /// - `shadow`
    Arbitrary {
        /// The rest of the modifier without the arbitrary value (e.g. `bg` in `bg-[rgb(12_12_12)]`)
        prefix: &'a str,

        /// The type hint needed for ambiguous values
        hint: &'a str,

        /// The inner value of the modifier
        value: &'a str,
    },
}

/// A selector is a full class, containing the variants, the namespace and the modifier.
///
/// See [`crate::selector`] for more informations.
#[derive(Clone, Debug)]
pub struct Selector<'a> {
    pub(crate) order: usize,
    pub(crate) full: &'a str,
    pub(crate) modifier: Modifier<'a>,
    pub(crate) variants: &'a str,
    pub(crate) is_important: bool,
    pub(crate) plugin: &'static (dyn Plugin + Sync + Send),
}

impl<'a> Selector<'a> {
    pub(crate) fn new(full: &'a str, config: &Config) -> Option<Self> {
        // We need to ignore all characters in arbitrary values (wrapped in `[]`) and we know that
        // nothing interesting is placed after them, so we can just split by `[` and take the first
        // value
        let variants = {
            let before_arbitrary = full.split('[').next()?;
            &before_arbitrary.get(..before_arbitrary.rfind(VARIANT_SEPARATOR).unwrap_or(0))?
        };

        // The rest of the selector is the namespace and the modifier
        let mut rest = if variants.is_empty() {
            full
        } else {
            full.strip_prefix(variants)?
                .strip_prefix(VARIANT_SEPARATOR)?
        };

        // Strip the important flag (must be before the negative one)
        let is_important = if rest.starts_with('!') {
            rest = &rest[1..];
            true
        } else {
            false
        };

        // Strip the negative flag
        let is_negative = if rest.starts_with('-') {
            rest = &rest[1..];
            true
        } else {
            false
        };

        // Find the right plugin for handling this selector
        let find_fn = |(i, plugin): (usize, &&'static (dyn Plugin + Send + Sync))| {
            // Find the modifier
            if let Some(modifier_part) = rest.strip_prefix(&plugin.namespace()) {
                let modifier_part = modifier_part
                    .strip_prefix(MODIFIER_SEPARATOR)
                    .unwrap_or(modifier_part);

                let modifier = if let Some((mut prefix, mut after)) =
                    modifier_part.split_once(ARBITRARY_SEPARATOR_START)
                {
                    prefix = prefix.strip_suffix(MODIFIER_SEPARATOR).unwrap_or(prefix);
                    after = after.strip_suffix(ARBITRARY_SEPARATOR_END)?;

                    if let Some((maybe_hint, rest)) = after.split_once(HINT_SEPARATOR) {
                        if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                            Modifier::Arbitrary {
                                prefix,
                                hint: maybe_hint,
                                value: rest,
                            }
                        } else {
                            Modifier::Arbitrary {
                                prefix,
                                hint: "",
                                value: after,
                            }
                        }
                    } else {
                        Modifier::Arbitrary {
                            prefix,
                            hint: "",
                            value: after,
                        }
                    }
                } else {
                    Modifier::Builtin {
                        is_negative,
                        value: modifier_part,
                    }
                };

                let context = ContextCanHandle {
                    config,
                    modifier: &modifier,
                };

                if plugin.can_handle(context) {
                    Some((i, *plugin, modifier))
                } else {
                    None
                }
            } else {
                None
            }
        };

        #[cfg(not(feature = "rayon"))]
        let result = BUILTIN_PLUGINS.iter().enumerate().find_map(find_fn);

        #[cfg(feature = "rayon")]
        let result = BUILTIN_PLUGINS
            .par_iter()
            .enumerate()
            .find_map_first(find_fn);

        if let Some((order, plugin, modifier)) = result {
            Some(Self {
                order,
                full,
                modifier,
                variants,
                is_important,
                plugin,
            })
        } else {
            None
        }
    }

    pub(crate) fn get_css_class(&self, custom_variants: &BTreeMap<Cow<str>, Variant>) -> String {
        let mut base_class = ".".to_string()
            + &self
                .full
                .chars()
                .enumerate()
                .map(|(i, ch)| {
                    if i == 0 {
                        if ch.is_numeric() {
                            // CSS classes must not start with a number, we need to escape it
                            "\\3".to_string() + &ch.to_string()
                        } else {
                            ch.to_string()
                        }
                    } else if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
                        format!("\\{}", ch)
                    } else {
                        ch.to_string()
                    }
                })
                .collect::<String>();

        if !self.variants.is_empty() {
            self.variants
                .split(VARIANT_SEPARATOR)
                .rev()
                .for_each(|variant| {
                    if let Some(Variant::WrapClass(template)) = BUILTIN_VARIANTS
                        .iter()
                        .find_map(|v| if v.0 == variant { Some(&v.1) } else { None })
                        .or_else(|| custom_variants.get(&Cow::from(variant)))
                    {
                        base_class = template.replace('&', &base_class);
                    }
                });
        }

        base_class
    }
}

impl<'a> PartialEq for Selector<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.full == other.full
    }
}

impl<'a> Eq for Selector<'a> {}

impl<'a> PartialOrd for Selector<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Selector<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.variants.is_empty() && !other.variants.is_empty() {
            Ordering::Less
        } else if !self.variants.is_empty() && other.variants.is_empty() {
            Ordering::Greater
        } else {
            self.order
                .cmp(&other.order)
                .then_with(|| self.full.cmp(other.full))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, selector::Selector};

    use std::collections::BTreeSet;

    #[test]
    fn sorting_test() {
        let config = Config::default();
        let mut selectors = BTreeSet::new();
        selectors.insert(Selector::new("lg:bg-red-500", &config).unwrap());
        selectors.insert(Selector::new("bg-red-500", &config).unwrap());

        assert_eq!(
            selectors.iter().collect::<Vec<&Selector>>(),
            vec![
                &Selector::new("bg-red-500", &config).unwrap(),
                &Selector::new("lg:bg-red-500", &config).unwrap(),
            ]
        );
    }
}
