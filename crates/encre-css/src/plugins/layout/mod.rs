//! Layout utilities
pub mod aspect_ratio;
pub mod box_decoration_break;
pub mod box_sizing;
pub mod break_after;
pub mod break_before;
pub mod break_inside;
pub mod clear;
pub mod columns;
pub mod container;
pub mod display;
pub mod floats;
pub mod isolation;
pub mod object_fit;
pub mod object_position;
pub mod overflow;
pub mod overscroll_behavior;
pub mod placement;
pub mod position;
pub mod visibility;
pub mod z_index;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn aspect_ratio() {
        assert_eq!(
            testing::generate_css("aspect-auto"),
            ".aspect-auto {
  aspect-ratio: auto;
}"
        );
        assert_eq!(
            testing::generate_css("aspect-video"),
            ".aspect-video {
  aspect-ratio: 16 / 9;
}"
        );
    }

    #[test]
    fn box_decoration_break() {
        assert_eq!(
            testing::generate_css("box-decoration-clone"),
            ".box-decoration-clone {
  box-decoration-break: clone;
}"
        );
    }

    #[test]
    fn box_sizing() {
        assert_eq!(
            testing::generate_css("box-content"),
            ".box-content {
  box-sizing: content-box;
}"
        );
    }

    #[test]
    fn break_after() {
        assert_eq!(
            testing::generate_css("break-after-all"),
            ".break-after-all {
  break-after: all;
}"
        );
        assert_eq!(
            testing::generate_css("break-after-avoid-page"),
            ".break-after-avoid-page {
  break-after: avoid-page;
}"
        );
        assert_eq!(
            testing::generate_css("break-after-column"),
            ".break-after-column {
  break-after: column;
}"
        );
    }

    #[test]
    fn break_before() {
        assert_eq!(
            testing::generate_css("break-before-all"),
            ".break-before-all {
  break-before: all;
}"
        );
        assert_eq!(
            testing::generate_css("break-before-avoid-page"),
            ".break-before-avoid-page {
  break-before: avoid-page;
}"
        );
        assert_eq!(
            testing::generate_css("break-before-column"),
            ".break-before-column {
  break-before: column;
}"
        );
    }

    #[test]
    fn break_inside() {
        assert_eq!(
            testing::generate_css("break-inside-auto"),
            ".break-inside-auto {
  break-inside: auto;
}"
        );
        assert_eq!(
            testing::generate_css("break-inside-avoid-page"),
            ".break-inside-avoid-page {
  break-inside: avoid-page;
}"
        );
    }

    #[test]
    fn clear() {
        assert_eq!(
            testing::generate_css("clear-both"),
            ".clear-both {
  clear: both;
}"
        );
    }

    #[test]
    fn columns() {
        assert_eq!(
            testing::generate_css("columns-md"),
            ".columns-md {
  columns: 28rem;
}"
        );
        assert_eq!(
            testing::generate_css("columns-4"),
            ".columns-4 {
  columns: 4;
}"
        );
    }

    #[test]
    fn container() {
        assert_eq!(
            testing::generate_css("container"),
            ".container {
  width: 100%;
}

@media (min-width: 640px) {
  .container {
    max-width: 640px;
  }
}

@media (min-width: 768px) {
  .container {
    max-width: 768px;
  }
}

@media (min-width: 1024px) {
  .container {
    max-width: 1024px;
  }
}

@media (min-width: 1280px) {
  .container {
    max-width: 1280px;
  }
}

@media (min-width: 1536px) {
  .container {
    max-width: 1536px;
  }
}"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            testing::generate_css("inline-flex"),
            ".inline-flex {
  display: inline-flex;
}"
        );
        assert_eq!(
            testing::generate_css("hidden"),
            ".hidden {
  display: none;
}"
        );
    }

    #[test]
    fn floats() {
        assert_eq!(
            testing::generate_css("float-right"),
            ".float-right {
  float: right;
}"
        );
    }

    #[test]
    fn isolation() {
        assert_eq!(
            testing::generate_css("isolate"),
            ".isolate {
  isolation: isolate;
}"
        );
        assert_eq!(
            testing::generate_css("isolation-auto"),
            ".isolation-auto {
  isolation: auto;
}"
        );
    }

    #[test]
    fn object_fit() {
        assert_eq!(
            testing::generate_css("object-contain"),
            ".object-contain {
  object-fit: contain;
}"
        );
        assert_eq!(
            testing::generate_css("object-scale-down"),
            ".object-scale-down {
  object-fit: scale-down;
}"
        );
    }

    #[test]
    fn object_position() {
        assert_eq!(
            testing::generate_css("object-bottom"),
            ".object-bottom {
  object-position: bottom;
}"
        );
        assert_eq!(
            testing::generate_css("object-left-top"),
            ".object-left-top {
  object-position: left top;
}"
        );
        assert_eq!(
            testing::generate_css("object-[center_bottom]"),
            r".object-\[center_bottom\] {
  object-position: center bottom;
}"
        );
    }

    #[test]
    fn overflow() {
        assert_eq!(
            testing::generate_css("overflow-hidden"),
            ".overflow-hidden {
  overflow: hidden;
}"
        );
        assert_eq!(
            testing::generate_css("overflow-y-scroll"),
            ".overflow-y-scroll {
  overflow-y: scroll;
}"
        );
    }

    #[test]
    fn overscroll_behavior() {
        assert_eq!(
            testing::generate_css("overscroll-auto"),
            ".overscroll-auto {
  overscroll-behavior: auto;
}"
        );
        assert_eq!(
            testing::generate_css("overscroll-y-none"),
            ".overscroll-y-none {
  overscroll-behavior-y: none;
}"
        );
    }

    #[test]
    fn placement() {
        assert_eq!(
            testing::generate_css("top-12"),
            ".top-12 {
  top: 3rem;
}"
        );
        assert_eq!(
            testing::generate_css("top-[1px]"),
            r".top-\[1px\] {
  top: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("left-auto"),
            ".left-auto {
  left: auto;
}"
        );
        assert_eq!(
            testing::generate_css("right-full"),
            ".right-full {
  right: 100%;
}"
        );
        assert_eq!(
            testing::generate_css("inset-2"),
            ".inset-2 {
  top: 0.5rem;
  right: 0.5rem;
  bottom: 0.5rem;
  left: 0.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("-left-20"),
            ".-left-20 {
  left: -5rem;
}"
        );
        assert_eq!(
            testing::generate_css("right-[20%]"),
            r".right-\[20\%\] {
  right: 20%;
}"
        );
        assert_eq!(
            testing::generate_css("inset-[20px]"),
            r".inset-\[20px\] {
  top: 20px;
  right: 20px;
  bottom: 20px;
  left: 20px;
}"
        );
        assert_eq!(
            testing::generate_css("inset-y-[10em]"),
            r".inset-y-\[10em\] {
  top: 10em;
  bottom: 10em;
}"
        );
    }

    #[test]
    fn position() {
        assert_eq!(
            testing::generate_css("relative"),
            ".relative {
  position: relative;
}"
        );
    }

    #[test]
    fn visibility() {
        assert_eq!(
            testing::generate_css("visible"),
            ".visible {
  visibility: visible;
}"
        );
        assert_eq!(
            testing::generate_css("invisible"),
            ".invisible {
  visibility: hidden;
}"
        );
    }

    #[test]
    fn z_index() {
        assert_eq!(
            testing::generate_css("z-22"),
            ".z-22 {
  z-index: 22;
}"
        );
    }
}
