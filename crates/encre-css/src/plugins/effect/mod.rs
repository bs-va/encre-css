//! Effect utilities
pub mod background_blend_mode;
pub mod box_shadow;
pub mod box_shadow_color;
pub mod mix_blend_mode;
pub mod opacity;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn background_blend_mode() {
        assert_eq!(
            generate(["bg-blend-multiply"], &base_config()),
            ".bg-blend-multiply {
  background-blend-mode: multiply;
}"
        );
        assert_eq!(
            generate(["bg-blend-luminosity"], &base_config()),
            ".bg-blend-luminosity {
  background-blend-mode: luminosity;
}"
        );
    }

    #[test]
    fn box_shadow() {
        assert_eq!(
            generate(["shadow-sm"], &base_config()),
            ".shadow-sm {
  --en-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
  --en-shadow-colored: 0 1px 3px 0 var(--en-shadow-color), 0 1px 2px -1px var(--en-shadow-color);
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            generate(["shadow-2xl"], &base_config()),
            ".shadow-2xl {
  --en-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
  --en-shadow-colored: 0 25px 50px -12px var(--en-shadow-color);
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            generate(["shadow-inner"], &base_config()),
            ".shadow-inner {
  --en-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);
  --en-shadow-colored: inset 0 2px 4px 0 var(--en-shadow-color);
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);
}"
        );
        assert_eq!(
            generate(["shadow-none"], &base_config()),
            ".shadow-none {
  box-shadow: none;
}"
        );
        assert_eq!(
            generate(["shadow-[12px_3rem_12rem_2em_#f00]"], &base_config()),
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
            generate(["shadow-red-400"], &base_config()),
            ".shadow-red-400 {
  --en-shadow-color: oklch(70.4% .191 22.216);
  --en-shadow: var(--en-shadow-colored);
}"
        );
        assert_eq!(
            generate(["shadow-[rgb(12,12,12)]"], &base_config()),
            r".shadow-\[rgb\(12\,12\,12\)\] {
  --en-shadow-color: rgb(12,12,12);
  --en-shadow: var(--en-shadow-colored);
}"
        );
    }

    #[test]
    fn mix_blend_mode() {
        assert_eq!(
            generate(["mix-blend-multiply"], &base_config()),
            ".mix-blend-multiply {
  mix-blend-mode: multiply;
}"
        );
        assert_eq!(
            generate(["mix-blend-plus-lighter"], &base_config()),
            ".mix-blend-plus-lighter {
  mix-blend-mode: plus-lighter;
}"
        );
    }

    #[test]
    fn opacity() {
        assert_eq!(
            generate(["opacity-12"], &base_config()),
            ".opacity-12 {
  opacity: 0.12;
}"
        );
    }
}
