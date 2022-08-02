//! Define the default set of base CSS styles used to make websites consistent across browsers.
//!
//! By default, this base CSS is included in the generated CSS and you can customize it
//! by manually setting the [`Config::preflight`] configuration field to `Preflight::new_full()`
//! and by using [`Preflight::ring_color`], [`Preflight::border_color`],
//! [`Preflight::placeholder_color`], [`Preflight::font_family_sans`]
//! or [`Preflight::font_family_mono`].
//!
//! ```rust
//! use encre_css::{Preflight, Config, EncreGenerator};
//!
//! let mut config = Config::default();
//! config.preflight = Preflight::new_full()
//!     .border_color("#444");
//!
//! let mut generator = EncreGenerator::from_config(config);
//! assert!(generator.generate().contains("*, ::before, ::after {
//!   box-sizing: border-box;
//!   border-width: 0;
//!   border-style: solid;
//!   border-color: #444;
//! }"));
//! ```
//!
//! You can also use your own default CSS using [`Preflight::new_custom`].
//!
//! ```rust
//! use encre_css::{Preflight, Config, EncreGenerator};
//!
//! let mut config = Config::default();
//! config.preflight = Preflight::new_custom("html, body {
//!   width: 100vw;
//!   height: 100vh;
//!   margin: 0;
//! }");
//!
//! let mut generator = EncreGenerator::from_config(config);
//! assert_eq!(generator.generate(), "html, body {
//!   width: 100vw;
//!   height: 100vh;
//!   margin: 0;
//! }");
//! ```
//!
//! Finally you can disable it using [`Preflight::new_none`].
//!
//! ```rust
//! use encre_css::{Preflight, Config, EncreGenerator};
//!
//! let mut config = Config::default();
//! config.preflight = Preflight::new_none();
//!
//! let mut generator = EncreGenerator::from_config(config);
//! assert_eq!(generator.generate(), "");
//! ```
//!
//! Based on [Tailwind's default preflight](https://tailwindcss.com/docs/preflight).
//!
//! [`Config::preflight`]: crate::config::Config::preflight
use serde::Deserialize;
use std::borrow::Cow;

const DEFAULT_RING_COLOR: &str = "rgb(59 130 246 / 0.5)";
const DEFAULT_BORDER_COLOR: &str = "currentColor";
const DEFAULT_PLACEHOLDER_COLOR: &str = "#9ca3af";
const DEFAULT_FONT_FAMILY_SANS: &str = r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji""#;
const DEFAULT_FONT_FAMILY_MONO: &str = r#"ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace"#;

const DEFAULT_PREFLIGHT: &str = concat!(
    // 1. Prevent padding and border from affecting element width.
    // 2. Allow adding a border to an element by just adding a border-width.
    r#"*, ::before, ::after {
  box-sizing: border-box;
  border-width: 0;
  border-style: solid;
  border-color: theme('borderColor');
}

::before, ::after {
  --en-content: '';
}

"#,
    // 1. Use a consistent sensible line-height in all browsers.
    // 2. Prevent adjustments of font size after orientation changes in iOS.
    // 3. Use a more readable tab size.
    // 4. Use the user's configured `sans` font-family by default.
    r#"html {
  line-height: 1.5;
  -webkit-text-size-adjust: 100%;
  -moz-tab-size: 4;
  tab-size: 4;
  font-family: theme('fontFamily.sans');
}

"#,
    // 1. Remove the margin in all browsers.
    // 2. Inherit line-height from `html` so users can set them as a class directly on the `html` element.
    r#"body {
  margin: 0;
  line-height: inherit;
}

"#,
    // 1. Add the correct height in Firefox.
    // 2. Correct the inheritance of border color in Firefox.
    // 3. Ensure horizontal rules are visible by default.
    r#"hr {
  height: 0;
  color: inherit;
  border-top-width: 1px;
}

"#,
    // Add the correct text decoration in Chrome, Edge, and Safari.
    r#"abbr:where([title]) {
  text-decoration: underline dotted;
}

"#,
    // Remove the default font size and weight for headings.
    r#"h1, h2, h3, h4, h5, h6 {
  font-size: inherit;
  font-weight: inherit;
}

"#,
    // Reset links to optimize for opt-in styling instead of opt-out.
    r#"a {
  color: inherit;
  text-decoration: inherit;
}

