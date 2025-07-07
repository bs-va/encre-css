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
//! 1. The **[variants](crate::selector::VariantType)** (used to add pseudo-selectors, pseudo-elements, pseudo classes, media queries), in this case
//!    the class will be applied only on a screen larger than 1280px (see [`BUILTIN_SCREENS`]) and
//!    if hovered;
//! 2. The **namespace** (basically the name of the plugin), in this case `bg` for changing the background;
//! 3. The **[modifier](crate::selector::Modifier)** (used to clarify the CSS needed to be generated), in this case the
//!    background color will become `rgb(239 68 68)` (see [`BUILTIN_COLORS`]).
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span style="counter-set: hints 3;">[&>*]</span><span>:</span><span style="counter-set: hints 4;">[@supports_(display:flex)]</span><span>:</span><span style="counter-set: hints 1;">flex</span><span>-</span><span style="counter-set: hints 5;">[2_2_10%]</span></b></p>
//!
//! 4. The **arbitrary variant** (used to modify the class generated), in this case the class
//!    will be `.\[\&\>\*]\:flex-\[2_2_10\%\]>*`.
//!
//! 5. Another **arbitrary variant** (with another syntax used to add [at rules](https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule)),
//!    in this case the rule will become `@supports (display:flex) { <rule content> }` (spaces need to be replaced with underscores in arbitrary
//!    variants).
//!
//! 6. The **[arbitrary value](crate::selector::Modifier::Arbitrary)**
//!    (used to specify a value not included in your design system), in this case the background
//!    color will become `2 2 10%` (spaces need to be replaced with underscores in arbitrary
//!    values).
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>[mask-type:luminance]</span></b></p>
//!
//! 7. The **arbitrary CSS property** (used to use a CSS property not supported by `encre-css`), in
//!    this case the rule content will be `.\[mask-type\:luminance\] { mask-type: luminance; }`.
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>dark:(text-white,bg-gray-500)</span></b></p>
//!
//! 8. The **variant group** (used to group together several classes conditionally enabled by the
//!    same variant), in this case the class will be expanded to `dark:text-white` and
//!    `dark:bg-gray-500`.
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>(hover,focus-visible):bg-blue-400</span></b></p>
//!
//! 9. The **variant group without any namespace or modifier** (used to group together several classes conditionally enabled by the
//!    same variant but sharing the same modifier), in this case the class will be expanded to `hover:bg-blue-400` and
//!    `focus-visible:bg-blue-400`.
//!
//! As you can see, by default variants are separated by `:`, modifiers by `-` (the dash after the
//! first modifier can be omitted, e.g. `m1` instead of `m-1`), arbitrary values/variants are surrounded by `[]` and variant
//! groups are surrounded by `()`.
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

