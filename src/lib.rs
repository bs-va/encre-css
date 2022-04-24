//! A TailwindCSS compatible CSS generation library written in Rust
//!
//! # Example
//!
//! ```rust
//! use tailwind_rs::TailwindGenerator;
//!
//! let mut generator = TailwindGenerator::new();
//! generator.scan_content(r#"class="bg-red-500""#);
//!
//! assert!(generator.generate().contains(r#".bg-red-500 {
//!   --tw-bg-opacity: 1;
//!   background-color: rgb(239 68 68 / var(--tw-bg-opacity));
//! }"#));
//! ```
pub mod plugins;
pub mod preflight;
pub mod selector;
pub mod utils;
pub mod variant;

use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, io::Read, path::PathBuf};
use rayon::prelude::*;

use plugins::PLUGINS;
use preflight::TAILWIND_PREFLIGHT_CSS;
use selector::Selector;
use variant::VARIANTS;

// TODO features:
// - Cache (dedup directly by scanning in all files at once (+ use rayon later))
// - Variant stacking (instead let variant = split.next()..., reverse the iterator and collect all
// variants)
// - Find changed files (using timestamp of generated files and timestamp of source files)
// - Real prefix (like tw-)???
// - CSS variant for dark: configurable
// - Configurable preflight
// - Support a safelist in the configuration file
//
// Performances:
// - Use the std::write macro and pass a Formatter (instead of returning a String) in
// css_template_value (for preventing .to_string()ing)

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r#"(?-u)[\s'"`;>=]+"#).unwrap();
    static ref FILTER_REGEX: fancy_regex::Regex =
        fancy_regex::Regex::new(r"(?!\d|-{2}|-\d)[a-zA-Z0-9\u00A0-\uFFFF-_:%-?']").unwrap();
    static ref URL_REGEX: Regex = Regex::new("^url\\(.*\\)$").unwrap();
}

const WILL_BE_REPLACED_BY_UNDERSCORE: &str = "WILL-BE-REPLACED-BY-UNDERSCORE";