"#,
    // Add the correct font weight in Edge and Safari.
    r#"b, strong {
  font-weight: bolder;
}

"#,
    // 1. Use the user's configured `mono` font family by default.
    // 2. Correct the odd `em` font sizing in all browsers.
    r#"code, kbd, samp, pre {
  font-family: theme('fontFamily.mono');
  font-size: 1em;
}

"#,
    // Add the correct font size in all browsers.
    r#"small {
  font-size: 80%;
}

"#,
    // Prevent `sub` and `sup` elements from affecting the line height in all browsers.
    r#"sub, sup {
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

"#,
    // 1. Remove text indentation from table contents in Chrome and Safari.
    // 2. Correct table border color inheritance in all Chrome and Safari.
    // 3. Remove gaps between table borders by default.
    r#"table {
  text-indent: 0;
  border-color: inherit;
  border-collapse: collapse;
}

"#,
    // 1. Change the font styles in all browsers.
    // 2. Remove the margin in Firefox and Safari.
    // 3. Remove default padding in all browsers.
    r#"button, input, optgroup, select, textarea {
  font-family: inherit;
  font-size: 100%;
  font-weight: inherit;
  line-height: inherit;
  color: inherit;
  margin: 0;
  padding: 0;
}

"#,
    // Remove the inheritance of text transform in Edge and Firefox.
    r#"button, select {
  text-transform: none;
}

"#,
    // 1. Correct the inability to style clickable types in iOS and Safari.
    // 2. Remove default button styles.
    r#"button, [type='button'], [type='reset'], [type='submit'] {
  -webkit-appearance: button;
  background-color: transparent;
  background-image: none;
}

"#,
    // Use the modern Firefox focus style for all focusable elements.
    r#":-moz-focusring {
  outline: auto;
}

"#,
    // Remove the additional `:invalid` styles in Firefox.
    r#":-moz-ui-invalid {
  box-shadow: none;
}

"#,
    // Add the correct vertical alignment in Chrome and Firefox.
    r#"progress {
  vertical-align: baseline;
}

"#,
    // Correct the cursor style of increment and decrement buttons in Safari.
    r#"::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
  height: auto;
}

"#,
    // 1. Correct the odd appearance in Chrome and Safari.
    // 2. Correct the outline style in Safari.
    r#"[type='search'] {
  -webkit-appearance: textfield;
  outline-offset: -2px;
}

"#,
    // Remove the inner padding in Chrome and Safari on macOS.
    r#"::-webkit-search-decoration {
  -webkit-appearance: none;
}

"#,
    // 1. Correct the inability to style clickable types in iOS and Safari.
    // 2. Change font properties to `inherit` in Safari.
    r#"::-webkit-file-upload-button {
  -webkit-appearance: button;
  font: inherit;
}

"#,
    // Add the correct display in Chrome and Safari.
    r#"summary {
  display: list-item;
}

"#,
    // Removes the default spacing and border for appropriate elements.
    r#"blockquote, dl, dd, h1, h2, h3, h4, h5, h6, hr, figure, p, pre {
  margin: 0;
}

fieldset {
  margin: 0;
  padding: 0;
}

legend {
  padding: 0;
}

ol, ul, menu {
  list-style: none;
  margin: 0;
  padding: 0;
}

"#,
    // Prevent resizing textareas horizontally by default.
    r#"textarea {
  resize: vertical;
}

"#,
    // 1. Reset the default placeholder opacity in Firefox. (https://github.com/tailwindlabs/tailwindcss/issues/3300)
    // 2. Set the default placeholder color to the user's configured gray 400 color.
    r#"input::placeholder, textarea::placeholder {
  opacity: 1;
  color: theme('placeholderColor');
}

"#,
    // Set the default cursor for buttons.
    r#"button, [role="button"] {
  cursor: pointer;
}

"#,
    // Make sure disabled buttons don't get the pointer cursor.
    r#":disabled {
  cursor: default;
}

"#,
    // 1. Make replaced elements `display: block` by default. (https://github.com/mozdevs/cssremedy/issues/14)
    // 2. Add `vertical-align: middle` to align replaced elements more sensibly by default. (https://github.com/jensimmons/cssremedy/issues/14#issuecomment-634934210)
    //    This can trigger a poorly considered lint error in some tools but is included by design.
    r#"img, svg, video, canvas, audio, iframe, embed, object {
  display: block;
  vertical-align: middle;
}

