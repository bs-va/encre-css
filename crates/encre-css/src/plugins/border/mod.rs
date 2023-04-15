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
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn border_color() {
        assert_eq!(
            generate(["border-red-400"], &base_config()),
            ".border-red-400 {
  --en-border-opacity: 1;
  border-color: rgb(248 113 113 / var(--en-border-opacity));
}"
        );
        assert_eq!(
            generate(["border-[rgb(12,12,12)]"], &base_config()),
            r".border-\[rgb\(12\,12\,12\)\] {
  border-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["border-x-[#ff0]"], &base_config()),
            r".border-x-\[\#ff0\] {
  border-left-color: #ff0;
  border-right-color: #ff0;
}"
        );
        assert_eq!(
            generate(["border-s-blue-400"], &base_config()),
            ".border-s-blue-400 {
  --en-border-opacity: 1;
  border-inline-start-color: rgb(96 165 250 / var(--en-border-opacity));
}"
        );
        assert_eq!(
            generate(["border-e-blue-400/10"], &base_config()),
            r".border-e-blue-400\/10 {
  border-inline-end-color: rgb(96 165 250 / 0.1);
}"
        );
    }

    #[test]
    fn border_opacity() {
        assert_eq!(
            generate(["border-red-400/12"], &base_config()),
            r".border-red-400\/12 {
  border-color: rgb(248 113 113 / 0.12);
}"
        );
        assert_eq!(
            generate(["border-opacity-12"], &base_config()),
            ".border-opacity-12 {
  --en-border-opacity: 0.12;
}"
        );
    }

    #[test]
    fn border_style() {
        assert_eq!(
            generate(["border-dashed"], &base_config()),
            ".border-dashed {
  border-style: dashed;
}"
        );

        assert_eq!(
            generate(["border-[solid_none_solid_none]"], &base_config()),
            r".border-\[solid_none_solid_none\] {
  border-style: solid none solid none;
}"
        );
    }

    #[test]
    fn border_width() {
        assert_eq!(
            generate(["border"], &base_config()),
            ".border {
  border-width: 1px;
}"
        );
        assert_eq!(
            generate(["border-x"], &base_config()),
            ".border-x {
  border-left-width: 1px;
  border-right-width: 1px;
}"
        );
        assert_eq!(
            generate(["border-s"], &base_config()),
            ".border-s {
  border-inline-start-width: 1px;
}"
        );
        assert_eq!(
            generate(["border-t"], &base_config()),
            ".border-t {
  border-top-width: 1px;
}"
        );

        assert_eq!(
            generate(["border-2"], &base_config()),
            ".border-2 {
  border-width: 2px;
}"
        );
        assert_eq!(
            generate(["border-x-24"], &base_config()),
            ".border-x-24 {
  border-left-width: 24px;
  border-right-width: 24px;
}"
        );
        assert_eq!(
            generate(["border-e-2"], &base_config()),
            ".border-e-2 {
  border-inline-end-width: 2px;
}"
        );
        assert_eq!(
            generate(["border-t-42"], &base_config()),
            ".border-t-42 {
  border-top-width: 42px;
}"
        );
        assert_eq!(
            generate(["border-b-[3rem]"], &base_config()),
            r".border-b-\[3rem\] {
  border-bottom-width: 3rem;
}"
        );
        assert_eq!(
            generate(["border-y-[thick]"], &base_config()),
            r".border-y-\[thick\] {
  border-top-width: thick;
  border-bottom-width: thick;
}"
        );
    }

    #[test]
    fn border_radius() {
        assert_eq!(
            generate(["rounded"], &base_config()),
            ".rounded {
  border-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-[2px]"], &base_config()),
            r".rounded-\[2px\] {
  border-radius: 2px;
}"
        );
        assert_eq!(
            generate(["rounded-bl"], &base_config()),
            ".rounded-bl {
  border-bottom-left-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-t"], &base_config()),
            ".rounded-t {
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-s"], &base_config()),
            ".rounded-s {
  border-start-start-radius: 0.25rem;
  border-end-start-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-xl"], &base_config()),
            ".rounded-xl {
  border-radius: 0.75rem;
}"
        );

        assert_eq!(
            generate(["rounded-sm"], &base_config()),
            ".rounded-sm {
  border-radius: 0.125rem;
}"
        );
        assert_eq!(
            generate(["rounded-bl-full"], &base_config()),
            ".rounded-bl-full {
  border-bottom-left-radius: 9999px;
}"
        );
        assert_eq!(
            generate(["rounded-ee-md"], &base_config()),
            ".rounded-ee-md {
  border-end-end-radius: 0.375rem;
}"
        );
        assert_eq!(
            generate(["rounded-t-3xl"], &base_config()),
            ".rounded-t-3xl {
  border-top-left-radius: 1.5rem;
  border-top-right-radius: 1.5rem;
}"
        );
        assert_eq!(
            generate(["rounded-b-[3em]"], &base_config()),
            r".rounded-b-\[3em\] {
  border-bottom-left-radius: 3em;
  border-bottom-right-radius: 3em;
}"
        );
        assert_eq!(
            generate(["rounded-tr-[20%]"], &base_config()),
            r".rounded-tr-\[20\%\] {
  border-top-right-radius: 20%;
}"
        );
    }

    #[test]
    fn divide_style() {
        assert_eq!(
            generate(["divide-double"], &base_config()),
            ".divide-double > :not([hidden]) ~ :not([hidden]) {
  border-style: double;
}"
        );
    }

    #[test]
    fn divide_color() {
        assert_eq!(
            generate(["divide-red-400"], &base_config()),
            ".divide-red-400 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-opacity: 1;
  border-color: rgb(248 113 113 / var(--en-divide-opacity));
}"
        );
        assert_eq!(
            generate(["divide-[rgb(12,12,12)]"], &base_config()),
            r".divide-\[rgb\(12\,12\,12\)\] > :not([hidden]) ~ :not([hidden]) {
  border-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn divide_opacity() {
        assert_eq!(
            generate(["divide-red-400/42"], &base_config()),
            r".divide-red-400\/42 > :not([hidden]) ~ :not([hidden]) {
  border-color: rgb(248 113 113 / 0.42);
}"
        );

        assert_eq!(
            generate(["divide-opacity-42"], &base_config()),
            ".divide-opacity-42 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-opacity: 0.42;
}"
        );
    }

    #[test]
    fn divide_width() {
        assert_eq!(
            generate(["divide-x"], &base_config()),
            ".divide-x > :not([hidden]) ~ :not([hidden]) {
  --en-divide-x-reverse: 0;
  border-right-width: calc(1px * var(--en-divide-x-reverse));
  border-left-width: calc(1px * calc(1 - var(--en-divide-x-reverse)));
}"
        );
        assert_eq!(
            generate(["divide-y-2"], &base_config()),
            ".divide-y-2 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 0;
  border-top-width: calc(2px * calc(1 - var(--en-divide-y-reverse)));
  border-bottom-width: calc(2px * var(--en-divide-y-reverse));
}"
        );
        assert_eq!(
            generate(["divide-x-reverse"], &base_config()),
            ".divide-x-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-divide-x-reverse: 1;
}"
        );
        assert_eq!(
            generate(["divide-y-[0.1rem]"], &base_config()),
            r".divide-y-\[0\.1rem\] > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 0;
  border-top-width: calc(0.1rem * calc(1 - var(--en-divide-y-reverse)));
  border-bottom-width: calc(0.1rem * var(--en-divide-y-reverse));
}"
        );
        assert_eq!(
            generate(["divide-y-reverse"], &base_config()),
            ".divide-y-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 1;
}"
        );
    }

    #[test]
    fn outline_style() {
        assert_eq!(
            generate(["outline-none"], &base_config()),
            ".outline-none {
  outline: 2px solid transparent;
  outline-offset: 2px;
}"
        );
        assert_eq!(
            generate(["outline-dashed"], &base_config()),
            ".outline-dashed {
  outline-style: dashed;
}"
        );
    }

    #[test]
    fn outline_width() {
        assert_eq!(
            generate(["outline-33"], &base_config()),
            ".outline-33 {
  outline-width: 33px;
}"
        );
        assert_eq!(
            generate(["outline-[2rem]"], &base_config()),
            r".outline-\[2rem\] {
  outline-width: 2rem;
}"
        );
        assert_eq!(
            generate(["outline-[thin]"], &base_config()),
            r".outline-\[thin\] {
  outline-width: thin;
}"
        );
    }

    #[test]
    fn outline_color() {
        assert_eq!(
            generate(["outline-red-400"], &base_config()),
            ".outline-red-400 {
  outline-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            generate(["outline-[rgb(12,12,12)]"], &base_config()),
            r".outline-\[rgb\(12\,12\,12\)\] {
  outline-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn outline_offset() {
        assert_eq!(
            generate(["outline-offset-12"], &base_config()),
            ".outline-offset-12 {
  outline-offset: 12px;
}"
        );
        assert_eq!(
            generate(["outline-offset-[1rem]"], &base_config()),
            r".outline-offset-\[1rem\] {
  outline-offset: 1rem;
}"
        );
    }

    #[test]
    fn ring_width() {
        assert_eq!(
            generate(["ring"], &base_config()),
            ".ring {
  --en-ring-shadow: var(--en-ring-inset) 0 0 0 calc(3px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-ring-offset-shadow), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);
}"
        );
        assert_eq!(
            generate(["ring-11"], &base_config()),
            ".ring-11 {
  --en-ring-shadow: var(--en-ring-inset) 0 0 0 calc(11px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-ring-offset-shadow), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);
}"
        );
        assert_eq!(
            generate(["ring-inset"], &base_config()),
            ".ring-inset {
  --en-ring-inset: inset;
}"
        );
    }

    #[test]
    fn ring_color() {
        assert_eq!(
            generate(["ring-red-400"], &base_config()),
            ".ring-red-400 {
  --en-ring-opacity: 1;
  --en-ring-color: rgb(248 113 113 / var(--en-ring-opacity));
}"
        );
        assert_eq!(
            generate(["ring-[rgb(12,12,12)]"], &base_config()),
            r".ring-\[rgb\(12\,12\,12\)\] {
  --en-ring-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn ring_opacity() {
        assert_eq!(
            generate(["ring-red-400/42"], &base_config()),
            r".ring-red-400\/42 {
  --en-ring-color: rgb(248 113 113 / 0.42);
}"
        );

        assert_eq!(
            generate(["ring-opacity-42"], &base_config()),
            ".ring-opacity-42 {
  --en-ring-opacity: 0.42;
}"
        );
    }

    #[test]
    fn ring_offset_width() {
        assert_eq!(
            generate(["ring-offset-13"], &base_config()),
            ".ring-offset-13 {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-width: 13px;
}"
        );
        assert_eq!(
            generate(["ring-offset-[13em]"], &base_config()),
            r".ring-offset-\[13em\] {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-width: 13em;
}"
        );
    }

    #[test]
    fn ring_offset_color() {
        assert_eq!(
            generate(["ring-offset-red-400"], &base_config()),
            ".ring-offset-red-400 {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            generate(["ring-offset-[rgb(12,12,12)]"], &base_config()),
            r".ring-offset-\[rgb\(12\,12\,12\)\] {
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
  --en-ring-offset-color: rgb(12,12,12);
}"
        );
    }
}