/// Structure used to add pseudo-selectors, pseudo-elements, pseudo classes and media queries to
/// CSS rules.
///
/// Variant are useful to <i>conditionally</i> apply utility classes.
///
/// See [`config::BUILTIN_VARIANTS`] for a list of all default variants.
///
/// See [Tailwind's documentation](https://tailwindcss.com/docs/hover-focus-and-other-states) to learn more about variants.
///
/// [`config::BUILTIN_VARIANTS`]: crate::config::BUILTIN_VARIANTS
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantType {
    /// A CSS [pseudo element](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements).
    ///
    /// # Example
    ///
    /// If the variant is `VariantType::PseudoClass("before")` and the original class is `".bg-red-500"`, the class will become `".bg-red-500::before"`).
    PseudoElement(&'static str),

    /// A CSS [pseudo class](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes).
    ///
    /// # Example
    ///
    /// If the variant is `VariantType::PseudoClass("hover")` and the original class is `".bg-red-500"`, the class will become `".bg-red-500:hover"`).
    PseudoClass(&'static str),

    /// Wrap the original class to make another one.
    ///
    /// # Example
    ///
    /// If the variant is `VariantType::WrapClass("&[open]")` and the original class is `".bg-red-500"`, the class will become `".bg-red-500[open]"`).
    WrapClass(Cow<'static, str>),

    /// Add a `@` CSS rule (like `@media`, `@supports`).
    ///
    /// # Example
    ///
    /// If the variant is `VariantType::AtRule("@media (orientation: portrait)")` and the original
    /// class is `".bg-red-500"`, the class will become `"@media (orientation: portrait) { .bg-red-500 { ... } }"`).
    AtRule(Cow<'static, str>),

    /// A variant applied to a group element like `group-hover`.
    ///
    /// This variant should not be built manually.
    Group {
        /// The name of the group if it's specified.
        ///
        /// It's parsed as the part after the slash, e.g `item` in `group-hover/item:block`
        name: Option<String>,

        /// The pseudo class to apply to the group elements.
        class: &'static str,
    },

    /// A variant applied to a peer element like `peer-focus`.
    ///
    /// This variant should not be built manually.
    Peer {
        /// The name of the peer if it's specified.
        ///
        /// It's parsed as the part after the slash, e.g `item` in `peer-hover/item:block`
        name: Option<String>,

        /// The pseudo class to apply to the peer elements.
        class: &'static str,
    },

    /// A negated variant applied to a peer element like `peer-not-hover`.
    ///
    /// This variant should not be built manually.
    PeerNot {
        /// The name of the peer if it's specified.
        ///
        /// It's parsed as the part after the slash, e.g `item` in `peer-not-hover/item:block`
        name: Option<String>,

        /// The pseudo class to apply to the peer elements.
        class: &'static str
    },
}

/// A selector variant.
#[derive(Debug, Clone, Eq)]
pub enum Variant<'a> {
    /// A known variant with a static variant type.
    Builtin {
        /// The order of the variant among other variants.
        ///
        /// It's used to decide where the generated class having this variant will be placed in
        /// the generated CSS.
        order: usize,

        /// Whether the variant is a prefix followed by an arbitrary value, e.g
        /// `supports-[display:flex]` or a common variant like `hover`.
        prefixed: bool,

        /// The variant type.
        variant: VariantType
    },

    /// A dynamic variant having an arbitrary contents.
    ///
    /// The inner string follows a specific syntax where `&` specifies
    /// a placeholder where the rest of the selector is injected.
    Arbitrary(Cow<'a, str>),
}

impl PartialEq for Variant<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Does not test order because it can change
        match (self, other) {
            (Self::Builtin { variant: v1, .. }, Self::Builtin { variant: v2, .. }) => v1 == v2,
            (Self::Arbitrary(s1), Self::Arbitrary(s2)) => s1 == s2,
            _ => false,
        }
    }
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

impl PartialEq for Selector<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Does not test order because it can change
        self.full == other.full
            && self.modifier == other.modifier
            && self.variants == other.variants
            && self.is_important == other.is_important
    }
}

impl Eq for Selector<'_> {}

impl PartialOrd for Selector<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Selector<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        if self.variants.is_empty() && !other.variants.is_empty() {
            Ordering::Less
        } else if !self.variants.is_empty() && other.variants.is_empty() {
            Ordering::Greater
        } else if !self.variants.is_empty() && !other.variants.is_empty() {
            let mut compared = None;

            // Compare variants in the lexicographic order
            for variant_i in 0..self.variants.len() {
                if variant_i >= other.variants.len() {
                    compared = Some(Ordering::Greater);
                    break;
                }

                let res = match self.variants.get(variant_i).as_ref().unwrap() {
                    Variant::Builtin { order, .. } => order,
                    Variant::Arbitrary(_) => &1_000_000,
                }
                .cmp(&match other.variants.get(variant_i).unwrap() {
                    Variant::Builtin { order, .. } => *order,
                    Variant::Arbitrary(_) => 1_000_001,
                });

                if res != Ordering::Equal {
                    compared = Some(res);
                    break;
                }
            }

            compared.unwrap_or(Ordering::Less).then_with(|| {
                self.order
                    .cmp(&other.order)
                    .then_with(|| self.full.cmp(other.full))
            })
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

        let selectors1 = parse(
            "lg:bg-red-500",
            None,
            None,
            &config,
            &config.get_derived_variants(),
        );
        let selectors2 = parse(
            "bg-red-500",
            None,
            None,
            &config,
            &config.get_derived_variants(),
        );

        let mut selectors = BTreeSet::new();
        selectors.insert(selectors1[0].as_ref().unwrap());
        selectors.insert(selectors2[0].as_ref().unwrap());

        let mut iter = selectors.iter();
        assert!(
            iter.next().unwrap().full == "bg-red-500"
                && iter.next().unwrap().full == "lg:bg-red-500"
        );
    }
}