"#,
    // Constrain images and videos to the parent width and preserve their intrinsic aspect ratio. (https://github.com/mozdevs/cssremedy/issues/14)
    r#"img, video {
  max-width: 100%;
  height: auto;
}

"#,
    // Defaults
    r#"*, ::before, ::after {
  --en-border-spacing-x: 0;
  --en-border-spacing-y: 0;
  --en-translate-x: 0;
  --en-translate-y: 0;
  --en-rotate: 0;
  --en-skew-x: 0;
  --en-skew-y: 0;
  --en-scale-x: 1;
  --en-scale-y: 1;
  --en-pan-x: ;
  --en-pan-y: ;
  --en-pinch-zoom: ;
  --en-scroll-snap-strictness: proximity;
  --en-ordinal: ;
  --en-slashed-zero: ;
  --en-numeric-figure: ;
  --en-numeric-spacing: ;
  --en-numeric-fraction: ;
  --en-ring-inset: ;
  --en-ring-offset-width: 0px;
  --en-ring-offset-color: #fff;
  --en-ring-color: theme('ringColor');
  --en-ring-offset-shadow: 0 0 #0000;
  --en-ring-shadow: 0 0 #0000;
  --en-shadow: 0 0 #0000;
  --en-shadow-colored: 0 0 #0000;
  --en-blur: ;
  --en-brightness: ;
  --en-contrast: ;
  --en-grayscale: ;
  --en-hue-rotate: ;
  --en-invert: ;
  --en-saturate: ;
  --en-sepia: ;
  --en-drop-shadow: ;
  --en-backdrop-blur: ;
  --en-backdrop-brightness: ;
  --en-backdrop-contrast: ;
  --en-backdrop-grayscale: ;
  --en-backdrop-hue-rotate: ;
  --en-backdrop-invert: ;
  --en-backdrop-opacity: ;
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}

::-webkit-backdrop {
  --en-border-spacing-x: 0;
  --en-border-spacing-y: 0;
  --en-translate-x: 0;
  --en-translate-y: 0;
  --en-rotate: 0;
  --en-skew-x: 0;
  --en-skew-y: 0;
  --en-scale-x: 1;
  --en-scale-y: 1;
  --en-pan-x: ;
  --en-pan-y: ;
  --en-pinch-zoom: ;
  --en-scroll-snap-strictness: proximity;
  --en-ordinal: ;
  --en-slashed-zero: ;
  --en-numeric-figure: ;
  --en-numeric-spacing: ;
  --en-numeric-fraction: ;
  --en-ring-inset: ;
  --en-ring-offset-width: 0px;
  --en-ring-offset-color: #fff;
  --en-ring-color: theme('ringColor');
  --en-ring-offset-shadow: 0 0 #0000;
  --en-ring-shadow: 0 0 #0000;
  --en-shadow: 0 0 #0000;
  --en-shadow-colored: 0 0 #0000;
  --en-blur: ;
  --en-brightness: ;
  --en-contrast: ;
  --en-grayscale: ;
  --en-hue-rotate: ;
  --en-invert: ;
  --en-saturate: ;
  --en-sepia: ;
  --en-drop-shadow: ;
  --en-backdrop-blur: ;
  --en-backdrop-brightness: ;
  --en-backdrop-contrast: ;
  --en-backdrop-grayscale: ;
  --en-backdrop-hue-rotate: ;
  --en-backdrop-invert: ;
  --en-backdrop-opacity: ;
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}

::backdrop {
  --en-border-spacing-x: 0;
  --en-border-spacing-y: 0;
  --en-translate-x: 0;
  --en-translate-y: 0;
  --en-rotate: 0;
  --en-skew-x: 0;
  --en-skew-y: 0;
  --en-scale-x: 1;
  --en-scale-y: 1;
  --en-pan-x: ;
  --en-pan-y: ;
  --en-pinch-zoom: ;
  --en-scroll-snap-strictness: proximity;
  --en-ordinal: ;
  --en-slashed-zero: ;
  --en-numeric-figure: ;
  --en-numeric-spacing: ;
  --en-numeric-fraction: ;
  --en-ring-inset: ;
  --en-ring-offset-width: 0px;
  --en-ring-offset-color: #fff;
  --en-ring-color: theme('ringColor');
  --en-ring-offset-shadow: 0 0 #0000;
  --en-ring-shadow: 0 0 #0000;
  --en-shadow: 0 0 #0000;
  --en-shadow-colored: 0 0 #0000;
  --en-blur: ;
  --en-brightness: ;
  --en-contrast: ;
  --en-grayscale: ;
  --en-hue-rotate: ;
  --en-invert: ;
  --en-saturate: ;
  --en-sepia: ;
  --en-drop-shadow: ;
  --en-backdrop-blur: ;
  --en-backdrop-brightness: ;
  --en-backdrop-contrast: ;
  --en-backdrop-grayscale: ;
  --en-backdrop-hue-rotate: ;
  --en-backdrop-invert: ;
  --en-backdrop-opacity: ;
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}"#
);

