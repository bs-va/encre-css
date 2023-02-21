//! Effect utilities
pub mod background_blend_mode;
pub mod box_shadow;
pub mod box_shadow_color;
pub mod mix_blend_mode;
pub mod opacity;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn background_blend_mode() {
        assert_eq!(
            testing::generate_css("bg-blend-multiply"),
            ".bg-blend-multiply {
  background-blend-mode: multiply;
}"
        );
        assert_eq!(
            testing::generate_css("bg-blend-luminosity"),
            ".bg-blend-luminosity {
  background-blend-mode: luminosity;
}"
        );
    }

    #[test]
    fn box_shadow() {
        assert_eq!(
            testing::generate_css("shadow"),
            ".shadow {
  --en-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
  --en-shadow-colored: 0 1px 3px 0 var(--en-shadow-color), 0 1px 2px -1px var(--en-shadow-color);
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("shadow-2xl"),
            ".shadow-2xl {
  --en-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
  --en-shadow-colored: 0 25px 50px -12px var(--en-shadow-color);
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("shadow-inner"),
            ".shadow-inner {
  --en-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);
  --en-shadow-colored: inset 0 2px 4px 0 var(--en-shadow-color);
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            testing::generate_css("shadow-none"),
            ".shadow-none {
  box-shadow: none;
}"
        );
        assert_eq!(
            testing::generate_css("shadow-[12px_3rem_12rem_2em_#f00]"),
            r".shadow-\[12px_3rem_12rem_2em_\#f00\] {
  --en-shadow: 12px 3rem 12rem 2em #f00;
  --en-shadow-colored: 12px 3rem 12rem 2em var(--en-shadow-color);
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
    }

    #[test]
    fn box_shadow_color() {
        assert_eq!(
            testing::generate_css("shadow-red-400"),
            ".shadow-red-400 {
  --en-shadow-color: rgb(248 113 113);
  --en-shadow: var(--en-shadow-colored);
}"
        );
        assert_eq!(
            testing::generate_css("shadow-[rgb(12,12,12)]"),
            r".shadow-\[rgb\(12\,12\,12\)\] {
  --en-shadow-color: rgb(12,12,12);
  --en-shadow: var(--en-shadow-colored);
}"
        );
    }

    #[test]
    fn mix_blend_mode() {
        assert_eq!(
            testing::generate_css("mix-blend-multiply"),
            ".mix-blend-multiply {
  mix-blend-mode: multiply;
}"
        );
        assert_eq!(
            testing::generate_css("mix-blend-plus-lighter"),
            ".mix-blend-plus-lighter {
  mix-blend-mode: plus-lighter;
}"
        );
    }

    #[test]
    fn opacity() {
        assert_eq!(
            testing::generate_css("opacity-12"),
            ".opacity-12 {
  opacity: 0.12;
}"
        );
    }
}
