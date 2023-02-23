//! Border utilities
pub mod border_color;
pub mod border_opacity;
pub mod border_radius;
pub mod border_style;
pub mod border_width;
pub mod divide_color;
pub mod divide_opacity;
pub mod divide_style;
pub mod divide_width;
pub mod outline_color;
pub mod outline_offset;
pub mod outline_style;
pub mod outline_width;
pub mod ring_color;
pub mod ring_offset_color;
pub mod ring_offset_width;
pub mod ring_opacity;
pub mod ring_width;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn border_color() {
        assert_eq!(
            testing::generate_css("border-red-400"),
            ".border-red-400 {
  --en-border-opacity: 1;
  border-color: rgb(248 113 113 / var(--en-border-opacity));
}"
        );
        assert_eq!(
            testing::generate_css("border-[rgb(12,12,12)]"),
            r".border-\[rgb\(12\,12\,12\)\] {
  border-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn border_opacity() {
        assert_eq!(
            testing::generate_css("border-red-400/12"),
            r".border-red-400\/12 {
  border-color: rgb(248 113 113 / 0.12);
}"
        );
        assert_eq!(
            testing::generate_css("border-opacity-12"),
            ".border-opacity-12 {
  --en-border-opacity: 0.12;
}"
        );
    }

    #[test]
    fn border_style() {
        assert_eq!(
            testing::generate_css("border-dashed"),
            ".border-dashed {
  border-style: dashed;
}"
        );

        assert_eq!(
            testing::generate_css("border-[solid_none_solid_none]"),
            r".border-\[solid_none_solid_none\] {
  border-style: solid none solid none;
}"
        );
    }

    #[test]
    fn border_width() {
        assert_eq!(
            testing::generate_css("border"),
            ".border {
  border-width: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("border-x"),
            ".border-x {
  border-left-width: 1px;
  border-right-width: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("border-t"),
            ".border-t {
  border-top-width: 1px;
}"
        );

        assert_eq!(
            testing::generate_css("border-2"),
            ".border-2 {
  border-width: 2px;
}"
        );
        assert_eq!(
            testing::generate_css("border-x-24"),
            ".border-x-24 {
  border-left-width: 24px;
  border-right-width: 24px;
}"
        );
        assert_eq!(
            testing::generate_css("border-t-42"),
            ".border-t-42 {
  border-top-width: 42px;
}"
        );
        assert_eq!(
            testing::generate_css("border-b-[3rem]"),
            r".border-b-\[3rem\] {
  border-bottom-width: 3rem;
}"
        );
        assert_eq!(
            testing::generate_css("border-y-[thick]"),
            r".border-y-\[thick\] {
  border-top-width: thick;
  border-bottom-width: thick;
}"
        );
    }

    #[test]
    fn border_radius() {
        assert_eq!(
            testing::generate_css("rounded"),
            ".rounded {
  border-radius: 0.25rem;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-[2px]"),
            r".rounded-\[2px\] {
  border-radius: 2px;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-bl"),
            ".rounded-bl {
  border-bottom-left-radius: 0.25rem;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-t"),
            ".rounded-t {
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-xl"),
            ".rounded-xl {
  border-radius: 0.75rem;
}"
        );

        assert_eq!(
            testing::generate_css("rounded-sm"),
            ".rounded-sm {
  border-radius: 0.125rem;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-bl-full"),
            ".rounded-bl-full {
  border-bottom-left-radius: 9999px;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-t-3xl"),
            ".rounded-t-3xl {
  border-top-left-radius: 1.5rem;
  border-top-right-radius: 1.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-b-[3em]"),
            r".rounded-b-\[3em\] {
  border-bottom-left-radius: 3em;
  border-bottom-right-radius: 3em;
}"
        );
        assert_eq!(
            testing::generate_css("rounded-tr-[20%]"),
            r".rounded-tr-\[20\%\] {
  border-top-right-radius: 20%;
}"
        );
    }

    #[test]
    fn divide_style() {
        assert_eq!(
            testing::generate_css("divide-double"),
            ".divide-double > :not([hidden]) ~ :not([hidden]) {
  border-style: double;
}"
        );
    }

    #[test]
    fn divide_color() {
        assert_eq!(
            testing::generate_css("divide-red-400"),
            ".divide-red-400 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-opacity: 1;
  border-color: rgb(248 113 113 / var(--en-divide-opacity));
}"
        );
        assert_eq!(
            testing::generate_css("divide-[rgb(12,12,12)]"),
            r".divide-\[rgb\(12\,12\,12\)\] > :not([hidden]) ~ :not([hidden]) {
  border-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn divide_opacity() {
        assert_eq!(
            testing::generate_css("divide-red-400/42"),
            r".divide-red-400\/42 > :not([hidden]) ~ :not([hidden]) {
  border-color: rgb(248 113 113 / 0.42);
}"
        );

        assert_eq!(
            testing::generate_css("divide-opacity-42"),
            ".divide-opacity-42 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-opacity: 0.42;
}"
        );
    }

    #[test]
    fn divide_width() {
        assert_eq!(
            testing::generate_css("divide-x"),
            ".divide-x > :not([hidden]) ~ :not([hidden]) {
  --en-divide-x-reverse: 0;
  border-right-width: calc(1px * var(--en-divide-x-reverse));
  border-left-width: calc(1px * calc(1 - var(--en-divide-x-reverse)));
}"
        );
        assert_eq!(
            testing::generate_css("divide-y-2"),
            ".divide-y-2 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 0;
  border-top-width: calc(2px * calc(1 - var(--en-divide-y-reverse)));
  border-bottom-width: calc(2px * var(--en-divide-y-reverse));
}"
        );
        assert_eq!(
            testing::generate_css("divide-x-reverse"),
            ".divide-x-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-divide-x-reverse: 1;
}"
        );
        assert_eq!(
            testing::generate_css("divide-y-[0.1rem]"),
            r".divide-y-\[0\.1rem\] > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 0;
  border-top-width: calc(0.1rem * calc(1 - var(--en-divide-y-reverse)));
  border-bottom-width: calc(0.1rem * var(--en-divide-y-reverse));
}"
        );
        assert_eq!(
            testing::generate_css("divide-y-reverse"),
            ".divide-y-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 1;
}"
        );
    }

    #[test]
    fn outline_style() {
        assert_eq!(
            testing::generate_css("outline-none"),
            ".outline-none {
  outline: 2px solid transparent;
  outline-offset: 2px;
}"
        );
        assert_eq!(
            testing::generate_css("outline-dashed"),
            ".outline-dashed {
  outline-style: dashed;
}"
        );
    }

    #[test]
    fn outline_width() {
        assert_eq!(
            testing::generate_css("outline-33"),
            ".outline-33 {
  outline-width: 33px;
}"
        );
        assert_eq!(
            testing::generate_css("outline-[2rem]"),
            r".outline-\[2rem\] {
  outline-width: 2rem;
}"
        );
        assert_eq!(
            testing::generate_css("outline-[thin]"),
            r".outline-\[thin\] {
  outline-width: thin;
}"
        );
    }

    #[test]
    fn outline_color() {
        assert_eq!(
            testing::generate_css("outline-red-400"),
            ".outline-red-400 {
  outline-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            testing::generate_css("outline-[rgb(12,12,12)]"),
            r".outline-\[rgb\(12\,12\,12\)\] {
  outline-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn outline_offset() {
        assert_eq!(
            testing::generate_css("outline-offset-12"),
            ".outline-offset-12 {
  outline-offset: 12px;
}"
        );
        assert_eq!(
            testing::generate_css("outline-offset-[1rem]"),
            r".outline-offset-\[1rem\] {
  outline-offset: 1rem;
}"
        );
    }

    #[test]
    fn ring_width() {
        assert_eq!(
            testing::generate_css("ring"),
            ".ring {
  --en-ring-shadow: var(--en-ring-inset) 0 0 0 calc(3px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-ring-offset-shadow), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);
}"
        );
        assert_eq!(
            testing::generate_css("ring-11"),
            ".ring-11 {
  --en-ring-shadow: var(--en-ring-inset) 0 0 0 calc(11px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-ring-offset-shadow), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);
}"
        );
        assert_eq!(
            testing::generate_css("ring-inset"),
            ".ring-inset {
  --en-ring-inset: inset;
}"
        );
    }

    #[test]
    fn ring_color() {
        assert_eq!(
            testing::generate_css("ring-red-400"),
            ".ring-red-400 {
  --en-ring-opacity: 1;
  --en-ring-color: rgb(248 113 113 / var(--en-ring-opacity));
}"
        );
        assert_eq!(
            testing::generate_css("ring-[rgb(12,12,12)]"),
            r".ring-\[rgb\(12\,12\,12\)\] {
  --en-ring-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn ring_opacity() {
        assert_eq!(
            testing::generate_css("ring-red-400/42"),
            r".ring-red-400\/42 {
  --en-ring-color: rgb(248 113 113 / 0.42);
}"
        );

        assert_eq!(
            testing::generate_css("ring-opacity-42"),
            ".ring-opacity-42 {
  --en-ring-opacity: 0.42;
}"
        );
    }

    #[test]
    fn ring_offset_width() {
        assert_eq!(
            testing::generate_css("ring-offset-13"),
            ".ring-offset-13 {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-width: 13px;
}"
        );
        assert_eq!(
            testing::generate_css("ring-offset-[13em]"),
            r".ring-offset-\[13em\] {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-width: 13em;
}"
        );
    }

    #[test]
    fn ring_offset_color() {
        assert_eq!(
            testing::generate_css("ring-offset-red-400"),
            ".ring-offset-red-400 {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            testing::generate_css("ring-offset-[rgb(12,12,12)]"),
            r".ring-offset-\[rgb\(12\,12\,12\)\] {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-color: rgb(12,12,12);
}"
        );
    }
}