/// The set of default styles.
///
/// See [`crate::preflight`].
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Preflight {
    /// No preflight will be generated.
    None,

    /// A custom preflight will be used.
    Custom(Cow<'static, str>),

    /// The full default preflight will be generated with some configuration options.
    Full {
        /// Set the default ring color.
        ring_color: Option<Cow<'static, str>>,

        /// Set the default border color.
        border_color: Option<Cow<'static, str>>,

        /// Set the default placeholder color.
        placeholder_color: Option<Cow<'static, str>>,

        /// Set the default sans-serif font family.
        font_family_sans: Option<Cow<'static, str>>,

        /// Set the default monospace font family.
        font_family_mono: Option<Cow<'static, str>>,
    },
}

impl Preflight {
    /// Create a new [`Preflight::None`].
    pub fn new_none() -> Self {
        Self::None
    }

    /// Create a new [`Preflight::Custom`].
    pub fn new_custom<T: Into<Cow<'static, str>>>(css: T) -> Self {
        Self::Custom(css.into())
    }

    /// Create a new [`Preflight::Full`] with default values for options.
    pub fn new_full() -> Self {
        Self::Full {
            ring_color: None,
            border_color: None,
            placeholder_color: None,
            font_family_sans: None,
            font_family_mono: None,
        }
    }

    /// Set the default ring color.
    ///
    /// The default value is `rgb(59 130 246 / 0.5)`.
    #[must_use]
    pub fn ring_color<T: Into<Cow<'static, str>>>(mut self, new_ring_color: T) -> Self {
        if let Self::Full {
            ref mut ring_color, ..
        } = self
        {
            *ring_color = Some(new_ring_color.into());
        }

        self
    }

    /// Set the default border color.
    ///
    /// The default value is `currentColor`.
    #[must_use]
    pub fn border_color<T: Into<Cow<'static, str>>>(mut self, new_border_color: T) -> Self {
        if let Self::Full {
            ref mut border_color,
            ..
        } = self
        {
            *border_color = Some(new_border_color.into());
        }

        self
    }

    /// Set the default placeholder color.
    ///
    /// The default value is `#9ca3af`.
    #[must_use]
    pub fn placeholder_color<T: Into<Cow<'static, str>>>(
        mut self,
        new_placeholder_color: T,
    ) -> Self {
        if let Self::Full {
            ref mut placeholder_color,
            ..
        } = self
        {
            *placeholder_color = Some(new_placeholder_color.into());
        }

        self
    }

    /// Set the default sans-serif font family.
    ///
    /// The default value is `ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"`.
    #[must_use]
    pub fn font_family_sans<T: Into<Cow<'static, str>>>(mut self, new_font_family_sans: T) -> Self {
        if let Self::Full {
            ref mut font_family_sans,
            ..
        } = self
        {
            *font_family_sans = Some(new_font_family_sans.into());
        }

        self
    }

    /// Set the default monospace font family.
    ///
    /// The default value is `ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace`.
    #[must_use]
    pub fn font_family_mono<T: Into<Cow<'static, str>>>(mut self, new_font_family_mono: T) -> Self {
        if let Self::Full {
            ref mut font_family_mono,
            ..
        } = self
        {
            *font_family_mono = Some(new_font_family_mono.into());
        }

        self
    }

    pub(crate) fn build(&self) -> Cow<'static, str> {
        match self {
            Self::None => Cow::from(""),
            Self::Custom(css) => css.clone(),
            Self::Full {
                ring_color,
                border_color,
                font_family_sans,
                font_family_mono,
                placeholder_color,
            } => Cow::from(
                DEFAULT_PREFLIGHT
                    .replace(
                        "theme('ringColor')",
                        ring_color
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_RING_COLOR)),
                    )
                    .replace(
                        "theme('borderColor')",
                        border_color
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_BORDER_COLOR)),
                    )
                    .replace(
                        "theme('placeholderColor')",
                        placeholder_color
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_PLACEHOLDER_COLOR)),
                    )
                    .replace(
                        "theme('fontFamily.sans')",
                        font_family_sans
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_FONT_FAMILY_SANS)),
                    )
                    .replace(
                        "theme('fontFamily.mono')",
                        font_family_mono
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_FONT_FAMILY_MONO)),
                    ),
            ),
        }
    }
}

