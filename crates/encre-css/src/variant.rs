use crate::config::{Config, DarkModeConfig};

use std::collections::BTreeMap;
use std::borrow::Cow;

pub const VARIANT_SEPARATOR: &str = ":";

pub enum Variant {
    PseudoClass(&'static str),
    PseudoElement(&'static str),

    // TODO: Support group variants
    // Parent(&'static str),
    WrapSelector(Cow<'static, str>),
    AtRule(&'static str),
}

pub fn init_variants(config: &Config) -> BTreeMap<&'static str, Variant> {
    // NOTE: If there is a variant starting with the same characters than another complete
    // variant, the first found will be the first in alphabetic order
    let mut variants = BTreeMap::new();

    // --- Pseudo element ---

    variants.insert("first-letter", Variant::PseudoElement("first-letter"));
    variants.insert("first-line", Variant::PseudoElement("first-line"));
    variants.insert("file", Variant::PseudoElement("file-selector-button"));
    variants.insert("placeholder", Variant::PseudoElement("placeholder"));
    variants.insert("backdrop", Variant::PseudoElement("backdrop"));
    variants.insert("before", Variant::PseudoElement("before"));
    variants.insert("after", Variant::PseudoElement("after"));
    variants.insert("marker", Variant::WrapSelector(Cow::from("& *::marker, &::marker")));
    variants.insert("selection", Variant::WrapSelector(Cow::from("& *::selection, &::selection")));

    // --- Pseudo class ---

    // Interactive
    variants.insert("focus-within", Variant::PseudoClass("focus-within"));
    variants.insert("hover", Variant::PseudoClass("hover"));
    variants.insert("focus", Variant::PseudoClass("focus"));
    variants.insert("focus-visible", Variant::PseudoClass("focus-visible"));
    variants.insert("focus-within", Variant::PseudoClass("focus-within"));
    variants.insert("active", Variant::PseudoClass("active"));
    variants.insert("enabled", Variant::PseudoClass("enabled"));
    variants.insert("disabled", Variant::PseudoClass("disabled"));
    variants.insert("not-disabled", Variant::PseudoClass("not(:disabled)"));

    // Forms
    variants.insert("default", Variant::PseudoClass("default"));
    variants.insert("checked", Variant::PseudoClass("checked"));
    variants.insert("not-checked", Variant::PseudoClass("not(:checked)"));
    variants.insert("indeterminate", Variant::PseudoClass("indeterminate"));
    variants.insert("placeholder-shown", Variant::PseudoClass("placeholder-shown"));
    variants.insert("autofill", Variant::PseudoClass("autofill"));
    variants.insert("required", Variant::PseudoClass("required"));
    variants.insert("valid", Variant::PseudoClass("valid"));
    variants.insert("invalid", Variant::PseudoClass("invalid"));
    variants.insert("in-range", Variant::PseudoClass("in-range"));
    variants.insert("out-of-range", Variant::PseudoClass("out-of-range"));
    variants.insert("read-only", Variant::PseudoClass("read-only"));
    variants.insert("read-write", Variant::PseudoClass("read-write"));

    // Positional
    variants.insert("first", Variant::PseudoClass("first-child"));
    variants.insert("not-first", Variant::PseudoClass("not(:first-child)"));
    variants.insert("last", Variant::PseudoClass("last-child"));
    variants.insert("not-last", Variant::PseudoClass("not(:last-child)"));
    variants.insert("only", Variant::PseudoClass("only-child"));
    variants.insert("not-only", Variant::PseudoClass("not(:only-child)"));
    variants.insert("odd", Variant::PseudoClass("nth-child(odd)"));
    variants.insert("even", Variant::PseudoClass("nth-child(even)"));
    variants.insert("first-of-type", Variant::PseudoClass("first-of-type"));
    variants.insert("not-first-of-type", Variant::PseudoClass("not(:first-of-type)"));
    variants.insert("last-of-type", Variant::PseudoClass("last-of-type"));
    variants.insert("not-last-of-type", Variant::PseudoClass("not(:last-of-type)"));
    variants.insert("empty", Variant::PseudoClass("empty"));

    // State
    variants.insert("visited", Variant::PseudoClass("visited"));
    variants.insert("target", Variant::PseudoClass("target"));
    variants.insert("open", Variant::WrapSelector(Cow::from("&[open]")));

    // --- Direction ---

    variants.insert("ltr", Variant::WrapSelector(Cow::from("[dir=\"ltr\"] &")));
    variants.insert("rtl", Variant::WrapSelector(Cow::from("[dir=\"rtl\"] &")));

    // --- Reduced motion ---
    variants.insert("motion-safe", Variant::AtRule("@media (prefers-reduced-motion: no-preference)"));
    variants.insert("motion-reduce", Variant::AtRule("@media (prefers-reduced-motion: reduce)"));

    // --- Dark ---

    match &config.theme.dark_mode {
        DarkModeConfig::Media => variants.insert("dark", Variant::AtRule("@media (prefers-color-scheme: dark)")),
        DarkModeConfig::Class(name) => variants.insert("dark", Variant::WrapSelector(Cow::from(format!("{} &", name)))),
    };

    // --- Print ---

    variants.insert("print", Variant::AtRule("@media print"));

    // --- Screen ---

    // TODO: Config file for screen breakpoints
    variants.insert("sm", Variant::AtRule("@media (min-width: 640px)"));
    variants.insert("md", Variant::AtRule("@media (min-width: 768px)"));
    variants.insert("lg", Variant::AtRule("@media (min-width: 1024px)"));
    variants.insert("xl", Variant::AtRule("@media (min-width: 1280px)"));
    variants.insert("2xl", Variant::AtRule("@media (min-width: 1536px)"));

    // --- Orientation ---

    variants.insert("portrait", Variant::AtRule("@media (orientation: portrait)"));
    variants.insert("landscape", Variant::AtRule("@media (orientation: landscape)"));

    variants
}