/// Convert an arbitrary value into a CSS value
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s)
pub fn to_css_value(val: &str) -> String {
    // Don't replace `_` if it is a URL
    //
    // TODO: Do the same for values **containing** an url (e.g. 10px_5px_1px_2px_url('/hello/world.png'))
    if !URL_REGEX.is_match(val) {
        // Don't replace `_` if prefixed by a `\`
        val.replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE)
            .replace('_', " ")
            .replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_")
    } else {
        val.to_string() // TODO: Prevent allocating
    }

    // TODO: Add spaces around operators inside calc() that do not follow an operator
    /* or '('.
    return value.replace(
        /(-?\d*\.?\d(?!\b-.+[,)](?![^+\-/*])\D)(?:%|[a-z]+)?|\))([+\-/*])/g,
        '$1 $2 '
    )
    }*/*/*/
}

/// <https://v2.tailwindcss.com/docs/just-in-time-mode#arbitrary-value-support>
pub const VALID_PLUGIN_HINT: [&str; 4] = ["color", "length", "angle", "list"];

/// Generate a complete CSS rule (with a class selector, a rule content and, if requested, some
/// pseudo-elements or `@media` queries)
pub fn gen_css_rule(selector: &Selector, css_content: &str) -> String {
    let css_selector = selector
        .to_string()
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('/', "\\/")
        .replace('\"', "\\\"")
        .replace('\'', "\\'")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('#', "\\#")
        .replace(':', "\\:")
        .replace(',', r"\2c ")
        .replace('.', "\\.")
        .replace('!', "\\!");

    let css_content = if selector.is_important() {
        css_content.replace(';', " !important;")
    } else {
        css_content.to_string() // TODO: Prevent?
    };

    if let Some(ref variant) = selector.get_variant() {
        let with_variant = if let Some(result) = VARIANTS.get(variant.as_str()) {
            result
        } else {
            panic!("Unknown variant: {}", variant);
        };

        with_variant
            .replace("{class}", &css_selector)
            .replace("{css}", &css_content)
    } else {
        format!(".{} {{\n  {}\n}}", css_selector, &css_content)
    }
}

/// Main structure used to generate CSS from selectors
#[derive(Default)]
pub struct TailwindGenerator {
    scanned_selectors_without_variant: Vec<Selector>,
    scanned_selectors_with_variant: Vec<Selector>,
}

impl TailwindGenerator {
    pub fn new() -> Self {
        Self {
            scanned_selectors_without_variant: vec![],
            scanned_selectors_with_variant: vec![],
        }
    }

    /// Add a new selector which will have its CSS generated
    ///
    /// This function automatically handles duplicated selectors
    pub fn add_selector(&mut self, val: &str) {
        let selector = Selector::new(val);

        if selector.get_variant().is_some() {
            if !self.scanned_selectors_with_variant.contains(&selector) {
                self.scanned_selectors_with_variant.push(selector);
            }
        } else if !self.scanned_selectors_without_variant.contains(&selector) {
            self.scanned_selectors_without_variant.push(selector);
        }
    }

    /// Scan the content of a file and store all the selectors found
    pub fn scan_content(&mut self, content: &str) {
        for val in SPLIT_REGEX
            .split(content)
            .filter(|m| FILTER_REGEX.is_match(m).unwrap())
        {
            self.add_selector(val);
        }
    }

    /// Scan all files given and store all the selectors found
    pub fn scan_files(&mut self, files: impl Iterator<Item = PathBuf>) {
        let mut file_content: String = String::new();

        for file in files {
            let mut file = fs::File::open(file).unwrap();
            file_content.clear();
            file.read_to_string(&mut file_content).unwrap();
            self.scan_content(&file_content);
        }
    }

    /// Generate the CSS styles needed based on the scanned selectors
    ///
    /// NOTE: Don't forget to scan selectors using either [scan_files] or [scan_content] or by
    /// adding individual selectors using [add_selector]
    ///
    /// [scan_files]: TailwindGenerator::scan_files
    /// [scan_content]: TailwindGenerator::scan_content
    /// [add_selector]: TailwindGenerator::add_selector
    pub fn generate(&self) -> String {
        let result = [
            &self.scanned_selectors_without_variant,
            &self.scanned_selectors_with_variant,
        ]
        .par_iter()
        .map(|v| *v)
        .flatten()
        .filter_map(|selector| {
            let mut css_content = String::new();

            // Find the right plugin to handle this selector (if the resulting CSS is valid,
            // the plugin is good)
            for plugin in PLUGINS.iter() {
                if selector.check_namespace(plugin.namespace()) {
                    let maybe_arbitrary_value = selector.get_arbitrary_value();
                    let arbitrary_value =
                        if let Some(ref arbitrary_value) = maybe_arbitrary_value {
                            let mut split = arbitrary_value.split(':');
                            let maybe_hint = split.next().unwrap();

                            if maybe_hint == arbitrary_value {
                                // No plugin hint
                                Some(("", maybe_hint))
                            } else {
                                let val = split.next();

                                if let Some(val) = val {
                                    if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                                        // Valid! Return (hint, stripped arbitrary value)
                                        Some((maybe_hint, val))
                                    } else {
                                        // Unknown plugin hint (like `bg-[sth:#333]`)
                                        // TODO: Display a warning
                                        Some(("", val))
                                    }
                                } else {
                                    // Malformed arbitrary value (like just `bg-[color:]`)
                                    // TODO: Display a warning
                                    Some(("", maybe_hint))
                                }
                            }
                        } else {
                            None
                        };

                    if let Some(arbitrary_value) = arbitrary_value {
                        if plugin.is_matching_value(arbitrary_value.0, arbitrary_value.1) {
                            plugin.css_template_value(&to_css_value(arbitrary_value.1), &mut css_content).expect("failed to get the CSS from the modifier");

                            if !css_content.is_empty() {
                                return Some(gen_css_rule(selector, &css_content));
                            }
                        }
                    } else {
                        plugin.get_css_for_modifier(&selector.get_modifier(plugin.namespace()), &mut css_content).expect("failed to get the CSS from the modifier");

                        if !css_content.is_empty() {
                            return Some(gen_css_rule(selector, &css_content));
                        }
                    }
                }
            }

            None
        }).collect::<Vec<String>>();

        format!("{}{}", TAILWIND_PREFLIGHT_CSS, result.join("\n\n"))
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn scan_selectors_test() {
        let mut result = vec![];
        scan_selectors(
            &mut result,
            r#"<div class="w-full h-full absolute bg-blue-500 foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"></div>"#
                .to_string()
        );

        assert_eq!(
            result,
            vec![
                Selector::new("w-full"),
                Selector::new("h-full"),
                Selector::new("absolute"),
                Selector::new("bg-blue-500"),
                Selector::new("hover:bg-black"),
                Selector::new("border-[#333]"),
                Selector::new("text-[color:var(--hello)]"),
            ]
        );
    }

    #[test]
    fn gen_selector_css_test() {
        assert_eq!(
            gen_css_from_content("w-full"),
            format!(
                r#"{}.w-full {{
  width: 100%;
}}"#,
                preflight::TAILWIND_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_with_variant_selector_css_test() {
        assert_eq!(
            gen_css_from_content("focus:w-full"),
            format!(
                r#"{}.focus\:w-full:focus {{
  width: 100%;
}}"#,
                preflight::TAILWIND_PREFLIGHT_CSS,
            )
        );
    }

    #[test]
    fn gen_css_from_files_test() {
        assert_eq!(
            gen_css_from_files(&["tests/fixtures/index.html".into()]),
            r#"/*
! tailwindcss v3.0.23 | MIT License | https://tailwindcss.com
*/

