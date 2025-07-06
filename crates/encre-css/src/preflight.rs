//! Define the default set of base CSS styles used to make websites consistent across browsers.
//!
//! By default, this base CSS is included in the generated CSS and you can customize it
//! by manually setting the [`Config::preflight`] configuration field to `Preflight::new_full()`
//! and by using the various associated methods, e.g [`Preflight::font_family_sans`]
//! or [`Preflight::font_family_mono`].
//!
//! ```
//! use encre_css::{Preflight, Config};
//!
//! let mut config = Config::default();
//! config.preflight = Preflight::new_full()
//!     .font_family_mono("'Fira Code'");
//!
//! assert!(encre_css::generate([], &config).contains("code, kbd, samp, pre {
//!   font-family: 'Fira Code';"));
//! ```
//!
//! You can also use your own default CSS using [`Preflight::new_custom`].
//!
//! ```
//! use encre_css::{Preflight, Config};
//!
//! let mut config = Config::default();
//! config.preflight = Preflight::new_custom("html, body {
//!   width: 100vw;
//!   height: 100vh;
//!   margin: 0;
//! }");
//!
//! assert_eq!(encre_css::generate([], &config), "html, body {
//!   width: 100vw;
//!   height: 100vh;
//!   margin: 0;
//! }");
//! ```
//!
//! Finally you can disable it using [`Preflight::new_none`].
//!
//! ```
//! use encre_css::{Preflight, Config};
//!
//! let mut config = Config::default();
//! config.preflight = Preflight::new_none();
//!
//! assert_eq!(encre_css::generate([], &config), "");
//! ```
//!
//! Based on [Tailwind's default preflight](https://tailwindcss.com/docs/preflight).
//!
//! [`Config::preflight`]: crate::config::Config::preflight
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

const DEFAULT_FONT_FEATURE_SETTINGS: &str = "normal";
const DEFAULT_FONT_VARIATION_SETTINGS: &str = "normal";
const DEFAULT_FONT_FAMILY_SANS: &str = "ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'";
const DEFAULT_FONT_FAMILY_MONO: &str = "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace";

const DEFAULT_PREFLIGHT: &str = concat!(
    /*
    1. Prevent padding and border from affecting element width. (https://github.com/mozdevs/cssremedy/issues/4)
    2. Remove default margins and padding
    3. Reset all borders.
    */
    r#"*, ::after, ::before, ::backdrop, ::file-selector-button {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  border: 0 solid;
}

::before, ::after {
  --en-content: '';
}

"#,
    /*
    1. Use a consistent sensible line-height in all browsers.
    2. Prevent adjustments of font size after orientation changes in iOS.
    3. Use a more readable tab size.
    4. Use the user's configured `sans` font-family by default.
    5. Use the user's configured `sans` font-feature-settings by default.
    6. Use the user's configured `sans` font-variation-settings by default.
    7. Disable tap highlights on iOS.
    */
    r#"html, :host {
  line-height: 1.5;
  -webkit-text-size-adjust: 100%;
  tab-size: 4;
  font-family: theme('fontFamily.sans');
  font-feature-settings: theme('fontFeatureSettings.sans');
  font-variation-settings: theme('fontVariationSettings.sans');
  -webkit-tap-highlight-color: transparent;
}

"#,
    /*
    1. Add the correct height in Firefox.
    2. Correct the inheritance of border color in Firefox. (https://bugzilla.mozilla.org/show_bug.cgi?id=190655)
    3. Reset the default border style to a 1px solid border.
    */
    r#"hr {
  height: 0;
  color: inherit;
  border-top-width: 1px;
}

"#,
    // Add the correct text decoration in Chrome, Edge, and Safari.
    r#"abbr:where([title]) {
  -webkit-text-decoration: underline dotted;
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
  -webkit-text-decoration: inherit;
  text-decoration: inherit;
}

"#,
    // Add the correct font weight in Edge and Safari.
    r#"b, strong {
  font-weight: bolder;
}

"#,
    /*
    1. Use the user's configured `mono` font-family by default.
    2. Use the user's configured `mono` font-feature-settings by default.
    3. Use the user's configured `mono` font-variation-settings by default.
    4. Correct the odd `em` font sizing in all browsers.
    */
    r#"code, kbd, samp, pre {
  font-family: theme('fontFamily.mono');
  font-feature-settings: theme('fontFeatureSettings.mono');
  font-variation-settings: theme('fontVariationSettings.mono');
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

"#,
    r#"sub {
  bottom: -0.25em;
}

"#,
    r#"sup {
  top: -0.5em;
}

