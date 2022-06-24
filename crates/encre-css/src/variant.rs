use crate::config::{Config, DarkModeConfig, BUILTIN_SCREENS};

use std::borrow::Cow;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: char = ':';

#[derive(Debug)]
pub enum Variant {
    BeforeClass(Cow<'static, str>),
    AfterClass(&'static str),
    BeforeRule(Cow<'static, str>),
}

pub const BUILTIN_VARIANTS: &[(&str, Variant)] = &[
    // --- Pseudo element ---
    ("first-letter", Variant::AfterClass("::first-letter")),
    ("first-line", Variant::AfterClass("::first-line")),
    ("file", Variant::AfterClass("::file-selector-button")),
    ("placeholder", Variant::AfterClass("::placeholder")),
    ("backdrop", Variant::AfterClass("::backdrop")),
    ("before", Variant::AfterClass("::before")),
    ("after", Variant::AfterClass("::after")),
    // TODO: Support `& *::marker` and `& *::selection`
    ("marker", Variant::AfterClass("::marker")),
    ("selection", Variant::AfterClass("::selection")),
    // --- Pseudo class ---

    // Interactive
    ("focus-within", Variant::AfterClass(":focus-within")),
    ("hover", Variant::AfterClass(":hover")),
    ("focus", Variant::AfterClass(":focus")),
    ("focus-visible", Variant::AfterClass(":focus-visible")),
    ("focus-within", Variant::AfterClass(":focus-within")),
    ("active", Variant::AfterClass(":active")),
    ("enabled", Variant::AfterClass(":enabled")),
    ("disabled", Variant::AfterClass(":disabled")),
    ("not-disabled", Variant::AfterClass(":not(:disabled)")),
    // Forms
    ("default", Variant::AfterClass(":default")),
    ("checked", Variant::AfterClass(":checked")),
    ("not-checked", Variant::AfterClass(":not(:checked)")),
    ("indeterminate", Variant::AfterClass(":indeterminate")),
    (
        "placeholder-shown",
        Variant::AfterClass(":placeholder-shown"),
    ),
    ("autofill", Variant::AfterClass(":autofill")),
    ("required", Variant::AfterClass(":required")),
    ("valid", Variant::AfterClass(":valid")),
    ("invalid", Variant::AfterClass(":invalid")),
    ("in-range", Variant::AfterClass(":in-range")),
    ("out-of-range", Variant::AfterClass(":out-of-range")),
    ("read-only", Variant::AfterClass(":read-only")),
    ("read-write", Variant::AfterClass(":read-write")),
    // Positional
    ("first", Variant::AfterClass(":first-child")),
    ("not-first", Variant::AfterClass(":not(:first-child)")),
    ("last", Variant::AfterClass(":last-child")),
    ("not-last", Variant::AfterClass(":not(:last-child)")),
    ("only", Variant::AfterClass(":only-child")),
    ("not-only", Variant::AfterClass(":not(:only-child)")),
    ("odd", Variant::AfterClass(":nth-child(odd)")),
    ("even", Variant::AfterClass(":nth-child(even)")),
    ("first-of-type", Variant::AfterClass(":first-of-type")),
    (
        "not-first-of-type",
        Variant::AfterClass(":not(:first-of-type)"),
    ),
    ("last-of-type", Variant::AfterClass(":last-of-type")),
    (
        "not-last-of-type",
        Variant::AfterClass(":not(:last-of-type)"),
    ),
    ("empty", Variant::AfterClass(":empty")),
    // State
    ("visited", Variant::AfterClass(":visited")),
    ("target", Variant::AfterClass(":target")),
    ("open", Variant::AfterClass("[open]")),
    // --- Direction ---
    ("ltr", Variant::BeforeClass(Cow::Borrowed("[dir=\"ltr\"] "))),
    ("rtl", Variant::BeforeClass(Cow::Borrowed("[dir=\"rtl\"] "))),
    // --- Reduced motion ---
    (
        "motion-safe",
        Variant::BeforeRule(Cow::Borrowed(
            "@media (prefers-reduced-motion: no-preference)",
        )),
    ),
    (
        "motion-reduce",
        Variant::BeforeRule(Cow::Borrowed("@media (prefers-reduced-motion: reduce)")),
    ),
    // --- Print ---
    ("print", Variant::BeforeRule(Cow::Borrowed("@media print"))),
    // --- Orientation ---
    (
        "portrait",
        Variant::BeforeRule(Cow::Borrowed("@media (orientation: portrait)")),
    ),
    (
        "landscape",
        Variant::BeforeRule(Cow::Borrowed("@media (orientation: landscape)")),
    ),
    // TODO: Group, peer, parent variants
];

pub fn init_variants(config: &Config) -> BTreeMap<Cow<str>, Variant> {
    // NOTE: If there is a variant starting with the same characters than another complete
    // variant, the first found will be the first in alphabetic order
    let mut variants = BTreeMap::new();

    // --- Screen ---

    BUILTIN_SCREENS.iter().for_each(|screen| {
        variants.insert(
            Cow::from(screen.0),
            Variant::BeforeRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    });

    config.theme.screens.iter().for_each(|screen| {
        variants.insert(
            screen.0.clone(),
            Variant::BeforeRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    });

    // --- Dark ---

    match &config.theme.dark_mode {
        DarkModeConfig::Media => {
            variants.insert(
                Cow::from("dark"),
                Variant::BeforeRule(Cow::from("@media (prefers-color-scheme: dark)")),
            );
        }
        DarkModeConfig::Class(name) => {
            variants.insert(Cow::from("dark"), Variant::BeforeClass(name.clone() + " "));
        }
    }

    variants
}