/*
1. Prevent padding and border from affecting element width. (https://github.com/mozdevs/cssremedy/issues/4)
2. Allow adding a border to an element by just adding a border-width. (https://github.com/tailwindcss/tailwindcss/pull/116)
*/

*,
::before,
::after {
  box-sizing: border-box;
/* 1 */
  border-width: 0;
/* 2 */
  border-style: solid;
/* 2 */
  border-color: #e5e7eb;
/* 2 */
}

::before,
::after {
  --tw-content: '';
}

/*
1. Use a consistent sensible line-height in all browsers.
2. Prevent adjustments of font size after orientation changes in iOS.
3. Use a more readable tab size.
4. Use the user's configured `sans` font-family by default.
*/

html {
  line-height: 1.5;
/* 1 */
  -webkit-text-size-adjust: 100%;
/* 2 */
  -moz-tab-size: 4;
/* 3 */
  -o-tab-size: 4;
     tab-size: 4;
/* 3 */
  font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
/* 4 */
}

/*
1. Remove the margin in all browsers.
2. Inherit line-height from `html` so users can set them as a class directly on the `html` element.
*/

body {
  margin: 0;
/* 1 */
  line-height: inherit;
/* 2 */
}

/*
1. Add the correct height in Firefox.
2. Correct the inheritance of border color in Firefox. (https://bugzilla.mozilla.org/show_bug.cgi?id=190655)
3. Ensure horizontal rules are visible by default.
*/

hr {
  height: 0;
/* 1 */
  color: inherit;
/* 2 */
  border-top-width: 1px;
/* 3 */
}

/*
Add the correct text decoration in Chrome, Edge, and Safari.
*/

abbr:where([title]) {
  -webkit-text-decoration: underline dotted;
          text-decoration: underline dotted;
}

/*
Remove the default font size and weight for headings.
*/

h1,
h2,
h3,
h4,
h5,
h6 {
  font-size: inherit;
  font-weight: inherit;
}

/*
Reset links to optimize for opt-in styling instead of opt-out.
*/

a {
  color: inherit;
  text-decoration: inherit;
}

/*
Add the correct font weight in Edge and Safari.
*/

b,
strong {
  font-weight: bolder;
}

/*
1. Use the user's configured `mono` font family by default.
2. Correct the odd `em` font sizing in all browsers.
*/

code,
kbd,
samp,
pre {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
/* 1 */
  font-size: 1em;
/* 2 */
}

/*
Add the correct font size in all browsers.
*/

small {
  font-size: 80%;
}

