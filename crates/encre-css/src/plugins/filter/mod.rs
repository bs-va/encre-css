//! Filter utilities
pub mod backdrop_blur;
pub mod backdrop_brightness;
pub mod backdrop_contrast;
pub mod backdrop_filter;
pub mod backdrop_grayscale;
pub mod backdrop_hue_rotate;
pub mod backdrop_invert;
pub mod backdrop_opacity;
pub mod backdrop_saturate;
pub mod backdrop_sepia;
pub mod blur;
pub mod brightness;
pub mod contrast;
pub mod drop_shadow;
pub mod filter_type;
pub mod grayscale;
pub mod hue_rotate;
pub mod invert;
pub mod saturate;
pub mod sepia;

const CSS_FILTER: &str = "filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);";
const CSS_BACKDROP_FILTER_1: &str = "-webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);";
const CSS_BACKDROP_FILTER_2: &str = "backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);";
