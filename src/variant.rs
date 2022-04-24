use lazy_static::lazy_static;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: &str = ":";

lazy_static! {
    pub static ref VARIANTS: BTreeMap<&'static str, &'static str> = {
        // NOTE: If there is a variant starting with the same characters than another complete
        // variant, the first found will be the first in alphabetic order
        let mut variants = BTreeMap::new();

        // --- Pseudo element ---

        variants.insert("first-letter", ".{class}::first-letter {
  {css}
}");
        variants.insert("first-line", ".{class}::first-line {
  {css}
}");
        variants.insert("marker", ".{class}::marker {
  {css}
}

.{class} *::marker {
  {css}
}");
        variants.insert("selection", ".{class}::selection {
  {css}
}

.{class} *::selection {
  {css}
}");
        variants.insert("file", ".{class}::file-selector-button {
  {css}
}");
        variants.insert("placeholder", ".{class}::placeholder {
  {css}
}");
        variants.insert("backdrop", ".{class}::backdrop {
  {css}
}");
        variants.insert("before", ".{class}::before {
  {css}
}");
        variants.insert("after", ".{class}::after {
  {css}
}");

        // --- Pseudo class ---

        // Positional
        variants.insert("first", ".{class}:first-child {
  {css}
}");
        variants.insert("last", ".{class}:last-child {
  {css}
}");
        variants.insert("only", ".{class}:only-child {
  {css}
}");
        variants.insert("odd", ".{class}:nth-child(odd) {
  {css}
}");
        variants.insert("even", ".{class}:nth-child(even) {
  {css}
}");
        variants.insert("first-of-type", ".{class}:first-of-type {
  {css}
}");
        variants.insert("last-of-type", ".{class}:last-of-type {
  {css}
}");

        // State
        variants.insert("visited", ".{class}:visited {
  {css}
}");
        variants.insert("target", ".{class}:target {
  {css}
}");
        variants.insert("open", ".{class}[open] {
  {css}
}");

        // Forms
        variants.insert("default", ".{class}:default {
  {css}
}");
        variants.insert("checked", ".{class}:checked {
  {css}
}");
        variants.insert("indeterminate", ".{class}:indeterminate {
  {css}
}");
        variants.insert("placeholder-shown", ".{class}:placeholder-shown {
  {css}
}");
        variants.insert("autofill", ".{class}:autofill {
  {css}
}");
        variants.insert("required", ".{class}:required {
  {css}
}");
        variants.insert("valid", ".{class}:valid {
  {css}
}");
        variants.insert("invalid", ".{class}:invalid {
  {css}
}");
        variants.insert("in-range", ".{class}:in-range {
  {css}
}");
        variants.insert("out-of-range", ".{class}:out-of-range {
  {css}
}");
        variants.insert("read-only", ".{class}:read-only {
  {css}
}");

        // Content
        variants.insert("empty", ".{class}:empty {
  {css}
}");

        // Interactive
        variants.insert("focus-within", ".{class}:focus-within {
  {css}
}");
        variants.insert("hover", ".{class}:hover {
  {css}
}");
        variants.insert("focus", ".{class}:focus {
  {css}
}");
        variants.insert("focus-visible", ".{class}:focus-visible {
  {css}
}");
        variants.insert("active", ".{class}:active {
  {css}
}");
        variants.insert("enabled", ".{class}:enabled {
  {css}
}");
        variants.insert("disabled", ".{class}:disabled {
  {css}
}");

        // --- Direction ---

        variants.insert("ltr", "[dir=\"ltr\"] .{class} {
  {css}
}");
        variants.insert("rtl", "[dir=\"rtl\"] .{class} {
  {css}
}");

        // --- Reduced motion ---
        variants.insert("motion-safe", "@media (prefers-reduced-motion: no-preference) {
  .{class} {
    {css}
  }
}");
        variants.insert("motion-reduce", "@media (prefers-reduced-motion: reduce) {
  .{class} {
    {css}
  }
}");

        // --- Dark ---

        // TODO: Config file for choosing mode (`class` or `media`) and className
        /*variants.insert("dark", "body.dark .{class} {
  {css}
}");*/

        // --- Print ---

        variants.insert("print", "@media print {
  .{class} {
    {css}
  }
}");

        // --- Screen ---

        // TODO: Config file for screens
        variants.insert("sm", "@media (min-width: 640px) {
  .{class} {
    {css}
  }
}");
        variants.insert("md", "@media (min-width: 768px) {
  .{class} {
    {css}
  }
}");
        variants.insert("lg", "@media (min-width: 1024px) {
  .{class} {
    {css}
  }
}");
        variants.insert("xl", "@media (min-width: 1280px) {
  .{class} {
    {css}
  }
}");
        variants.insert("2xl", "@media (min-width: 1536px) {
  .{class} {
    {css}
  }
}");

        // --- Orientation ---

        variants.insert("portrait", "@media (orientation: portrait) {
  .{class} {
    {css}
  }
}");
        variants.insert("landscape", "@media (orientation: landscape) {
  .{class} {
    {css}
  }
}");

        variants
    };
}