/*
Prevent `sub` and `sup` elements from affecting the line height in all browsers.
*/

sub,
sup {
  font-size: 75%;
  line-height: 0;
  position: relative;
  vertical-align: baseline;
}

sub {
  bottom: -0.25em;
}

sup {
  top: -0.5em;
}

/*
1. Remove text indentation from table contents in Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=999088, https://bugs.webkit.org/show_bug.cgi?id=201297)
2. Correct table border color inheritance in all Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=935729, https://bugs.webkit.org/show_bug.cgi?id=195016)
3. Remove gaps between table borders by default.
*/

table {
  text-indent: 0;
/* 1 */
  border-color: inherit;
/* 2 */
  border-collapse: collapse;
/* 3 */
}

/*
1. Change the font styles in all browsers.
2. Remove the margin in Firefox and Safari.
3. Remove default padding in all browsers.
*/

button,
input,
optgroup,
select,
textarea {
  font-family: inherit;
/* 1 */
  font-size: 100%;
/* 1 */
  line-height: inherit;
/* 1 */
  color: inherit;
/* 1 */
  margin: 0;
/* 2 */
  padding: 0;
/* 3 */
}

/*
Remove the inheritance of text transform in Edge and Firefox.
*/

button,
select {
  text-transform: none;
}

/*
1. Correct the inability to style clickable types in iOS and Safari.
2. Remove default button styles.
*/

button,
[type='button'],
[type='reset'],
[type='submit'] {
  -webkit-appearance: button;
/* 1 */
  background-color: transparent;
/* 2 */
  background-image: none;
/* 2 */
}

/*
Use the modern Firefox focus style for all focusable elements.
*/

:-moz-focusring {
  outline: auto;
}

/*
Remove the additional `:invalid` styles in Firefox. (https://github.com/mozilla/gecko-dev/blob/2f9eacd9d3d995c937b4251a5557d95d494c9be1/layout/style/res/forms.css#L728-L737)
*/

:-moz-ui-invalid {
  box-shadow: none;
}

/*
Add the correct vertical alignment in Chrome and Firefox.
*/

progress {
  vertical-align: baseline;
}

/*
Correct the cursor style of increment and decrement buttons in Safari.
*/

::-webkit-inner-spin-button,
::-webkit-outer-spin-button {
  height: auto;
}

/*
1. Correct the odd appearance in Chrome and Safari.
2. Correct the outline style in Safari.
*/

[type='search'] {
  -webkit-appearance: textfield;
/* 1 */
  outline-offset: -2px;
/* 2 */
}

/*
Remove the inner padding in Chrome and Safari on macOS.
*/

::-webkit-search-decoration {
  -webkit-appearance: none;
}

/*
1. Correct the inability to style clickable types in iOS and Safari.
2. Change font properties to `inherit` in Safari.
*/

::-webkit-file-upload-button {
  -webkit-appearance: button;
/* 1 */
  font: inherit;
/* 2 */
}

/*
Add the correct display in Chrome and Safari.
*/

summary {
  display: list-item;
}

/*
Removes the default spacing and border for appropriate elements.
*/

blockquote,
dl,
dd,
h1,
h2,
h3,
h4,
h5,
h6,
hr,
figure,
p,
pre {
  margin: 0;
}

fieldset {
  margin: 0;
  padding: 0;
}

legend {
  padding: 0;
}

ol,
ul,
menu {
  list-style: none;
  margin: 0;
  padding: 0;
}

/*
Prevent resizing textareas horizontally by default.
*/

textarea {
  resize: vertical;
}

/*
1. Reset the default placeholder opacity in Firefox. (https://github.com/tailwindlabs/tailwindcss/issues/3300)
2. Set the default placeholder color to the user's configured gray 400 color.
*/

input::-moz-placeholder, textarea::-moz-placeholder {
  opacity: 1;
/* 1 */
  color: #9ca3af;
/* 2 */
}

input:-ms-input-placeholder, textarea:-ms-input-placeholder {
  opacity: 1;
/* 1 */
  color: #9ca3af;
/* 2 */
}

input::placeholder,
textarea::placeholder {
  opacity: 1;
/* 1 */
  color: #9ca3af;
/* 2 */
}

