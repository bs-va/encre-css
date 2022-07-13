//! Define the structures used to parse scanned classes.
//!
//! ## Discover what is possible to do with classes by learning some vocabulary
//!
//! <style>
//! .with-hints {
//!   font-family: sans-serif;
//!   overflow: visible !important;
//!   height: 3rem;
//!   font-size: 1rem;
//! }
//!
//! .with-hints > b > span:nth-child(even) {
//!   padding: 0 0.35rem;
//! }
//!
//! .with-hints > b > span:nth-child(odd) {
//!   position: relative;
//!   text-decoration: underline;
//!   text-underline-offset: 6px;
//! }
//!
//! .with-hints span:nth-child(odd)::after {
//!   content: counter(hints);
//!   position: absolute;
//!   bottom: -1.75rem;
//!   left: 50%;
//!   transform: translateX(-50%);
//!   font-size: 0.7rem;
//!   border: 2px solid currentColor;
//!   border-radius: 50%;
//!   width: 1.1rem;
//!   height: 1.1rem;
//!   display: flex;
//!   justify-content: center;
//!   align-items: center;
//!   counter-increment: hints;
//! }
//! </style>
//!
//! <p class="with-hints" style="counter-reset: hints; margin-top: 2rem;"><b><span>hover:xl</span><span>:</span><span>bg</span><span>-</span><span>red-500</span></b></p>
//!
//! 1. The **[variants](crate::variant)** (used to add pseudo-selectors, pseudo-elements, pseudo classes, media queries), in this case
//!    the class will be applied only on a screen larger than 1280px (see [`BUILTIN_SCREENS`]) and
//!    if hovered;
//! 2. The **namespace** (basically the name of the plugin), in this case `bg` for changing the background;
//! 3. The **[modifier](crate::selector::Modifier)** (used to clarify the CSS needed to be generated), in this case the
//!       background color will become `rgb(239 68 68)` (see [`BUILTIN_COLORS`]).
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span style="counter-set: hints 3;">[&>*]</span><span>:</span><span style="counter-set: hints 1;">bg</span><span>-</span><span style="counter-set: hints 4;">[rgb(12_12_12)]</span></b></p>
//!
//! 4. The **arbitrary variant** (used to modify the class generated), in this case the class
//!    will be `.\[\&\>\*]\:bg-\[rgb\(12_12_12\)\]>*`.
//!
//! 5. The **[arbitrary value](crate::selector::Modifier::Arbitrary)**
//!    (used to specify a value not included in your design system), in this case the background
//!    color will become `rgb(12 12 12)` (spaces need to be replaced with underscores in arbitrary
//!    values).
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>[mask-type:luminance]</span></b></p>
//!
//! 6. The **arbitrary CSS property** (used to use a CSS property not supported by `encre-css`), in
//! this case the rule content will be `.\[mask-type\:luminance\] { mask-type: luminance; }`.
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>dark:(text-white,bg-gray-500)</span></b></p>
//!
//! 7. The **variant group** (used to group together several classes conditionally enabled by the
//!    same variant), in this case the class will be expanded to `dark:text-white` and
//!    `dark:bg-gray-500`.
//!
//! As you can see, by default variants are separated by `:`, modifiers by `-`, arbitrary
//! values/variants are surrounded by `[]` and variant groups are surrounded by `()`.
//!
//! [`BUILTIN_SCREENS`]: crate::config::BUILTIN_SCREENS
//! [`BUILTIN_COLORS`]: crate::config::BUILTIN_COLORS
pub(crate) mod parser;

use crate::plugins::Plugin;

use std::{borrow::Cow, cmp::Ordering};

pub(crate) use parser::parse;

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
    /// All underscores in the value will be replaced by spaces except in `url()`, if you really
    /// want to keep one of them, you can prefix it with a backslash `\_` and it will be used as
    /// is.
    ///
    /// Sometimes the value is ambiguous, for example `bg-[var(--foo)]` can be handled by either
    /// the [`background color`](crate::plugins::background::background_color) or the
    /// [`background size`](crate::plugins::background::background_size) utility. In this case,
    /// you need to provide a [CSS type](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types)
    /// hint (see the list of hints below) before the arbitrary value. For example
    /// `bg-[length:var(--foo)]` will generate `background-size: var(--foo);` (using the
    /// [`background size`](crate::plugins::background::background_size) utility).
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
        /// The rest of the modifier without the arbitrary value (e.g. `bg` in `bg-[rgb(12_12_12)]`).
        prefix: &'a str,

        /// The type hint needed for ambiguous values.
        hint: &'a str,

        /// The inner value of the modifier.
        ///
        /// All escaped characters (prefixed by a backslash) are already unescaped.
        value: Cow<'a, str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Variant<'a> {
    Builtin(&'a str),
    Arbitrary(Cow<'a, str>),
}

/// A parsed selector, aka a utility class, containing the variants, the namespace and the modifier.
///
/// See [`crate::selector`] for more informations.
#[derive(Clone, Debug)]
pub(crate) struct Selector<'a> {
    pub(crate) order: usize,
    pub(crate) full: &'a str,
    pub(crate) modifier: Modifier<'a>,
    pub(crate) variants: Vec<Variant<'a>>,
    pub(crate) is_important: bool,
    pub(crate) plugin: &'static (dyn Plugin + Sync + Send),
}

// Use a faster implementation when we are not testing
#[cfg(not(test))]
impl<'a> PartialEq for Selector<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.full == other.full
    }
}

#[cfg(test)]
impl<'a> PartialEq for Selector<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
            && self.full == other.full
            && self.modifier == other.modifier
            && self.variants == other.variants
            && self.is_important == other.is_important
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
    use crate::{config::Config, selector::parse};

    use std::collections::BTreeSet;

    #[test]
    fn sorting_test() {
        let config = Config::default();

        let mut selectors = BTreeSet::new();
        selectors.extend(parse("lg:bg-red-500", &config).unwrap());
        selectors.extend(parse("bg-red-500", &config).unwrap());

        let mut iter = selectors.iter();
        assert!(
            iter.next().unwrap().full == "bg-red-500"
                && iter.next().unwrap().full == "lg:bg-red-500"
        );
    }
}
