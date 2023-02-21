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
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn accent_color() {
        assert_eq!(
            testing::generate_css("accent-red-400"),
            ".accent-red-400 {
  accent-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            testing::generate_css("accent-[rgb(12,12,12)]"),
            r".accent-\[rgb\(12\,12\,12\)\] {
  accent-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn appearance() {
        assert_eq!(
            testing::generate_css("appearance-none"),
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
            testing::generate_css("caret-red-400"),
            ".caret-red-400 {
  caret-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            testing::generate_css("caret-[rgb(12,12,12)]"),
            r".caret-\[rgb\(12\,12\,12\)\] {
  caret-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn cursor() {
        assert_eq!(
            testing::generate_css("cursor-auto"),
            ".cursor-auto {
  cursor: auto;
}"
        );
        assert_eq!(
            testing::generate_css("cursor-zoom-out"),
            ".cursor-zoom-out {
  cursor: zoom-out;
}"
        );
        assert_eq!(
            testing::generate_css("cursor-[url(spinner.svg)_4_5,_progress]"),
            r".cursor-\[url\(spinner\.svg\)_4_5\,_progress\] {
  cursor: url(spinner.svg) 4 5, progress;
}"
        );
    }

    #[test]
    fn pointer_events() {
        assert_eq!(
            testing::generate_css("pointer-events-none"),
            ".pointer-events-none {
  pointer-events: none;
}"
        );
    }

    #[test]
    fn resize() {
        assert_eq!(
            testing::generate_css("resize"),
            ".resize {
  resize: both;
}"
        );
        assert_eq!(
            testing::generate_css("resize-y"),
            ".resize-y {
  resize: vertical;
}"
        );
    }

    #[test]
    fn scroll_behavior() {
        assert_eq!(
            testing::generate_css("scroll-smooth"),
            ".scroll-smooth {
  scroll-behavior: smooth;
}"
        );
    }

    #[test]
    fn scroll_snap_align() {
        assert_eq!(
            testing::generate_css("snap-center"),
            ".snap-center {
  scroll-snap-align: center;
}"
        );
        assert_eq!(
            testing::generate_css("snap-align-none"),
            ".snap-align-none {
  scroll-snap-align: none;
}"
        );
    }

    #[test]
    fn scroll_snap_stop() {
        assert_eq!(
            testing::generate_css("snap-always"),
            ".snap-always {
  scroll-snap-stop: always;
}"
        );
    }

    #[test]
    fn scroll_snap_type() {
        assert_eq!(
            testing::generate_css("snap-none"),
            ".snap-none {
  -ms-scroll-snap-type: none;
  scroll-snap-type: none;
}"
        );
        assert_eq!(
            testing::generate_css("snap-both"),
            ".snap-both {
  -ms-scroll-snap-type: both var(--en-scroll-snap-strictness);
  scroll-snap-type: both var(--en-scroll-snap-strictness);
}"
        );
        assert_eq!(
            testing::generate_css("snap-mandatory"),
            ".snap-mandatory {
  --en-scroll-snap-strictness: mandatory;
}"
        );
    }

    #[test]
    fn touch_action() {
        assert_eq!(
            testing::generate_css("touch-pan-right"),
            ".touch-pan-right {
  touch-action: pan-right;
}"
        );
        assert_eq!(
            testing::generate_css("touch-none"),
            ".touch-none {
  touch-action: none;
}"
        );
    }

    #[test]
    fn user_select() {
        assert_eq!(
            testing::generate_css("select-all"),
            ".select-all {
  user-select: all;
}"
        );
    }

    #[test]
    fn will_change() {
        assert_eq!(
            testing::generate_css("will-change-contents"),
            ".will-change-contents {
  will-change: contents;
}"
        );
    }
}
