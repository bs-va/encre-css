//! Table utilities
pub mod border_collapse;
pub mod border_spacing;
pub mod table_layout;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn border_collapse() {
        assert_eq!(
            testing::generate_css("border-separate"),
            ".border-separate {
  border-collapse: separate;
}"
        );
    }

    #[test]
    fn border_spacing() {
        assert_eq!(
            testing::generate_css("border-spacing-32"),
            ".border-spacing-32 {
  --en-border-spacing-x: 8rem;
  --en-border-spacing-y: 8rem;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            testing::generate_css("border-spacing-x-px"),
            ".border-spacing-x-px {
  --en-border-spacing-x: 1px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            testing::generate_css("border-spacing-y-0"),
            ".border-spacing-y-0 {
  --en-border-spacing-y: 0px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            testing::generate_css("border-spacing-[22px]"),
            r".border-spacing-\[22px\] {
  --en-border-spacing-x: 22px;
  --en-border-spacing-y: 22px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            testing::generate_css("border-spacing-x-[22px]"),
            r".border-spacing-x-\[22px\] {
  --en-border-spacing-x: 22px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            testing::generate_css("border-spacing-y-[12px]"),
            r".border-spacing-y-\[12px\] {
  --en-border-spacing-y: 12px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
    }

    #[test]
    fn table_layout() {
        assert_eq!(
            testing::generate_css("table-fixed"),
            ".table-fixed {
  table-layout: fixed;
}"
        );
    }
}