impl Default for Preflight {
    fn default() -> Self {
        Self::Full {
            ring_color: None,
            border_color: None,
            placeholder_color: None,
            font_family_sans: None,
            font_family_mono: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Config, EncreGenerator};

    use pretty_assertions::assert_eq;

    #[test]
    fn full_preflight() {
        let mut config = Config::default();
        config.preflight = Preflight::new_full().border_color("#f00");

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("w-full");

        assert_eq!(
            generator.generate(),
            String::from(
                r#"*, ::before, ::after {
  box-sizing: border-box;
  border-width: 0;
  border-style: solid;
  border-color: #f00;
}

::before, ::after {
  --en-content: '';
}

html {
  line-height: 1.5;
  -webkit-text-size-adjust: 100%;
  -moz-tab-size: 4;
  tab-size: 4;
  font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
}

body {
  margin: 0;
  line-height: inherit;
}

hr {
  height: 0;
  color: inherit;
  border-top-width: 1px;
}

abbr:where([title]) {
  text-decoration: underline dotted;
}

h1, h2, h3, h4, h5, h6 {
  font-size: inherit;
  font-weight: inherit;
}

a {
  color: inherit;
  text-decoration: inherit;
}

b, strong {
  font-weight: bolder;
}

code, kbd, samp, pre {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 1em;
}

small {
  font-size: 80%;
}

sub, sup {
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

table {
  text-indent: 0;
  border-color: inherit;
  border-collapse: collapse;
}

button, input, optgroup, select, textarea {
  font-family: inherit;
  font-size: 100%;
  font-weight: inherit;
  line-height: inherit;
  color: inherit;
  margin: 0;
  padding: 0;
}

button, select {
  text-transform: none;
}

button, [type='button'], [type='reset'], [type='submit'] {
  -webkit-appearance: button;
  background-color: transparent;
  background-image: none;
}

:-moz-focusring {
  outline: auto;
}

:-moz-ui-invalid {
  box-shadow: none;
}

progress {
  vertical-align: baseline;
}

::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
  height: auto;
}

[type='search'] {
  -webkit-appearance: textfield;
  outline-offset: -2px;
}

::-webkit-search-decoration {
  -webkit-appearance: none;
}

::-webkit-file-upload-button {
  -webkit-appearance: button;
  font: inherit;
}

summary {
  display: list-item;
}

blockquote, dl, dd, h1, h2, h3, h4, h5, h6, hr, figure, p, pre {
  margin: 0;
}

fieldset {
  margin: 0;
  padding: 0;
}

legend {
  padding: 0;
}

ol, ul, menu {
  list-style: none;
  margin: 0;
  padding: 0;
}

textarea {
  resize: vertical;
}

input::placeholder, textarea::placeholder {
  opacity: 1;
  color: #9ca3af;
}

button, [role="button"] {
  cursor: pointer;
}

:disabled {
  cursor: default;
}

img, svg, video, canvas, audio, iframe, embed, object {
  display: block;
  vertical-align: middle;
}

img, video {
  max-width: 100%;
  height: auto;
}

*, ::before, ::after {
  --en-border-spacing-x: 0;
  --en-border-spacing-y: 0;
  --en-translate-x: 0;
  --en-translate-y: 0;
  --en-rotate: 0;
  --en-skew-x: 0;
  --en-skew-y: 0;
  --en-scale-x: 1;
  --en-scale-y: 1;
  --en-pan-x: ;
  --en-pan-y: ;
  --en-pinch-zoom: ;
  --en-scroll-snap-strictness: proximity;
  --en-ordinal: ;
  --en-slashed-zero: ;
  --en-numeric-figure: ;
  --en-numeric-spacing: ;
  --en-numeric-fraction: ;
  --en-ring-inset: ;
  --en-ring-offset-width: 0px;
  --en-ring-offset-color: #fff;
  --en-ring-color: rgb(59 130 246 / 0.5);
  --en-ring-offset-shadow: 0 0 #0000;
  --en-ring-shadow: 0 0 #0000;
  --en-shadow: 0 0 #0000;
  --en-shadow-colored: 0 0 #0000;
  --en-blur: ;
  --en-brightness: ;
  --en-contrast: ;
  --en-grayscale: ;
  --en-hue-rotate: ;
  --en-invert: ;
  --en-saturate: ;
  --en-sepia: ;
  --en-drop-shadow: ;
  --en-backdrop-blur: ;
  --en-backdrop-brightness: ;
  --en-backdrop-contrast: ;
  --en-backdrop-grayscale: ;
  --en-backdrop-hue-rotate: ;
  --en-backdrop-invert: ;
  --en-backdrop-opacity: ;
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}

::-webkit-backdrop {
  --en-border-spacing-x: 0;
  --en-border-spacing-y: 0;
  --en-translate-x: 0;
  --en-translate-y: 0;
  --en-rotate: 0;
  --en-skew-x: 0;
  --en-skew-y: 0;
  --en-scale-x: 1;
  --en-scale-y: 1;
  --en-pan-x: ;
  --en-pan-y: ;
  --en-pinch-zoom: ;
  --en-scroll-snap-strictness: proximity;
  --en-ordinal: ;
  --en-slashed-zero: ;
  --en-numeric-figure: ;
  --en-numeric-spacing: ;
  --en-numeric-fraction: ;
  --en-ring-inset: ;
  --en-ring-offset-width: 0px;
  --en-ring-offset-color: #fff;
  --en-ring-color: rgb(59 130 246 / 0.5);
  --en-ring-offset-shadow: 0 0 #0000;
  --en-ring-shadow: 0 0 #0000;
  --en-shadow: 0 0 #0000;
  --en-shadow-colored: 0 0 #0000;
  --en-blur: ;
  --en-brightness: ;
  --en-contrast: ;
  --en-grayscale: ;
  --en-hue-rotate: ;
  --en-invert: ;
  --en-saturate: ;
  --en-sepia: ;
  --en-drop-shadow: ;
  --en-backdrop-blur: ;
  --en-backdrop-brightness: ;
  --en-backdrop-contrast: ;
  --en-backdrop-grayscale: ;
  --en-backdrop-hue-rotate: ;
  --en-backdrop-invert: ;
  --en-backdrop-opacity: ;
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}

::backdrop {
  --en-border-spacing-x: 0;
  --en-border-spacing-y: 0;
  --en-translate-x: 0;
  --en-translate-y: 0;
  --en-rotate: 0;
  --en-skew-x: 0;
  --en-skew-y: 0;
  --en-scale-x: 1;
  --en-scale-y: 1;
  --en-pan-x: ;
  --en-pan-y: ;
  --en-pinch-zoom: ;
  --en-scroll-snap-strictness: proximity;
  --en-ordinal: ;
  --en-slashed-zero: ;
  --en-numeric-figure: ;
  --en-numeric-spacing: ;
  --en-numeric-fraction: ;
  --en-ring-inset: ;
  --en-ring-offset-width: 0px;
  --en-ring-offset-color: #fff;
  --en-ring-color: rgb(59 130 246 / 0.5);
  --en-ring-offset-shadow: 0 0 #0000;
  --en-ring-shadow: 0 0 #0000;
  --en-shadow: 0 0 #0000;
  --en-shadow-colored: 0 0 #0000;
  --en-blur: ;
  --en-brightness: ;
  --en-contrast: ;
  --en-grayscale: ;
  --en-hue-rotate: ;
  --en-invert: ;
  --en-saturate: ;
  --en-sepia: ;
  --en-drop-shadow: ;
  --en-backdrop-blur: ;
  --en-backdrop-brightness: ;
  --en-backdrop-contrast: ;
  --en-backdrop-grayscale: ;
  --en-backdrop-hue-rotate: ;
  --en-backdrop-invert: ;
  --en-backdrop-opacity: ;
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}

.w-full {
  width: 100%;
}"#
            )
        );
    }
}
