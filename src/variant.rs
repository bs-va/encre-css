use lazy_static::lazy_static;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: &str = ":";

lazy_static! {
    pub static ref VARIANTS: BTreeMap<String, String> = {
        // NOTE: If there is a variant starting with the same characters than another complete
        // variant, the first found will be the first in alphabetic order
        let mut variants = BTreeMap::new();

        // --- Pseudo element ---

        variants.insert("first-letter".to_string(), ".{class}::first-letter {
  {css}
}".to_string());
        variants.insert("first-line".to_string(), ".{class}::first-line {
  {css}
}".to_string());
        variants.insert("marker".to_string(), ".{class}::marker {
  {css}
}

.{class} *::marker {
  {css}
}".to_string());
        variants.insert("selection".to_string(), ".{class}::selection {
  {css}
}

.{class} *::selection {
  {css}
}".to_string());
        variants.insert("file".to_string(), ".{class}::file-selector-button {
  {css}
}".to_string());
        variants.insert("placeholder".to_string(), ".{class}::placeholder {
  {css}
}".to_string());
        variants.insert("backdrop".to_string(), ".{class}::backdrop {
  {css}
}".to_string());
        variants.insert("before".to_string(), ".{class}::before {
  {css}
}".to_string());
        variants.insert("after".to_string(), ".{class}::after {
  {css}
}".to_string());

        // --- Pseudo class ---

        // Positional
        variants.insert("first".to_string(), ".{class}:first-child {
  {css}
}".to_string());
        variants.insert("last".to_string(), ".{class}:last-child {
  {css}
}".to_string());
        variants.insert("only".to_string(), ".{class}:only-child {
  {css}
}".to_string());
        variants.insert("odd".to_string(), ".{class}:nth-child(odd) {
  {css}
}".to_string());
        variants.insert("even".to_string(), ".{class}:nth-child(even) {
  {css}
}".to_string());
        variants.insert("first-of-type".to_string(), ".{class}:first-of-type {
  {css}
}".to_string());
        variants.insert("last-of-type".to_string(), ".{class}:last-of-type {
  {css}
}".to_string());

        // State
        variants.insert("visited".to_string(), ".{class}:visited {
  {css}
}".to_string());
        variants.insert("target".to_string(), ".{class}:target {
  {css}
}".to_string());
        variants.insert("open".to_string(), ".{class}[open] {
  {css}
}".to_string());

        // Forms
        variants.insert("default".to_string(), ".{class}:default {
  {css}
}".to_string());
        variants.insert("checked".to_string(), ".{class}:checked {
  {css}
}".to_string());
        variants.insert("indeterminate".to_string(), ".{class}:indeterminate {
  {css}
}".to_string());
        variants.insert("placeholder-shown".to_string(), ".{class}:placeholder-shown {
  {css}
}".to_string());
        variants.insert("autofill".to_string(), ".{class}:autofill {
  {css}
}".to_string());
        variants.insert("required".to_string(), ".{class}:required {
  {css}
}".to_string());
        variants.insert("valid".to_string(), ".{class}:valid {
  {css}
}".to_string());
        variants.insert("invalid".to_string(), ".{class}:invalid {
  {css}
}".to_string());
        variants.insert("in-range".to_string(), ".{class}:in-range {
  {css}
}".to_string());
        variants.insert("out-of-range".to_string(), ".{class}:out-of-range {
  {css}
}".to_string());
        variants.insert("read-only".to_string(), ".{class}:read-only {
  {css}
}".to_string());

        // Content
        variants.insert("empty".to_string(), ".{class}:empty {
  {css}
}".to_string());

        // Interactive
        variants.insert("focus-within".to_string(), ".{class}:focus-within {
  {css}
}".to_string());
        variants.insert("hover".to_string(), ".{class}:hover {
  {css}
}".to_string());
        variants.insert("focus".to_string(), ".{class}:focus {
  {css}
}".to_string());
        variants.insert("focus-visible".to_string(), ".{class}:focus-visible {
  {css}
}".to_string());
        variants.insert("active".to_string(), ".{class}:active {
  {css}
}".to_string());
        variants.insert("enabled".to_string(), ".{class}:enabled {
  {css}
}".to_string());
        variants.insert("disabled".to_string(), ".{class}:disabled {
  {css}
}".to_string());

        // --- Direction ---

        variants.insert("ltr".to_string(), "[dir=\"ltr\"] .{class} {
  {css}
}".to_string());
        variants.insert("rtl".to_string(), "[dir=\"rtl\"] .{class} {
  {css}
}".to_string());

        // --- Reduced motion ---
        variants.insert("motion-safe".to_string(), "@media (prefers-reduced-motion: no-preference) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("motion-reduce".to_string(), "@media (prefers-reduced-motion: reduce) {
  .{class} {
    {css}
  }
}".to_string());

        // --- Dark ---

        // TODO: Config file for choosing mode (`class` or `media`) and className
        /*variants.insert("dark".to_string(), "body.dark .{class} {
  {css}
}".to_string());*/

        // --- Print ---

        variants.insert("print".to_string(), "@media print {
  .{class} {
    {css}
  }
}".to_string());

        // --- Screen ---

        // TODO: Config file for screens
        variants.insert("sm".to_string(), "@media (min-width: 640px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("md".to_string(), "@media (min-width: 768px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("lg".to_string(), "@media (min-width: 1024px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("xl".to_string(), "@media (min-width: 1280px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("2xl".to_string(), "@media (min-width: 1536px) {
  .{class} {
    {css}
  }
}".to_string());

        // --- Orientation ---

        variants.insert("portrait".to_string(), "@media (orientation: portrait) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("landscape".to_string(), "@media (orientation: landscape) {
  .{class} {
    {css}
  }
}".to_string());

        variants
    };
}
