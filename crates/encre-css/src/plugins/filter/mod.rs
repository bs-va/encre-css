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
const CSS_BACKDROP_FILTER: [&str; 2] = ["-webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);", "backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);"];

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn blur() {
        assert_eq!(
            testing::generate_css("blur-md"),
            ".blur-md {
  --en-blur: blur(12px);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("blur-none"),
            ".blur-none {
  --en-blur: blur(0);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("blur-[42px]"),
            r".blur-\[42px\] {
  --en-blur: blur(42px);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn brightness() {
        assert_eq!(
            testing::generate_css("brightness-20"),
            ".brightness-20 {
  --en-brightness: brightness(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn contrast() {
        assert_eq!(
            testing::generate_css("contrast-20"),
            ".contrast-20 {
  --en-contrast: contrast(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn drop_shadow() {
        assert_eq!(
            testing::generate_css("drop-shadow-md"),
            ".drop-shadow-md {
  --en-drop-shadow: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("drop-shadow-none"),
            ".drop-shadow-none {
  --en-drop-shadow: drop-shadow(0 0 #0000);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("drop-shadow-[42px_12em_#f0f]"),
            r".drop-shadow-\[42px_12em_\#f0f\] {
  --en-drop-shadow: drop-shadow(42px 12em #f0f);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn filter_type() {
        assert_eq!(
            testing::generate_css("filter"),
            ".filter {
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            testing::generate_css("filter-none"),
            ".filter-none {
  filter: none;
}"
        );
    }

    #[test]
    fn grayscale() {
        assert_eq!(
            testing::generate_css("grayscale-20"),
            ".grayscale-20 {
  --en-grayscale: grayscale(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            testing::generate_css("grayscale"),
            ".grayscale {
  --en-grayscale: grayscale(100%);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn hue_rotate() {
        assert_eq!(
            testing::generate_css("hue-rotate-170"),
            ".hue-rotate-170 {
  --en-hue-rotate: hue-rotate(170deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("-hue-rotate-170"),
            ".-hue-rotate-170 {
  --en-hue-rotate: hue-rotate(-170deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn invert() {
        assert_eq!(
            testing::generate_css("invert-20"),
            ".invert-20 {
  --en-invert: invert(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            testing::generate_css("invert"),
            ".invert {
  --en-invert: invert(100%);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn saturate() {
        assert_eq!(
            testing::generate_css("saturate-20"),
            ".saturate-20 {
  --en-saturate: saturate(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn sepia() {
        assert_eq!(
            testing::generate_css("sepia-20"),
            ".sepia-20 {
  --en-sepia: sepia(0.2);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );

        assert_eq!(
            testing::generate_css("sepia"),
            ".sepia {
  --en-sepia: sepia(100%);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
        );
    }

    #[test]
    fn backdrop_blur() {
        assert_eq!(
            testing::generate_css("backdrop-blur-md"),

            ".backdrop-blur-md {
  --en-backdrop-blur: blur(12px);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
        assert_eq!(
            testing::generate_css("backdrop-blur-none"),
            ".backdrop-blur-none {
  --en-backdrop-blur: blur(0);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
        assert_eq!(
            testing::generate_css("backdrop-blur-[42px]"),
            r".backdrop-blur-\[42px\] {
  --en-backdrop-blur: blur(42px);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_brightness() {
        assert_eq!(
            testing::generate_css("backdrop-brightness-20"),
            ".backdrop-brightness-20 {
  --en-backdrop-brightness: brightness(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_contrast() {
        assert_eq!(
            testing::generate_css("backdrop-contrast-20"),
            ".backdrop-contrast-20 {
  --en-backdrop-contrast: contrast(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_filter() {
        assert_eq!(
            testing::generate_css("backdrop-filter"),
            ".backdrop-filter {
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            testing::generate_css("backdrop-filter-none"),
            ".backdrop-filter-none {
  -webkit-backdrop-filter: none;
  backdrop-filter: none;
}"
        );
    }

    #[test]
    fn backdrop_grayscale() {
        assert_eq!(
            testing::generate_css("backdrop-grayscale-20"),
            ".backdrop-grayscale-20 {
  --en-backdrop-grayscale: grayscale(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            testing::generate_css("backdrop-grayscale"),
            ".backdrop-grayscale {
  --en-backdrop-grayscale: grayscale(100%);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_hue_rotate() {
        assert_eq!(
            testing::generate_css("backdrop-hue-rotate-170"),
            ".backdrop-hue-rotate-170 {
  --en-backdrop-hue-rotate: hue-rotate(170deg);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
        assert_eq!(
            testing::generate_css("-backdrop-hue-rotate-170"),
            ".-backdrop-hue-rotate-170 {
  --en-backdrop-hue-rotate: hue-rotate(-170deg);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_invert() {
        assert_eq!(
            testing::generate_css("backdrop-invert-20"),
            ".backdrop-invert-20 {
  --en-backdrop-invert: invert(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            testing::generate_css("backdrop-invert"),
            ".backdrop-invert {
  --en-backdrop-invert: invert(100%);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_opacity() {
        assert_eq!(
            testing::generate_css("backdrop-opacity-12"),
            ".backdrop-opacity-12 {
  --en-backdrop-opacity: 0.12;
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_saturate() {
        assert_eq!(
            testing::generate_css("backdrop-saturate-20"),
            ".backdrop-saturate-20 {
  --en-backdrop-saturate: saturate(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }

    #[test]
    fn backdrop_sepia() {
        assert_eq!(
            testing::generate_css("backdrop-sepia-20"),
            ".backdrop-sepia-20 {
  --en-backdrop-sepia: sepia(0.2);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );

        assert_eq!(
            testing::generate_css("backdrop-sepia"),
            ".backdrop-sepia {
  --en-backdrop-sepia: sepia(100%);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}"
        );
    }
}
