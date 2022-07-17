//! Define the [`VariantType`] enum used to format classes based on variants.
//!
//! Variant are useful to <i>conditionally</i> apply utility classes.
//!
//! See [Tailwind's documentation](https://tailwindcss.com/docs/hover-focus-and-other-states) to learn more about variants.
use crate::config::{Config, DarkMode, BUILTIN_SCREENS};

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Structure used to add pseudo-selectors, pseudo-elements, pseudo classes and media queries to
/// CSS rules.
#[derive(Debug, Clone, PartialEq)]
pub enum VariantType {
    /// A CSS [pseudo element](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements)
    ///
    /// # Example
    ///
    /// If the variant is `VariantType::PseudoClass("before")` and the original class is `".bg-red-500"`, the class will become `".bg-red-500::before"`).
    PseudoElement(&'static str),

    /// A CSS [pseudo class](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes)
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

    /// Add a `@` CSS rule (like `@media`, `@supports`)
    ///
    /// # Example
    ///
    /// If the variant is `VariantType::AtRule("@media (orientation: portrait)")` and the original
    /// class is `".bg-red-500"`, the class will become `"@media (orientation: portrait) { .bg-red-500 { ... } }"`).
    AtRule(Cow<'static, str>),
}

pub(crate) fn init_variants(config: &Config) -> BTreeMap<Cow<str>, VariantType> {
    let mut variants = BTreeMap::new();

    // --- Screen ---

    for screen in BUILTIN_SCREENS {
        variants.insert(
            Cow::from(screen.0),
            VariantType::AtRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    }

    for screen in config.theme.screens.iter() {
        variants.insert(
            screen.0.clone(),
            VariantType::AtRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    }

    // --- Dark ---

    match &config.theme.dark_mode {
        DarkMode::Media => {
            variants.insert(
                Cow::from("dark"),
                VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)")),
            );
        }
        DarkMode::Class(name) => {
            variants.insert(
                Cow::from("dark"),
                VariantType::WrapClass(name.clone() + " &"),
            );
        }
    }

    variants
}
