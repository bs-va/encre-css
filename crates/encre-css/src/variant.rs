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
    /// Wrap the original class to make another one.
    ///
    /// # Example
    ///
    /// If the variant is `VariantType::WrapClass("&::before")` and the original class is `".bg-red-500"`, the class will become `".bg-red-500::before"`).
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
    // NOTE: If there is a variant starting with the same characters than another complete
    // variant, the first found will be the first in alphabetic order
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