/*
Set the default cursor for buttons.
*/

button,
[role="button"] {
  cursor: pointer;
}

/*
Make sure disabled buttons don't get the pointer cursor.
*/

:disabled {
  cursor: default;
}

/*
1. Make replaced elements `display: block` by default. (https://github.com/mozdevs/cssremedy/issues/14)
2. Add `vertical-align: middle` to align replaced elements more sensibly by default. (https://github.com/jensimmons/cssremedy/issues/14#issuecomment-634934210)
   This can trigger a poorly considered lint error in some tools but is included by design.
*/

img,
svg,
video,
canvas,
audio,
iframe,
embed,
object {
  display: block;
/* 1 */
  vertical-align: middle;
/* 2 */
}

/*
Constrain images and videos to the parent width and preserve their intrinsic aspect ratio. (https://github.com/mozdevs/cssremedy/issues/14)
*/

img,
video {
  max-width: 100%;
  height: auto;
}

/*
Ensure the default browser behavior of the `hidden` attribute.
*/

[hidden] {
  display: none;
}

*, ::before, ::after {
  --tw-translate-x: 0;
  --tw-translate-y: 0;
  --tw-rotate: 0;
  --tw-skew-x: 0;
  --tw-skew-y: 0;
  --tw-scale-x: 1;
  --tw-scale-y: 1;
  --tw-pan-x:  ;
  --tw-pan-y:  ;
  --tw-pinch-zoom:  ;
  --tw-scroll-snap-strictness: proximity;
  --tw-ordinal:  ;
  --tw-slashed-zero:  ;
  --tw-numeric-figure:  ;
  --tw-numeric-spacing:  ;
  --tw-numeric-fraction:  ;
  --tw-ring-inset:  ;
  --tw-ring-offset-width: 0px;
  --tw-ring-offset-color: #fff;
  --tw-ring-color: rgb(59 130 246 / 0.5);
  --tw-ring-offset-shadow: 0 0 #0000;
  --tw-ring-shadow: 0 0 #0000;
  --tw-shadow: 0 0 #0000;
  --tw-shadow-colored: 0 0 #0000;
  --tw-blur:  ;
  --tw-brightness:  ;
  --tw-contrast:  ;
  --tw-grayscale:  ;
  --tw-hue-rotate:  ;
  --tw-invert:  ;
  --tw-saturate:  ;
  --tw-sepia:  ;
  --tw-drop-shadow:  ;
  --tw-backdrop-blur:  ;
  --tw-backdrop-brightness:  ;
  --tw-backdrop-contrast:  ;
  --tw-backdrop-grayscale:  ;
  --tw-backdrop-hue-rotate:  ;
  --tw-backdrop-invert:  ;
  --tw-backdrop-opacity:  ;
  --tw-backdrop-saturate:  ;
  --tw-backdrop-sepia:  ;
}

.absolute {
  position: absolute;
}

.relative {
  position: relative;
}

.inset-0 {
  top: 0px;
  right: 0px;
  bottom: 0px;
  left: 0px;
}

.mx-auto {
  margin-left: auto;
  margin-right: auto;
}

.my-4 {
  margin-top: 1rem;
  margin-bottom: 1rem;
}

.mt-6 {
  margin-top: 1.5rem;
}

.mt-10 {
  margin-top: 2.5rem;
}

.mt-16 {
  margin-top: 4rem;
}

.mt-1 {
  margin-top: 0.25rem;
}

.ml-2 {
  margin-left: 0.5rem;
}

.mb-5 {
  margin-bottom: 1.25rem;
}

.block {
  display: block;
}

.flex {
  display: flex;
}

.grid {
  display: grid;
}

.hidden {
  display: none;
}

.h-full {
  height: 100%;
}

.h-16 {
  height: 4rem;
}

.h-4 {
  height: 1rem;
}

.min-h-screen {
  min-height: 100vh;
}

.min-h-full {
  min-height: 100%;
}

.w-full {
  width: 100%;
}

.w-32 {
  width: 8rem;
}

.w-4 {
  width: 1rem;
}

