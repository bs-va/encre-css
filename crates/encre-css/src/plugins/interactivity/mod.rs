//! Interactivity utilities
pub mod accent_color;
pub mod appearance;
pub mod caret_color;
pub mod cursor;
pub mod pointer_events;
pub mod resize;
pub mod scroll_behavior;
pub mod scroll_margin;
pub mod scroll_padding;
pub mod scroll_snap_align;
pub mod scroll_snap_stop;
pub mod scroll_snap_type;
pub mod touch_action;
pub mod user_select;
pub mod will_change;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn accent_color() {
        assert_eq!(
            generate(["accent-red-400"], &base_config()),
            ".accent-red-400 {
  accent-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            generate(["accent-[rgb(12,12,12)]"], &base_config()),
            r".accent-\[rgb\(12\,12\,12\)\] {
  accent-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn appearance() {
        assert_eq!(
            generate(["appearance-none"], &base_config()),
            ".appearance-none {
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
}"
        );
    }

    #[test]
    fn caret_color() {
        assert_eq!(
            generate(["caret-red-400"], &base_config()),
            ".caret-red-400 {
  caret-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            generate(["caret-[rgb(12,12,12)]"], &base_config()),
            r".caret-\[rgb\(12\,12\,12\)\] {
  caret-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn cursor() {
        assert_eq!(
            generate(["cursor-auto"], &base_config()),
            ".cursor-auto {
  cursor: auto;
}"
        );
        assert_eq!(
            generate(["cursor-zoom-out"], &base_config()),
            ".cursor-zoom-out {
  cursor: zoom-out;
}"
        );
        assert_eq!(
            generate(["cursor-[url(spinner.svg)_4_5,_progress]"], &base_config()),
            r".cursor-\[url\(spinner\.svg\)_4_5\,_progress\] {
  cursor: url(spinner.svg) 4 5, progress;
}"
        );
    }

    #[test]
    fn pointer_events() {
        assert_eq!(
            generate(["pointer-events-none"], &base_config()),
            ".pointer-events-none {
  pointer-events: none;
}"
        );
    }

    #[test]
    fn resize() {
        assert_eq!(
            generate(["resize"], &base_config()),
            ".resize {
  resize: both;
}"
        );
        assert_eq!(
            generate(["resize-y"], &base_config()),
            ".resize-y {
  resize: vertical;
}"
        );
    }

    #[test]
    fn scroll_behavior() {
        assert_eq!(
            generate(["scroll-smooth"], &base_config()),
            ".scroll-smooth {
  scroll-behavior: smooth;
}"
        );
    }

    #[test]
    fn scroll_snap_align() {
        assert_eq!(
            generate(["snap-center"], &base_config()),
            ".snap-center {
  scroll-snap-align: center;
}"
        );
        assert_eq!(
            generate(["snap-align-none"], &base_config()),
            ".snap-align-none {
  scroll-snap-align: none;
}"
        );
    }

    #[test]
    fn scroll_snap_stop() {
        assert_eq!(
            generate(["snap-always"], &base_config()),
            ".snap-always {
  scroll-snap-stop: always;
}"
        );
    }

    #[test]
    fn scroll_snap_type() {
        assert_eq!(
            generate(["snap-none"], &base_config()),
            ".snap-none {
  -ms-scroll-snap-type: none;
  scroll-snap-type: none;
}"
        );
        assert_eq!(
            generate(["snap-both"], &base_config()),
            ".snap-both {
  -ms-scroll-snap-type: both var(--en-scroll-snap-strictness);
  scroll-snap-type: both var(--en-scroll-snap-strictness);
}"
        );
        assert_eq!(
            generate(["snap-mandatory"], &base_config()),
            ".snap-mandatory {
  --en-scroll-snap-strictness: mandatory;
}"
        );
    }

    #[test]
    fn touch_action() {
        assert_eq!(
            generate(["touch-pan-right"], &base_config()),
            ".touch-pan-right {
  touch-action: pan-right;
}"
        );
        assert_eq!(
            generate(["touch-none"], &base_config()),
            ".touch-none {
  touch-action: none;
}"
        );
    }

    #[test]
    fn user_select() {
        assert_eq!(
            generate(["select-all"], &base_config()),
            ".select-all {
  user-select: all;
}"
        );
    }

    #[test]
    fn will_change() {
        assert_eq!(
            generate(["will-change-contents"], &base_config()),
            ".will-change-contents {
  will-change: contents;
}"
        );
    }
}