"#,
    /*
    1. Remove text indentation from table contents in Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=999088, https://bugs.webkit.org/show_bug.cgi?id=201297)
    2. Correct table border color inheritance in all Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=935729, https://bugs.webkit.org/show_bug.cgi?id=195016)
    3. Remove gaps between table borders by default.
    */
    r#"table {
  text-indent: 0;
  border-color: inherit;
  border-collapse: collapse;
}

"#,
    // Use the modern Firefox focus style for all focusable elements.
    r#":-moz-focusring {
  outline: auto;
}

"#,
    // Add the correct vertical alignment in Chrome and Firefox.
    r#"progress {
  vertical-align: baseline;
}

"#,
    // Add the correct display in Chrome and Safari.
    r#"summary {
  display: list-item;
}

"#,
    // Make lists unstyled by default.
    r#"ol, ul, menu {
  list-style: none;
}

"#,
    /*
    1. Make replaced elements `display: block` by default. (https://github.com/mozdevs/cssremedy/issues/14)
    2. Add `vertical-align: middle` to align replaced elements more sensibly by default. (https://github.com/jensimmons/cssremedy/issues/14#issuecomment-634934210)
        This can trigger a poorly considered lint error in some tools but is included by design.
    */
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
    /*
    1. Inherit font styles in all browsers.
    2. Remove border radius in all browsers.
    3. Remove background color in all browsers.
    4. Ensure consistent opacity for disabled states in all browsers.
    */
    r#"button, input, select, optgroup, textarea, ::file-selector-button {
  font: inherit;
  font-feature-settings: inherit;
  font-variation-settings: inherit;
  letter-spacing: inherit;
  color: inherit;
  border-radius: 0;
  background-color: transparent;
  opacity: 1;
}

"#,
    // Restore default font weight.
    r#":where(select:is([multiple], [size])) optgroup {
  font-weight: bolder;
}

"#,
    // Restore indentation.
    r#":where(select:is([multiple], [size])) optgroup option {
  padding-inline-start: 20px;
}

"#,
    // Restore space after button.
    r#"::file-selector-button {
  margin-inline-end: 4px;
}

"#,
    // Reset the default placeholder opacity in Firefox. (https://github.com/tailwindlabs/tailwindcss/issues/3300)
    r#"::placeholder {
  opacity: 1;
}

"#,
    /*
    Set the default placeholder color to a semi-transparent version of the current text color in browsers that do not
    crash when using `color-mix(…)` with `currentcolor`. (https://github.com/tailwindlabs/tailwindcss/issues/17194)
    */
    r#"@supports (not (-webkit-appearance: -apple-pay-button)) /* Not Safari */ or (contain-intrinsic-size: 1px) /* Safari 17+ */ {
  ::placeholder {
    color: color-mix(in oklab, currentcolor 50%, transparent);
  }
}

"#,
    // Prevent resizing textareas horizontally by default.
    r#"textarea {
  resize: vertical;
}

"#,
    // Remove the inner padding in Chrome and Safari on macOS.
    r#"::-webkit-search-decoration {
  -webkit-appearance: none;
}

"#,
    /*
    1. Ensure date/time inputs have the same height when empty in iOS Safari.
    2. Ensure text alignment can be changed on date/time inputs in iOS Safari.
    */
    r#"::-webkit-date-and-time-value {
  min-height: 1lh;
  text-align: inherit;
}

"#,
    // Prevent height from changing on date/time inputs in macOS Safari when the input is set to `display: block`.
    r#"::-webkit-datetime-edit {
  display: inline-flex;
}

"#,
    // Remove excess padding from pseudo-elements in date/time inputs to ensure consistent height across browsers.
    r#"::-webkit-datetime-edit-fields-wrapper {
  padding: 0;
}

"#,
    r#"::-webkit-datetime-edit, ::-webkit-datetime-edit-year-field, ::-webkit-datetime-edit-month-field, ::-webkit-datetime-edit-day-field, ::-webkit-datetime-edit-hour-field, ::-webkit-datetime-edit-minute-field, ::-webkit-datetime-edit-second-field, ::-webkit-datetime-edit-millisecond-field, ::-webkit-datetime-edit-meridiem-field {
  padding-block: 0;
}

"#,
    // Remove the additional `:invalid` styles in Firefox. (https://github.com/mozilla/gecko-dev/blob/2f9eacd9d3d995c937b4251a5557d95d494c9be1/layout/style/res/forms.css#L728-L737)
    r#":-moz-ui-invalid {
  box-shadow: none;
}

"#,
    // Correct the inability to style the border radius in iOS Safari.
    r#"button, input:where([type='button'], [type='reset'], [type='submit']), ::file-selector-button {
  appearance: button;
}

"#,
    // Correct the cursor style of increment and decrement buttons in Safari.
    r#"::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
  height: auto;
}

