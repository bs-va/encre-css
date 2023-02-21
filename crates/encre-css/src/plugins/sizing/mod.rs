//! Sizing utilities
pub mod height;
pub mod max_height;
pub mod max_width;
pub mod min_height;
pub mod min_width;
pub mod width;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn height() {
        assert_eq!(
            testing::generate_css("h-px"),
            ".h-px {
  height: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("h-2.5"),
            r".h-2\.5 {
  height: 0.625rem;
}"
        );
        assert_eq!(
            testing::generate_css("h-60"),
            ".h-60 {
  height: 15rem;
}"
        );
        assert_eq!(
            testing::generate_css("-h-60"),
            ".-h-60 {
  height: -15rem;
}"
        );
        assert_eq!(
            testing::generate_css("h-auto"),
            ".h-auto {
  height: auto;
}"
        );
        assert_eq!(
            testing::generate_css("h-1/3"),
            r".h-1\/3 {
  height: 33.333333%;
}"
        );
        assert_eq!(
            testing::generate_css("-h-2/3"),
            r".-h-2\/3 {
  height: -66.666667%;
}"
        );
        assert_eq!(
            testing::generate_css("h-full"),
            ".h-full {
  height: 100%;
}"
        );
        assert_eq!(
            testing::generate_css("h-screen"),
            ".h-screen {
  height: 100vh;
}"
        );
        assert_eq!(
            testing::generate_css("h-fit"),
            ".h-fit {
  height: fit-content;
}"
        );
        assert_eq!(
            testing::generate_css("h-[32.555rem]"),
            r".h-\[32\.555rem\] {
  height: 32.555rem;
}"
        );
        assert_eq!(
            testing::generate_css("h-[10%]"),
            r".h-\[10\%\] {
  height: 10%;
}"
        );
    }

    #[test]
    fn max_height() {
        assert_eq!(
            testing::generate_css("max-h-px"),
            ".max-h-px {
  max-height: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-2.5"),
            r".max-h-2\.5 {
  max-height: 0.625rem;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-60"),
            ".max-h-60 {
  max-height: 15rem;
}"
        );
        assert_eq!(
            testing::generate_css("-max-h-60"),
            ".-max-h-60 {
  max-height: -15rem;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-auto"),
            ".max-h-auto {
  max-height: auto;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-none"),
            ".max-h-none {
  max-height: none;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-1/3"),
            r".max-h-1\/3 {
  max-height: 33.333333%;
}"
        );
        assert_eq!(
            testing::generate_css("-max-h-2/3"),
            r".-max-h-2\/3 {
  max-height: -66.666667%;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-full"),
            ".max-h-full {
  max-height: 100%;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-screen"),
            ".max-h-screen {
  max-height: 100vh;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-fit"),
            ".max-h-fit {
  max-height: fit-content;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-[32.555rem]"),
            r".max-h-\[32\.555rem\] {
  max-height: 32.555rem;
}"
        );
        assert_eq!(
            testing::generate_css("max-h-[10%]"),
            r".max-h-\[10\%\] {
  max-height: 10%;
}"
        );
    }

    #[test]
    fn max_width() {
        assert_eq!(
            testing::generate_css("max-w-6xl"),
            ".max-w-6xl {
  max-width: 72rem;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-prose"),
            ".max-w-prose {
  max-width: 65ch;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-screen-xl"),
            ".max-w-screen-xl {
  max-width: 1280px;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-px"),
            ".max-w-px {
  max-width: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-2.5"),
            r".max-w-2\.5 {
  max-width: 0.625rem;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-60"),
            ".max-w-60 {
  max-width: 15rem;
}"
        );
        assert_eq!(
            testing::generate_css("-max-w-60"),
            ".-max-w-60 {
  max-width: -15rem;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-none"),
            ".max-w-none {
  max-width: none;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-1/3"),
            r".max-w-1\/3 {
  max-width: 33.333333%;
}"
        );
        assert_eq!(
            testing::generate_css("-max-w-2/3"),
            r".-max-w-2\/3 {
  max-width: -66.666667%;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-full"),
            ".max-w-full {
  max-width: 100%;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-screen"),
            ".max-w-screen {
  max-width: 100vw;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-fit"),
            ".max-w-fit {
  max-width: fit-content;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-[32.555rem]"),
            r".max-w-\[32\.555rem\] {
  max-width: 32.555rem;
}"
        );
        assert_eq!(
            testing::generate_css("max-w-[10%]"),
            r".max-w-\[10\%\] {
  max-width: 10%;
}"
        );
    }

    #[test]
    fn min_height() {
        assert_eq!(
            testing::generate_css("min-h-px"),
            ".min-h-px {
  min-height: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-2.5"),
            r".min-h-2\.5 {
  min-height: 0.625rem;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-60"),
            ".min-h-60 {
  min-height: 15rem;
}"
        );
        assert_eq!(
            testing::generate_css("-min-h-60"),
            ".-min-h-60 {
  min-height: -15rem;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-auto"),
            ".min-h-auto {
  min-height: auto;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-1/3"),
            r".min-h-1\/3 {
  min-height: 33.333333%;
}"
        );
        assert_eq!(
            testing::generate_css("-min-h-2/3"),
            r".-min-h-2\/3 {
  min-height: -66.666667%;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-full"),
            ".min-h-full {
  min-height: 100%;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-screen"),
            ".min-h-screen {
  min-height: 100vh;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-fit"),
            ".min-h-fit {
  min-height: fit-content;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-[32.555rem]"),
            r".min-h-\[32\.555rem\] {
  min-height: 32.555rem;
}"
        );
        assert_eq!(
            testing::generate_css("min-h-[10%]"),
            r".min-h-\[10\%\] {
  min-height: 10%;
}"
        );
    }

    #[test]
    fn min_width() {
        assert_eq!(
            testing::generate_css("min-w-px"),
            ".min-w-px {
  min-width: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-2.5"),
            r".min-w-2\.5 {
  min-width: 0.625rem;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-60"),
            ".min-w-60 {
  min-width: 15rem;
}"
        );
        assert_eq!(
            testing::generate_css("-min-w-60"),
            ".-min-w-60 {
  min-width: -15rem;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-auto"),
            ".min-w-auto {
  min-width: auto;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-1/3"),
            r".min-w-1\/3 {
  min-width: 33.333333%;
}"
        );
        assert_eq!(
            testing::generate_css("-min-w-2/3"),
            r".-min-w-2\/3 {
  min-width: -66.666667%;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-full"),
            ".min-w-full {
  min-width: 100%;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-screen"),
            ".min-w-screen {
  min-width: 100vw;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-fit"),
            ".min-w-fit {
  min-width: fit-content;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-[32.555rem]"),
            r".min-w-\[32\.555rem\] {
  min-width: 32.555rem;
}"
        );
        assert_eq!(
            testing::generate_css("min-w-[10%]"),
            r".min-w-\[10\%\] {
  min-width: 10%;
}"
        );
    }

    #[test]
    fn width() {
        assert_eq!(
            testing::generate_css("w-px"),
            ".w-px {
  width: 1px;
}"
        );
        assert_eq!(
            testing::generate_css("w-2.5"),
            r".w-2\.5 {
  width: 0.625rem;
}"
        );
        assert_eq!(
            testing::generate_css("w-60"),
            ".w-60 {
  width: 15rem;
}"
        );
        assert_eq!(
            testing::generate_css("-w-60"),
            ".-w-60 {
  width: -15rem;
}"
        );
        assert_eq!(
            testing::generate_css("w-auto"),
            ".w-auto {
  width: auto;
}"
        );
        assert_eq!(
            testing::generate_css("w-1/3"),
            r".w-1\/3 {
  width: 33.333333%;
}"
        );
        assert_eq!(
            testing::generate_css("-w-2/3"),
            r".-w-2\/3 {
  width: -66.666667%;
}"
        );
        assert_eq!(
            testing::generate_css("w-full"),
            ".w-full {
  width: 100%;
}"
        );
        assert_eq!(
            testing::generate_css("w-screen"),
            ".w-screen {
  width: 100vw;
}"
        );
        assert_eq!(
            testing::generate_css("w-fit"),
            ".w-fit {
  width: fit-content;
}"
        );
        assert_eq!(
            testing::generate_css("w-[32.555rem]"),
            r".w-\[32\.555rem\] {
  width: 32.555rem;
}"
        );
        assert_eq!(
            testing::generate_css("w-[10%]"),
            r".w-\[10\%\] {
  width: 10%;
}"
        );
    }
}