.w-80 {
  width: 20rem;
}

.max-w-7xl {
  max-width: 80rem;
}

.max-w-lg {
  max-width: 32rem;
}

.max-w-sm {
  max-width: 24rem;
}

.max-w-\[12rem\] {
  max-width: 12rem;
}

.max-w-md {
  max-width: 28rem;
}

.flex-1 {
  flex: 1 1 0%;
}

.cursor-pointer {
  cursor: pointer;
}

.flex-col {
  flex-direction: column;
}

.flex-wrap {
  flex-wrap: wrap;
}

.items-center {
  align-items: center;
}

.justify-center {
  justify-content: center;
}

.justify-between {
  justify-content: space-between;
}

.gap-8 {
  gap: 2rem;
}

.gap-1 {
  gap: 0.25rem;
}

.gap-4 {
  gap: 1rem;
}

.space-y-4 > :not([hidden]) ~ :not([hidden]) {
  --tw-space-y-reverse: 0;
  margin-top: calc(1rem * calc(1 - var(--tw-space-y-reverse)));
  margin-bottom: calc(1rem * var(--tw-space-y-reverse));
}

.space-y-6 > :not([hidden]) ~ :not([hidden]) {
  --tw-space-y-reverse: 0;
  margin-top: calc(1.5rem * calc(1 - var(--tw-space-y-reverse)));
  margin-bottom: calc(1.5rem * var(--tw-space-y-reverse));
}

.rounded-md {
  border-radius: 0.375rem;
}

.rounded {
  border-radius: 0.25rem;
}

.rounded-lg {
  border-radius: 0.5rem;
}

.border {
  border-width: 1px;
}

.border-2 {
  border-width: 2px;
}

.border-r {
  border-right-width: 1px;
}

.border-b {
  border-bottom-width: 1px;
}

.border-transparent {
  border-color: transparent;
}

.border-gray-500 {
  --tw-border-opacity: 1;
  border-color: rgb(107 114 128 / var(--tw-border-opacity));
}

.border-gray-300 {
  --tw-border-opacity: 1;
  border-color: rgb(209 213 219 / var(--tw-border-opacity));
}

.border-blue-500 {
  --tw-border-opacity: 1;
  border-color: rgb(59 130 246 / var(--tw-border-opacity));
}

.bg-white {
  --tw-bg-opacity: 1;
  background-color: rgb(255 255 255 / var(--tw-bg-opacity));
}

.bg-\[color\:rgba\(254\2c 204\2c 27\2c 0\.5\)\] {
  background-color: rgba(254,204,27,0.5);
}

.bg-yellow-500 {
  --tw-bg-opacity: 1;
  background-color: rgb(234 179 8 / var(--tw-bg-opacity));
}

.bg-blue-500 {
  --tw-bg-opacity: 1;
  background-color: rgb(59 130 246 / var(--tw-bg-opacity));
}

.bg-slate-800 {
  --tw-bg-opacity: 1;
  background-color: rgb(30 41 59 / var(--tw-bg-opacity));
}

.bg-slate-600 {
  --tw-bg-opacity: 1;
  background-color: rgb(71 85 105 / var(--tw-bg-opacity));
}

.bg-gray-50 {
  --tw-bg-opacity: 1;
  background-color: rgb(249 250 251 / var(--tw-bg-opacity));
}

.object-cover {
  -o-object-fit: cover;
     object-fit: cover;
}

.p-1 {
  padding: 0.25rem;
}

.p-4 {
  padding: 1rem;
}

.p-6 {
  padding: 1.5rem;
}

.p-5 {
  padding: 1.25rem;
}

.px-4 {
  padding-left: 1rem;
  padding-right: 1rem;
}

.py-3 {
  padding-top: 0.75rem;
  padding-bottom: 0.75rem;
}

.py-2 {
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}

.px-8 {
  padding-left: 2rem;
  padding-right: 2rem;
}

.px-2 {
  padding-left: 0.5rem;
  padding-right: 0.5rem;
}

.py-1 {
  padding-top: 0.25rem;
  padding-bottom: 0.25rem;
}

.py-6 {
  padding-top: 1.5rem;
  padding-bottom: 1.5rem;
}

