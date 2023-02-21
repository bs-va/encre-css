//! SVG utilities
pub mod fill;
pub mod stroke;
pub mod stroke_width;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn fill() {
        assert_eq!(
            testing::generate_css("fill-red-400"),
            ".fill-red-400 {
  fill: rgb(248 113 113);
}"
        );
        assert_eq!(
            testing::generate_css("fill-[rgb(12,12,12)]"),
            r".fill-\[rgb\(12\,12\,12\)\] {
  fill: rgb(12,12,12);
}"
        );
        assert_eq!(
            testing::generate_css("fill-[purple]"),
            r".fill-\[purple\] {
  fill: purple;
}"
        );
    }

    #[test]
    fn stroke() {
        assert_eq!(
            testing::generate_css("stroke-red-400"),
            ".stroke-red-400 {
  stroke: rgb(248 113 113);
}"
        );
        assert_eq!(
            testing::generate_css("stroke-[rgb(12,12,12)]"),
            r".stroke-\[rgb\(12\,12\,12\)\] {
  stroke: rgb(12,12,12);
}"
        );
        assert_eq!(
            testing::generate_css("stroke-[purple]"),
            r".stroke-\[purple\] {
  stroke: purple;
}"
        );
    }

    #[test]
    fn stroke_width() {
        assert_eq!(
            testing::generate_css("stroke-2"),
            ".stroke-2 {
  stroke-width: 2px;
}"
        );
        assert_eq!(
            testing::generate_css("stroke-[0.25rem]"),
            r".stroke-\[0\.25rem\] {
  stroke-width: 0.25rem;
}"
        );
        assert_eq!(
            testing::generate_css("stroke-[5%]"),
            r".stroke-\[5\%\] {
  stroke-width: 5%;
}"
        );
    }
}
