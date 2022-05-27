use crate::config::{Config, DarkModeConfig};

use std::borrow::Cow;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: &str = ":";

#[derive(Debug)]
pub enum Variant {
    PseudoClass(&'static str),
    PseudoElement(&'static str),

    // TODO: Support group variants
    // Parent(&'a str),
    WrapSelector(Cow<'static, str>),
    AtRule(Cow<'static, str>),
}

pub fn init_variants(config: &Config) -> BTreeMap<Cow<'static, str>, Variant> {
    // NOTE: If there is a variant starting with the same characters than another complete
    // variant, the first found will be the first in alphabetic order
    let mut variants = BTreeMap::new();

    // --- Pseudo element ---

    variants.insert(
        Cow::from("first-letter"),
        Variant::PseudoElement("first-letter"),
    );
    variants.insert(
        Cow::from("first-line"),
        Variant::PseudoElement("first-line"),
    );
    variants.insert(
        Cow::from("file"),
        Variant::PseudoElement("file-selector-button"),
    );
    variants.insert(
        Cow::from("placeholder"),
        Variant::PseudoElement("placeholder"),
    );
    variants.insert(Cow::from("backdrop"), Variant::PseudoElement("backdrop"));
    variants.insert(Cow::from("before"), Variant::PseudoElement("before"));
    variants.insert(Cow::from("after"), Variant::PseudoElement("after"));
    variants.insert(
        Cow::from("marker"),
        Variant::WrapSelector(Cow::from("& *::marker, &::marker")),
    );
    variants.insert(
        Cow::from("selection"),
        Variant::WrapSelector(Cow::from("& *::selection, &::selection")),
    );

    // --- Pseudo class ---

    // Interactive
    variants.insert(
        Cow::from("focus-within"),
        Variant::PseudoClass("focus-within"),
    );
    variants.insert(Cow::from("hover"), Variant::PseudoClass("hover"));
    variants.insert(Cow::from("focus"), Variant::PseudoClass("focus"));
    variants.insert(
        Cow::from("focus-visible"),
        Variant::PseudoClass("focus-visible"),
    );
    variants.insert(
        Cow::from("focus-within"),
        Variant::PseudoClass("focus-within"),
    );
    variants.insert(Cow::from("active"), Variant::PseudoClass("active"));
    variants.insert(Cow::from("enabled"), Variant::PseudoClass("enabled"));
    variants.insert(Cow::from("disabled"), Variant::PseudoClass("disabled"));
    variants.insert(
        Cow::from("not-disabled"),
        Variant::PseudoClass("not(:disabled)"),
    );

    // Forms
    variants.insert(Cow::from("default"), Variant::PseudoClass("default"));
    variants.insert(Cow::from("checked"), Variant::PseudoClass("checked"));
    variants.insert(
        Cow::from("not-checked"),
        Variant::PseudoClass("not(:checked)"),
    );
    variants.insert(
        Cow::from("indeterminate"),
        Variant::PseudoClass("indeterminate"),
    );
    variants.insert(
        Cow::from("placeholder-shown"),
        Variant::PseudoClass("placeholder-shown"),
    );
    variants.insert(Cow::from("autofill"), Variant::PseudoClass("autofill"));
    variants.insert(Cow::from("required"), Variant::PseudoClass("required"));
    variants.insert(Cow::from("valid"), Variant::PseudoClass("valid"));
    variants.insert(Cow::from("invalid"), Variant::PseudoClass("invalid"));
    variants.insert(Cow::from("in-range"), Variant::PseudoClass("in-range"));
    variants.insert(
        Cow::from("out-of-range"),
        Variant::PseudoClass("out-of-range"),
    );
    variants.insert(Cow::from("read-only"), Variant::PseudoClass("read-only"));
    variants.insert(Cow::from("read-write"), Variant::PseudoClass("read-write"));

    // Positional
    variants.insert(Cow::from("first"), Variant::PseudoClass("first-child"));
    variants.insert(
        Cow::from("not-first"),
        Variant::PseudoClass("not(:first-child)"),
    );
    variants.insert(Cow::from("last"), Variant::PseudoClass("last-child"));
    variants.insert(
        Cow::from("not-last"),
        Variant::PseudoClass("not(:last-child)"),
    );
    variants.insert(Cow::from("only"), Variant::PseudoClass("only-child"));
    variants.insert(
        Cow::from("not-only"),
        Variant::PseudoClass("not(:only-child)"),
    );
    variants.insert(Cow::from("odd"), Variant::PseudoClass("nth-child(odd)"));
    variants.insert(Cow::from("even"), Variant::PseudoClass("nth-child(even)"));
    variants.insert(
        Cow::from("first-of-type"),
        Variant::PseudoClass("first-of-type"),
    );
    variants.insert(
        Cow::from("not-first-of-type"),
        Variant::PseudoClass("not(:first-of-type)"),
    );
    variants.insert(
        Cow::from("last-of-type"),
        Variant::PseudoClass("last-of-type"),
    );
    variants.insert(
        Cow::from("not-last-of-type"),
        Variant::PseudoClass("not(:last-of-type)"),
    );
    variants.insert(Cow::from("empty"), Variant::PseudoClass("empty"));

    // State
    variants.insert(Cow::from("visited"), Variant::PseudoClass("visited"));
    variants.insert(Cow::from("target"), Variant::PseudoClass("target"));
    variants.insert(
        Cow::from("open"),
        Variant::WrapSelector(Cow::from("&[open]")),
    );

    // --- Direction ---

    variants.insert(
        Cow::from("ltr"),
        Variant::WrapSelector(Cow::from("[dir=\"ltr\"] &")),
    );
    variants.insert(
        Cow::from("rtl"),
        Variant::WrapSelector(Cow::from("[dir=\"rtl\"] &")),
    );

    // --- Reduced motion ---
    variants.insert(
        Cow::from("motion-safe"),
        Variant::AtRule(Cow::from("@media (prefers-reduced-motion: no-preference)")),
    );
    variants.insert(
        Cow::from("motion-reduce"),
        Variant::AtRule(Cow::from("@media (prefers-reduced-motion: reduce)")),
    );

    // --- Dark ---

    match &config.theme.dark_mode {
        DarkModeConfig::Media => variants.insert(
            Cow::from("dark"),
            Variant::AtRule(Cow::from("@media (prefers-color-scheme: dark)")),
        ),
        DarkModeConfig::Class(name) => variants.insert(
            Cow::from("dark"),
            Variant::WrapSelector(Cow::from(format!("{} &", name))),
        ),
    };

    // --- Print ---

    variants.insert(
        Cow::from("print"),
        Variant::AtRule(Cow::from("@media print")),
    );

    // --- Screen ---

    for screen in &*config.theme.screens {
        variants.insert(
            screen.0.clone(),
            Variant::AtRule(Cow::Owned(format!("@media (min-width: {})", screen.1))),
        );
    }

    // --- Orientation ---

    variants.insert(
        Cow::from("portrait"),
        Variant::AtRule(Cow::from("@media (orientation: portrait)")),
    );
    variants.insert(
        Cow::from("landscape"),
        Variant::AtRule(Cow::from("@media (orientation: landscape)")),
    );

    variants
}