.px-3 {
  padding-left: 0.75rem;
  padding-right: 0.75rem;
}

.pt-16 {
  padding-top: 4rem;
}

.pb-8 {
  padding-bottom: 2rem;
}

.pt-1 {
  padding-top: 0.25rem;
}

.text-center {
  text-align: center;
}

.text-right {
  text-align: right;
}

.text-6xl {
  font-size: 3.75rem;
  line-height: 1;
}

.text-xl {
  font-size: 1.25rem;
  line-height: 1.75rem;
}

.text-base {
  font-size: 1rem;
  line-height: 1.5rem;
}

.text-sm {
  font-size: 0.875rem;
  line-height: 1.25rem;
}

.text-lg {
  font-size: 1.125rem;
  line-height: 1.75rem;
}

.text-3xl {
  font-size: 1.875rem;
  line-height: 2.25rem;
}

.text-2xl {
  font-size: 1.5rem;
  line-height: 2rem;
}

.text-5xl {
  font-size: 3rem;
  line-height: 1;
}

.font-extrabold {
  font-weight: 800;
}

.font-medium {
  font-weight: 500;
}

.font-bold {
  font-weight: 700;
}

.font-black {
  font-weight: 900;
}

.uppercase {
  text-transform: uppercase;
}

.leading-loose {
  line-height: 2;
}

.leading-6 {
  line-height: 1.5rem;
}

.tracking-tight {
  letter-spacing: -0.025em;
}

.text-yellow-500 {
  --tw-text-opacity: 1;
  color: rgb(234 179 8 / var(--tw-text-opacity));
}

.text-white {
  --tw-text-opacity: 1;
  color: rgb(255 255 255 / var(--tw-text-opacity));
}

.text-yellow-700 {
  --tw-text-opacity: 1;
  color: rgb(161 98 7 / var(--tw-text-opacity));
}

.text-gray-700 {
  --tw-text-opacity: 1;
  color: rgb(55 65 81 / var(--tw-text-opacity));
}

.text-red-700 {
  --tw-text-opacity: 1;
  color: rgb(185 28 28 / var(--tw-text-opacity));
}

.text-gray-500 {
  --tw-text-opacity: 1;
  color: rgb(107 114 128 / var(--tw-text-opacity));
}

.text-blue-500 {
  --tw-text-opacity: 1;
  color: rgb(59 130 246 / var(--tw-text-opacity));
}

.text-blue-600 {
  --tw-text-opacity: 1;
  color: rgb(37 99 235 / var(--tw-text-opacity));
}

.text-gray-900 {
  --tw-text-opacity: 1;
  color: rgb(17 24 39 / var(--tw-text-opacity));
}

.text-blue-100 {
  --tw-text-opacity: 1;
  color: rgb(219 234 254 / var(--tw-text-opacity));
}

.underline {
  -webkit-text-decoration-line: underline;
          text-decoration-line: underline;
}

.decoration-blue-500 {
  -webkit-text-decoration-color: #3b82f6;
          text-decoration-color: #3b82f6;
}

.decoration-2 {
  text-decoration-thickness: 2px;
}

.underline-offset-4 {
  text-underline-offset: 4px;
}

.mix-blend-multiply {
  mix-blend-mode: multiply;
}

.shadow-xl {
  --tw-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
  --tw-shadow-colored: 0 20px 25px -5px var(--tw-shadow-color), 0 8px 10px -6px var(--tw-shadow-color);
  box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow);
}

.shadow-sm {
  --tw-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --tw-shadow-colored: 0 1px 2px 0 var(--tw-shadow-color);
  box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow);
}

.drop-shadow-md {
  --tw-drop-shadow: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));
  filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);
}

.grayscale {
  --tw-grayscale: grayscale(100%);
  filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);
}

.transition {
  transition-property: color, background-color, border-color, fill, stroke, opacity, box-shadow, transform, filter, -webkit-text-decoration-color, -webkit-backdrop-filter;
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter, -webkit-text-decoration-color, -webkit-backdrop-filter;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}