"#,
    // Make elements with the HTML hidden attribute stay hidden by default.
    r#"[hidden]:where(:not([hidden='until-found'])) {
  display: none !important;
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
  --en-ring-color: currentColor;
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
  --en-ring-color: currentColor;
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
  --en-ring-color: currentColor;
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
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}"#
);

/// The set of default styles.
///
/// See [`crate::preflight`].
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "css")]
pub enum Preflight {
    /// No preflight will be generated.
    None,

    /// A custom preflight will be used.
    Custom(Cow<'static, str>),

    /// The full default preflight will be generated with some configuration options.
    Full {
        /// Set the [font feature settings](https://developer.mozilla.org/en-US/docs/Web/CSS/font-feature-settings) of the whole document for the sans-serif font.
        font_feature_settings_sans: Option<Cow<'static, str>>,

        /// Set the [font variation settings](https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings) of the whole document for the sans-serif font.
        font_variation_settings_sans: Option<Cow<'static, str>>,

        /// Set the [font feature settings](https://developer.mozilla.org/en-US/docs/Web/CSS/font-feature-settings) of the whole document for the monospace font.
        font_feature_settings_mono: Option<Cow<'static, str>>,

        /// Set the [font variation settings](https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings) of the whole document for the monospace font.
        font_variation_settings_mono: Option<Cow<'static, str>>,

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
            font_feature_settings_sans: None,
            font_variation_settings_sans: None,
            font_feature_settings_mono: None,
            font_variation_settings_mono: None,
            font_family_sans: None,
            font_family_mono: None,
        }
    }

