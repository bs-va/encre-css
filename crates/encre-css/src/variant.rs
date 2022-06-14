use crate::config::{Config, DarkModeConfig};

use std::borrow::Cow;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: char = ':';

#[derive(Debug)]
pub enum Variant {
    BeforeClass(Cow<'static, str>),
    AfterClass(&'static str),
    BeforeRule(Cow<'static, str>),
}

pub fn init_variants(config: &Config) -> BTreeMap<Cow<'static, str>, Variant> {
    // NOTE: If there is a variant starting with the same characters than another complete
    // variant, the first found will be the first in alphabetic order
    let mut variants = BTreeMap::new();

    // --- Pseudo element ---

    variants.insert(
        Cow::from("first-letter"),
        Variant::AfterClass("::first-letter"),
    );
    variants.insert(Cow::from("first-line"), Variant::AfterClass("::first-line"));
    variants.insert(
        Cow::from("file"),
        Variant::AfterClass("::file-selector-button"),
    );
    variants.insert(
        Cow::from("placeholder"),
        Variant::AfterClass("::placeholder"),
    );
    variants.insert(Cow::from("backdrop"), Variant::AfterClass("::backdrop"));
    variants.insert(Cow::from("before"), Variant::AfterClass("::before"));
    variants.insert(Cow::from("after"), Variant::AfterClass("::after"));
    // TODO: Support `& *::marker` and `& *::selection`
    variants.insert(Cow::from("marker"), Variant::AfterClass("::marker"));
    variants.insert(Cow::from("selection"), Variant::AfterClass("::selection"));

    // --- Pseudo class ---

    // Interactive
    variants.insert(
        Cow::from("focus-within"),
        Variant::AfterClass(":focus-within"),
    );
    variants.insert(Cow::from("hover"), Variant::AfterClass(":hover"));
    variants.insert(Cow::from("focus"), Variant::AfterClass(":focus"));
    variants.insert(
        Cow::from("focus-visible"),
        Variant::AfterClass(":focus-visible"),
    );
    variants.insert(
        Cow::from("focus-within"),
        Variant::AfterClass(":focus-within"),
    );
    variants.insert(Cow::from("active"), Variant::AfterClass(":active"));
    variants.insert(Cow::from("enabled"), Variant::AfterClass(":enabled"));
    variants.insert(Cow::from("disabled"), Variant::AfterClass(":disabled"));
    variants.insert(
        Cow::from("not-disabled"),
        Variant::AfterClass(":not(:disabled)"),
    );

    // Forms
    variants.insert(Cow::from("default"), Variant::AfterClass(":default"));
    variants.insert(Cow::from("checked"), Variant::AfterClass(":checked"));
    variants.insert(
        Cow::from("not-checked"),
        Variant::AfterClass(":not(:checked)"),
    );
    variants.insert(
        Cow::from("indeterminate"),
        Variant::AfterClass(":indeterminate"),
    );
    variants.insert(
        Cow::from("placeholder-shown"),
        Variant::AfterClass(":placeholder-shown"),
    );
    variants.insert(Cow::from("autofill"), Variant::AfterClass(":autofill"));
    variants.insert(Cow::from("required"), Variant::AfterClass(":required"));
    variants.insert(Cow::from("valid"), Variant::AfterClass(":valid"));
    variants.insert(Cow::from("invalid"), Variant::AfterClass(":invalid"));
    variants.insert(Cow::from("in-range"), Variant::AfterClass(":in-range"));
    variants.insert(
        Cow::from("out-of-range"),
        Variant::AfterClass(":out-of-range"),
    );
    variants.insert(Cow::from("read-only"), Variant::AfterClass(":read-only"));
    variants.insert(Cow::from("read-write"), Variant::AfterClass(":read-write"));

    // Positional
    variants.insert(Cow::from("first"), Variant::AfterClass(":first-child"));
    variants.insert(
        Cow::from("not-first"),
        Variant::AfterClass(":not(:first-child)"),
    );
    variants.insert(Cow::from("last"), Variant::AfterClass(":last-child"));
    variants.insert(
        Cow::from("not-last"),
        Variant::AfterClass(":not(:last-child)"),
    );
    variants.insert(Cow::from("only"), Variant::AfterClass(":only-child"));
    variants.insert(
        Cow::from("not-only"),
        Variant::AfterClass(":not(:only-child)"),
    );
    variants.insert(Cow::from("odd"), Variant::AfterClass(":nth-child(odd)"));
    variants.insert(Cow::from("even"), Variant::AfterClass(":nth-child(even)"));
    variants.insert(
        Cow::from("first-of-type"),
        Variant::AfterClass(":first-of-type"),
    );
    variants.insert(
        Cow::from("not-first-of-type"),
        Variant::AfterClass(":not(:first-of-type)"),
    );
    variants.insert(
        Cow::from("last-of-type"),
        Variant::AfterClass(":last-of-type"),
    );
    variants.insert(
        Cow::from("not-last-of-type"),
        Variant::AfterClass(":not(:last-of-type)"),
    );
    variants.insert(Cow::from("empty"), Variant::AfterClass(":empty"));

    // State
    variants.insert(Cow::from("visited"), Variant::AfterClass(":visited"));
    variants.insert(Cow::from("target"), Variant::AfterClass(":target"));
    variants.insert(Cow::from("open"), Variant::AfterClass("[open]"));

    // --- Direction ---

    variants.insert(
        Cow::from("ltr"),
        Variant::BeforeClass(Cow::from("[dir=\"ltr\"] ")),
    );
    variants.insert(
        Cow::from("rtl"),
        Variant::BeforeClass(Cow::from("[dir=\"rtl\"] ")),
    );

    // --- Reduced motion ---
    variants.insert(
        Cow::from("motion-safe"),
        Variant::BeforeRule(Cow::from("@media (prefers-reduced-motion: no-preference)")),
    );
    variants.insert(
        Cow::from("motion-reduce"),
        Variant::BeforeRule(Cow::from("@media (prefers-reduced-motion: reduce)")),
    );

    // --- Dark ---

    match &config.theme.dark_mode {
        DarkModeConfig::Media => variants.insert(
            Cow::from("dark"),
            Variant::BeforeRule(Cow::from("@media (prefers-color-scheme: dark)")),
        ),
        DarkModeConfig::Class(name) => {
            variants.insert(Cow::from("dark"), Variant::BeforeClass(name.clone() + " "))
        }
    };

    // --- Print ---

    variants.insert(
        Cow::from("print"),
        Variant::BeforeRule(Cow::from("@media print")),
    );

    // --- Screen ---

    config.theme.screens.iter().for_each(|screen| {
        variants.insert(
            screen.0.clone(),
            Variant::BeforeRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    });

    // --- Orientation ---

    variants.insert(
        Cow::from("portrait"),
        Variant::BeforeRule(Cow::from("@media (orientation: portrait)")),
    );
    variants.insert(
        Cow::from("landscape"),
        Variant::BeforeRule(Cow::from("@media (orientation: landscape)")),
    );

    // TODO: Group, peer, parent variants

    variants
}
