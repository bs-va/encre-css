//! Spacing utilities
pub mod margin;
pub mod padding;
pub mod space_between;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn margin() {
        assert_eq!(
            testing::generate_css("m-2"),
            ".m-2 {
  margin: 0.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("mt-auto"),
            ".mt-auto {
  margin-top: auto;
}"
        );
        assert_eq!(
            testing::generate_css("mx-[2px]"),
            r".mx-\[2px\] {
  margin-left: 2px;
  margin-right: 2px;
}"
        );
        assert_eq!(
            testing::generate_css("my-[20%]"),
            r".my-\[20\%\] {
  margin-top: 20%;
  margin-bottom: 20%;
}"
        );
    }

    #[test]
    fn padding() {
        assert_eq!(
            testing::generate_css("p-2"),
            ".p-2 {
  padding: 0.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("pt-auto"),
            ".pt-auto {
  padding-top: auto;
}"
        );
        assert_eq!(
            testing::generate_css("px-[2px]"),
            r".px-\[2px\] {
  padding-left: 2px;
  padding-right: 2px;
}"
        );
        assert_eq!(
            testing::generate_css("py-[20%]"),
            r".py-\[20\%\] {
  padding-top: 20%;
  padding-bottom: 20%;
}"
        );
    }

    #[test]
    fn space_between() {
        assert_eq!(
            testing::generate_css("space-x-42"),
            ".space-x-42 > :not([hidden]) ~ :not([hidden]) {
  --en-space-x-reverse: 0;
  margin-right: calc(10.5rem * var(--en-space-x-reverse));
  margin-left: calc(10.5rem * calc(1 - var(--en-space-x-reverse)));
}"
        );
        assert_eq!(
            testing::generate_css("space-x-[42px]"),
            r".space-x-\[42px\] > :not([hidden]) ~ :not([hidden]) {
  --en-space-x-reverse: 0;
  margin-right: calc(42px * var(--en-space-x-reverse));
  margin-left: calc(42px * calc(1 - var(--en-space-x-reverse)));
}"
        );
        assert_eq!(
            testing::generate_css("space-x-reverse"),
            ".space-x-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-space-x-reverse: 1;
}"
        );
        assert_eq!(
            testing::generate_css("space-y-reverse"),
            ".space-y-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-space-y-reverse: 1;
}"
        );
        assert_eq!(
            testing::generate_css("space-y-[12%]"),
            r".space-y-\[12\%\] > :not([hidden]) ~ :not([hidden]) {
  --en-space-y-reverse: 0;
  margin-top: calc(12% * calc(1 - var(--en-space-y-reverse)));
  margin-bottom: calc(12% * var(--en-space-y-reverse));
}"
        );
    }
}