.hover\:border-blue-500:hover {
  --tw-border-opacity: 1;
  border-color: rgb(59 130 246 / var(--tw-border-opacity));
}

.hover\:bg-yellow-50:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(254 252 232 / var(--tw-bg-opacity));
}

.hover\:bg-yellow-600:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(202 138 4 / var(--tw-bg-opacity));
}

.hover\:bg-blue-400:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(96 165 250 / var(--tw-bg-opacity));
}

.hover\:bg-blue-600:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(37 99 235 / var(--tw-bg-opacity));
}

.hover\:bg-blue-500:hover {
  --tw-bg-opacity: 1;
  background-color: rgb(59 130 246 / var(--tw-bg-opacity));
}

.hover\:underline:hover {
  -webkit-text-decoration-line: underline;
          text-decoration-line: underline;
}

.hover\:decoration-red-500:hover {
  -webkit-text-decoration-color: #ef4444;
          text-decoration-color: #ef4444;
}

.hover\:decoration-wavy:hover {
  -webkit-text-decoration-style: wavy;
          text-decoration-style: wavy;
}

.hover\:decoration-2:hover {
  text-decoration-thickness: 2px;
}

.hover\:grayscale-0:hover {
  --tw-grayscale: grayscale(0);
  filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);
}

.focus\:bg-blue-400:focus {
  --tw-bg-opacity: 1;
  background-color: rgb(96 165 250 / var(--tw-bg-opacity));
}

.focus\:ring-blue-500:focus {
  --tw-ring-opacity: 1;
  --tw-ring-color: rgb(59 130 246 / var(--tw-ring-opacity));
}

.focus\:grayscale-0:focus {
  --tw-grayscale: grayscale(0);
  filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);
}

.active\:bg-blue-600:active {
  --tw-bg-opacity: 1;
  background-color: rgb(37 99 235 / var(--tw-bg-opacity));
}

@media (min-width: 640px) {
  .sm\:flex {
    display: flex;
  }

  .sm\:inline-grid {
    display: inline-grid;
  }

  .sm\:max-w-3xl {
    max-width: 48rem;
  }

  .sm\:max-w-none {
    max-width: none;
  }

  .sm\:grid-cols-2 {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .sm\:grid-cols-1 {
    grid-template-columns: repeat(1, minmax(0, 1fr));
  }

  .sm\:items-center {
    align-items: center;
  }

  .sm\:justify-center {
    justify-content: center;
  }

  .sm\:gap-5 {
    gap: 1.25rem;
  }

  .sm\:space-y-0 > :not([hidden]) ~ :not([hidden]) {
    --tw-space-y-reverse: 0;
    margin-top: calc(0px * calc(1 - var(--tw-space-y-reverse)));
    margin-bottom: calc(0px * var(--tw-space-y-reverse));
  }

  .sm\:overflow-hidden {
    overflow: hidden;
  }

  .sm\:rounded-2xl {
    border-radius: 1rem;
  }

  .sm\:px-6 {
    padding-left: 1.5rem;
    padding-right: 1.5rem;
  }

  .sm\:px-8 {
    padding-left: 2rem;
    padding-right: 2rem;
  }

  .sm\:pb-16 {
    padding-bottom: 4rem;
  }

  .sm\:pt-8 {
    padding-top: 2rem;
  }

  .sm\:pt-24 {
    padding-top: 6rem;
  }

  .sm\:pb-14 {
    padding-bottom: 3.5rem;
  }

  .sm\:text-8xl {
    font-size: 6rem;
    line-height: 1;
  }
}

@media (min-width: 768px) {
  .md\:max-w-\[16rem\] {
    max-width: 16rem;
  }

  .md\:grid-cols-2 {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (min-width: 1024px) {
  .lg\:grid-cols-3 {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .lg\:px-8 {
    padding-left: 2rem;
    padding-right: 2rem;
  }

  .lg\:pt-32 {
    padding-top: 8rem;
  }

  .lg\:text-9xl {
    font-size: 8rem;
    line-height: 1;
  }
}

@media (min-width: 1280px) {
  .xl\:grid-cols-4 {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }
}"#
        );
    }
}*/