    /// Set the font feature settings of the whole document for the sans-serif font.
    ///
    /// The default value is `normal`.
    #[must_use]
    pub fn font_feature_settings_sans<T: Into<Cow<'static, str>>>(
        mut self,
        new_font_feature_settings: T,
    ) -> Self {
        if let Self::Full {
            ref mut font_feature_settings_sans,
            ..
        } = self
        {
            *font_feature_settings_sans = Some(new_font_feature_settings.into());
        }

        self
    }

    /// Set the font variation settings of the whole document for the sans-serif font.
    ///
    /// The default value is `normal`.
    #[must_use]
    pub fn font_variation_settings_sans<T: Into<Cow<'static, str>>>(
        mut self,
        new_font_variation_settings: T,
    ) -> Self {
        if let Self::Full {
            ref mut font_variation_settings_sans,
            ..
        } = self
        {
            *font_variation_settings_sans = Some(new_font_variation_settings.into());
        }

        self
    }

    /// Set the font feature settings of the whole document for the monospace font.
    ///
    /// The default value is `normal`.
    #[must_use]
    pub fn font_feature_settings_mono<T: Into<Cow<'static, str>>>(
        mut self,
        new_font_feature_settings: T,
    ) -> Self {
        if let Self::Full {
            ref mut font_feature_settings_mono,
            ..
        } = self
        {
            *font_feature_settings_mono = Some(new_font_feature_settings.into());
        }

        self
    }

    /// Set the font variation settings of the whole document for the monospace font.
    ///
    /// The default value is `normal`.
    #[must_use]
    pub fn font_variation_settings_mono<T: Into<Cow<'static, str>>>(
        mut self,
        new_font_variation_settings: T,
    ) -> Self {
        if let Self::Full {
            ref mut font_variation_settings_mono,
            ..
        } = self
        {
            *font_variation_settings_mono = Some(new_font_variation_settings.into());
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
                font_feature_settings_sans,
                font_variation_settings_sans,
                font_feature_settings_mono,
                font_variation_settings_mono,
                font_family_sans,
                font_family_mono,
            } => Cow::from(
                DEFAULT_PREFLIGHT
                    .replace(
                        "theme('fontFeatureSettings.sans')",
                        font_feature_settings_sans
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_FONT_FEATURE_SETTINGS)),
                    )
                    .replace(
                        "theme('fontVariationSettings.sans')",
                        font_variation_settings_sans
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_FONT_VARIATION_SETTINGS)),
                    )
                    .replace(
                        "theme('fontFeatureSettings.mono')",
                        font_feature_settings_mono
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_FONT_FEATURE_SETTINGS)),
                    )
                    .replace(
                        "theme('fontVariationSettings.mono')",
                        font_variation_settings_mono
                            .as_ref()
                            .unwrap_or(&Cow::Borrowed(DEFAULT_FONT_VARIATION_SETTINGS)),
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
            font_feature_settings_sans: None,
            font_variation_settings_sans: None,
            font_feature_settings_mono: None,
            font_variation_settings_mono: None,
            font_family_sans: None,
            font_family_mono: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate, Config};

    use pretty_assertions::assert_eq;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn full_preflight() {
        let preflight = Preflight::new_full()
            .font_feature_settings_sans("tnum")
            .font_variation_settings_sans("'xhgt' 0.7")
            .font_feature_settings_mono("'liga' 0")
            .font_variation_settings_mono("'whgt' 850")
            .font_family_sans("sans-serif")
            .font_family_mono("monospace");
        let config = Config {
            preflight,
            ..Default::default()
        };

        let generated = generate(["w-full"], &config);

        assert_eq!(
            generated,
            String::from(
                r#"*, ::after, ::before, ::backdrop, ::file-selector-button {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  border: 0 solid;
}

::before, ::after {
  --en-content: '';
}

html, :host {
  line-height: 1.5;
  -webkit-text-size-adjust: 100%;
  tab-size: 4;
  font-family: sans-serif;
  font-feature-settings: tnum;
  font-variation-settings: 'xhgt' 0.7;
  -webkit-tap-highlight-color: transparent;
}

hr {
  height: 0;
  color: inherit;
  border-top-width: 1px;
}

abbr:where([title]) {
  -webkit-text-decoration: underline dotted;
  text-decoration: underline dotted;
}

h1, h2, h3, h4, h5, h6 {
  font-size: inherit;
  font-weight: inherit;
}

a {
  color: inherit;
  -webkit-text-decoration: inherit;
  text-decoration: inherit;
}

b, strong {
  font-weight: bolder;
}

code, kbd, samp, pre {
  font-family: monospace;
  font-feature-settings: 'liga' 0;
  font-variation-settings: 'whgt' 850;
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

:-moz-focusring {
  outline: auto;
}

progress {
  vertical-align: baseline;
}

summary {
  display: list-item;
}

ol, ul, menu {
  list-style: none;
}

img, svg, video, canvas, audio, iframe, embed, object {
  display: block;
  vertical-align: middle;
}

img, video {
  max-width: 100%;
  height: auto;
}

button, input, select, optgroup, textarea, ::file-selector-button {
  font: inherit;
  font-feature-settings: inherit;
  font-variation-settings: inherit;
  letter-spacing: inherit;
  color: inherit;
  border-radius: 0;
  background-color: transparent;
  opacity: 1;
}

:where(select:is([multiple], [size])) optgroup {
  font-weight: bolder;
}

:where(select:is([multiple], [size])) optgroup option {
  padding-inline-start: 20px;
}

::file-selector-button {
  margin-inline-end: 4px;
}

::placeholder {
  opacity: 1;
}

@supports (not (-webkit-appearance: -apple-pay-button)) /* Not Safari */ or (contain-intrinsic-size: 1px) /* Safari 17+ */ {
  ::placeholder {
    color: color-mix(in oklab, currentcolor 50%, transparent);
  }
}

textarea {
  resize: vertical;
}

::-webkit-search-decoration {
  -webkit-appearance: none;
}

::-webkit-date-and-time-value {
  min-height: 1lh;
  text-align: inherit;
}

::-webkit-datetime-edit {
  display: inline-flex;
}

::-webkit-datetime-edit-fields-wrapper {
  padding: 0;
}

::-webkit-datetime-edit, ::-webkit-datetime-edit-year-field, ::-webkit-datetime-edit-month-field, ::-webkit-datetime-edit-day-field, ::-webkit-datetime-edit-hour-field, ::-webkit-datetime-edit-minute-field, ::-webkit-datetime-edit-second-field, ::-webkit-datetime-edit-millisecond-field, ::-webkit-datetime-edit-meridiem-field {
  padding-block: 0;
}

:-moz-ui-invalid {
  box-shadow: none;
}

button, input:where([type='button'], [type='reset'], [type='submit']), ::file-selector-button {
  appearance: button;
}

::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
  height: auto;
}

[hidden]:where(:not([hidden='until-found'])) {
  display: none !important;
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
  --en-ring-color: currentColor;
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
  --en-ring-color: currentColor;
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
  --en-ring-color: currentColor;
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
  --en-backdrop-saturate: ;
  --en-backdrop-sepia: ;
}

.w-full {
  width: 100%;
}"#
            )
        );
    }

    #[test]
    fn custom_preflight() {
        let preflight = Preflight::new_custom(
            "html, body {
  width: 100%;
  height: 100%;
  padding: 0;
  margin: 0;
  overflow-x: hidden;
}",
        );
        let config = Config {
            preflight,
            ..Default::default()
        };

        let generated = generate(["w-full"], &config);

        assert_eq!(
            generated,
            "html, body {
  width: 100%;
  height: 100%;
  padding: 0;
  margin: 0;
  overflow-x: hidden;
}

.w-full {
  width: 100%;
}"
        );
    }

    #[test]
    fn no_preflight() {
        let config = Config {
            preflight: Preflight::new_none(),
            ..Default::default()
        };

        let generated = generate(["w-full"], &config);

        assert_eq!(
            generated,
            ".w-full {
  width: 100%;
}"
        );
    }
}
