//! Define the [`Variant`] enum used to format classes based on variants.
//!
//! Variants are useful to <i>conditionally</i> apply utility classes.
use crate::config::{Config, DarkMode, BUILTIN_SCREENS};

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Structure used to add pseudo-selectors, pseudo-elements, pseudo classes and media queries to
/// CSS rules.
#[derive(Debug)]
pub enum Variant {
    /// Wrap the original class to make another one.
    ///
    /// # Example
    ///
    /// If the variant is `Variant::WrapClass("&::before")` and the original class is `".bg-red-500"`, the class will become `".bg-red-500::before"`).
    WrapClass(Cow<'static, str>),

    /// Add a `@` CSS rule (like `@media`, `@supports`)
    ///
    /// # Example
    ///
    /// If the variant is `Variant::AtRule("@media (orientation: portrait)")` and the original
    /// class is `".bg-red-500"`, the class will become `"@media (orientation: portrait) { .bg-red-500 { ... } }"`).
    AtRule(Cow<'static, str>),
}

pub(crate) fn init_variants(config: &Config) -> BTreeMap<Cow<str>, Variant> {
    // NOTE: If there is a variant starting with the same characters than another complete
    // variant, the first found will be the first in alphabetic order
    let mut variants = BTreeMap::new();

    // --- Screen ---

    for screen in BUILTIN_SCREENS {
        variants.insert(
            Cow::from(screen.0),
            Variant::AtRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    }

    for screen in config.theme.screens.iter() {
        variants.insert(
            screen.0.clone(),
            Variant::AtRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    }

    // --- Dark ---

    match &config.theme.dark_mode {
        DarkMode::Media => {
            variants.insert(
                Cow::from("dark"),
                Variant::AtRule(Cow::from("@media (prefers-color-scheme: dark)")),
            );
        }
        DarkMode::Class(name) => {
            variants.insert(Cow::from("dark"), Variant::WrapClass(name.clone() + " &"));
        }
    }

    variants
}
