use crate::config::{Config, DarkModeConfig, BUILTIN_SCREENS};

use std::borrow::Cow;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: char = ':';

#[derive(Debug)]
pub enum Variant {
    WrapClass(Cow<'static, str>),
    AtRule(Cow<'static, str>),
}

pub const BUILTIN_VARIANTS: &[(&str, Variant)] = &[
    // --- Pseudo element ---
    (
        "first-letter",
        Variant::WrapClass(Cow::Borrowed("&::first-letter")),
    ),
    ("first-line", Variant::WrapClass(Cow::Borrowed("&::first-line"))),
    (
        "file",
        Variant::WrapClass(Cow::Borrowed("&::file-selector-button")),
    ),
    (
        "placeholder",
        Variant::WrapClass(Cow::Borrowed("&::placeholder")),
    ),
    ("backdrop", Variant::WrapClass(Cow::Borrowed("&::backdrop"))),
    ("before", Variant::WrapClass(Cow::Borrowed("&::before"))),
    ("after", Variant::WrapClass(Cow::Borrowed("&::after"))),
    (
        "marker",
        Variant::WrapClass(Cow::Borrowed("& *::marker, &::marker")),
    ),
    (
        "selection",
        Variant::WrapClass(Cow::Borrowed("& *::selection, &::selection")),
    ),
    // --- Pseudo class ---

    // Interactive
    (
        "focus-within",
        Variant::WrapClass(Cow::Borrowed("&:focus-within")),
    ),
    ("hover", Variant::WrapClass(Cow::Borrowed("&:hover"))),
    ("focus", Variant::WrapClass(Cow::Borrowed("&:focus"))),
    (
        "focus-visible",
        Variant::WrapClass(Cow::Borrowed("&:focus-visible")),
    ),
    (
        "focus-within",
        Variant::WrapClass(Cow::Borrowed("&:focus-within")),
    ),
    ("active", Variant::WrapClass(Cow::Borrowed("&:active"))),
    ("enabled", Variant::WrapClass(Cow::Borrowed("&:enabled"))),
    ("disabled", Variant::WrapClass(Cow::Borrowed("&:disabled"))),
    (
        "not-disabled",
        Variant::WrapClass(Cow::Borrowed("&:not(:disabled)")),
    ),
    // Forms
    ("default", Variant::WrapClass(Cow::Borrowed("&:default"))),
    ("checked", Variant::WrapClass(Cow::Borrowed("&:checked"))),
    (
        "not-checked",
        Variant::WrapClass(Cow::Borrowed("&:not(:checked)")),
    ),
    (
        "indeterminate",
        Variant::WrapClass(Cow::Borrowed("&:indeterminate")),
    ),
    (
        "placeholder-shown",
        Variant::WrapClass(Cow::Borrowed("&:placeholder-shown")),
    ),
    ("autofill", Variant::WrapClass(Cow::Borrowed("&:autofill"))),
    ("required", Variant::WrapClass(Cow::Borrowed("&:required"))),
    ("valid", Variant::WrapClass(Cow::Borrowed("&:valid"))),
    ("invalid", Variant::WrapClass(Cow::Borrowed("&:invalid"))),
    ("in-range", Variant::WrapClass(Cow::Borrowed("&:in-range"))),
    (
        "out-of-range",
        Variant::WrapClass(Cow::Borrowed("&:out-of-range")),
    ),
    ("read-only", Variant::WrapClass(Cow::Borrowed("&:read-only"))),
    ("read-write", Variant::WrapClass(Cow::Borrowed("&:read-write"))),
    // Positional
    ("first", Variant::WrapClass(Cow::Borrowed("&:first-child"))),
    (
        "not-first",
        Variant::WrapClass(Cow::Borrowed("&:not(:first-child)")),
    ),
    ("last", Variant::WrapClass(Cow::Borrowed("&:last-child"))),
    (
        "not-last",
        Variant::WrapClass(Cow::Borrowed("&:not(:last-child)")),
    ),
    ("only", Variant::WrapClass(Cow::Borrowed("&:only-child"))),
    (
        "not-only",
        Variant::WrapClass(Cow::Borrowed("&:not(:only-child)")),
    ),
    ("odd", Variant::WrapClass(Cow::Borrowed("&:nth-child(odd)"))),
    ("even", Variant::WrapClass(Cow::Borrowed("&:nth-child(even)"))),
    (
        "first-of-type",
        Variant::WrapClass(Cow::Borrowed("&:first-of-type")),
    ),
    (
        "not-first-of-type",
        Variant::WrapClass(Cow::Borrowed("&:not(:first-of-type)")),
    ),
    (
        "last-of-type",
        Variant::WrapClass(Cow::Borrowed("&:last-of-type")),
    ),
    (
        "not-last-of-type",
        Variant::WrapClass(Cow::Borrowed("&:not(:last-of-type)")),
    ),
    ("empty", Variant::WrapClass(Cow::Borrowed("&:empty"))),
    // State
    ("visited", Variant::WrapClass(Cow::Borrowed("&:visited"))),
    ("target", Variant::WrapClass(Cow::Borrowed("&:target"))),
    ("open", Variant::WrapClass(Cow::Borrowed("&[open]"))),
    // --- Direction ---
    ("ltr", Variant::WrapClass(Cow::Borrowed("[dir=\"ltr\"] &"))),
    ("rtl", Variant::WrapClass(Cow::Borrowed("[dir=\"rtl\"] &"))),
    // --- Reduced motion ---
    (
        "motion-safe",
        Variant::AtRule(Cow::Borrowed(
            "@media (prefers-reduced-motion: no-preference)",
        )),
    ),
    (
        "motion-reduce",
        Variant::AtRule(Cow::Borrowed("@media (prefers-reduced-motion: reduce)")),
    ),
    // --- Print ---
    ("print", Variant::AtRule(Cow::Borrowed("@media print"))),
    // --- Orientation ---
    (
        "portrait",
        Variant::AtRule(Cow::Borrowed("@media (orientation: portrait)")),
    ),
    (
        "landscape",
        Variant::AtRule(Cow::Borrowed("@media (orientation: landscape)")),
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
            Variant::AtRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    });

    config.theme.screens.iter().for_each(|screen| {
        variants.insert(
            screen.0.clone(),
            Variant::AtRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    });

    // --- Dark ---

    match &config.theme.dark_mode {
        DarkModeConfig::Media => {
            variants.insert(
                Cow::from("dark"),
                Variant::AtRule(Cow::from("@media (prefers-color-scheme: dark)")),
            );
        }
        DarkModeConfig::Class(name) => {
            variants.insert(Cow::from("dark"), Variant::WrapClass(name.clone() + " &"));
        }
    }

    variants
}
