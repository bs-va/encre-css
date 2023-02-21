//! Grid utilities
pub mod gap;
pub mod grid_auto_columns;
pub mod grid_auto_flow;
pub mod grid_auto_rows;
pub mod grid_column;
pub mod grid_row;
pub mod grid_template_columns;
pub mod grid_template_rows;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn gap() {
        assert_eq!(
            testing::generate_css("gap-4"),
            ".gap-4 {
  gap: 1rem;
}"
        );
        assert_eq!(
            testing::generate_css("-gap-4"),
            ".-gap-4 {
  gap: -1rem;
}"
        );
        assert_eq!(
            testing::generate_css("gap-[40px]"),
            r".gap-\[40px\] {
  gap: 40px;
}"
        );
    }

    #[test]
    fn grid_auto_columns() {
        assert_eq!(
            testing::generate_css("auto-cols-auto"),
            ".auto-cols-auto {
  grid-auto-columns: auto;
}"
        );
        assert_eq!(
            testing::generate_css("auto-cols-fr"),
            ".auto-cols-fr {
  grid-auto-columns: minmax(0, 1fr);
}"
        );
        assert_eq!(
            testing::generate_css(
                "auto-cols-[100px_minmax(100px,_auto)_10%_0.5fr_fit-content(400px)]"
            ),
            r".auto-cols-\[100px_minmax\(100px\,_auto\)_10\%_0\.5fr_fit-content\(400px\)\] {
  grid-auto-columns: 100px minmax(100px, auto) 10% 0.5fr fit-content(400px);
}"
        );
    }

    #[test]
    fn grid_auto_flow() {
        assert_eq!(
            testing::generate_css("grid-flow-row"),
            ".grid-flow-row {
  grid-auto-flow: row;
}"
        );
        assert_eq!(
            testing::generate_css("grid-flow-col-dense"),
            ".grid-flow-col-dense {
  grid-auto-flow: column dense;
}"
        );
    }

    #[test]
    fn grid_auto_rows() {
        assert_eq!(
            testing::generate_css("auto-rows-auto"),
            ".auto-rows-auto {
  grid-auto-rows: auto;
}"
        );
        assert_eq!(
            testing::generate_css("auto-rows-fr"),
            ".auto-rows-fr {
  grid-auto-rows: minmax(0, 1fr);
}"
        );
        assert_eq!(
            testing::generate_css(
                "auto-rows-[100px_minmax(100px,_auto)_10%_0.5fr_fit-content(400px)]"
            ),
            r".auto-rows-\[100px_minmax\(100px\,_auto\)_10\%_0\.5fr_fit-content\(400px\)\] {
  grid-auto-rows: 100px minmax(100px, auto) 10% 0.5fr fit-content(400px);
}"
        );
    }

    #[test]
    fn grid_column() {
        assert_eq!(
            testing::generate_css("col-auto"),
            ".col-auto {
  grid-column: auto;
}"
        );
        assert_eq!(
            testing::generate_css("col-span-12"),
            ".col-span-12 {
  grid-column: span 12 / span 12;
}"
        );
        assert_eq!(
            testing::generate_css("col-span-full"),
            ".col-span-full {
  grid-column: 1 / -1;
}"
        );
        assert_eq!(
            testing::generate_css("col-start-2"),
            ".col-start-2 {
  grid-column-start: 2;
}"
        );
        assert_eq!(
            testing::generate_css("col-end-4"),
            ".col-end-4 {
  grid-column-end: 4;
}"
        );
        assert_eq!(
            testing::generate_css("col-[span_2_/_7]"),
            r".col-\[span_2_\/_7\] {
  grid-column: span 2 / 7;
}"
        );
    }

    #[test]
    fn grid_row() {
        assert_eq!(
            testing::generate_css("row-auto"),
            ".row-auto {
  grid-row: auto;
}"
        );
        assert_eq!(
            testing::generate_css("row-span-12"),
            ".row-span-12 {
  grid-row: span 12 / span 12;
}"
        );
        assert_eq!(
            testing::generate_css("row-span-full"),
            ".row-span-full {
  grid-row: 1 / -1;
}"
        );
        assert_eq!(
            testing::generate_css("row-start-2"),
            ".row-start-2 {
  grid-row-start: 2;
}"
        );
        assert_eq!(
            testing::generate_css("row-end-4"),
            ".row-end-4 {
  grid-row-end: 4;
}"
        );
        assert_eq!(
            testing::generate_css("row-[span_2_/_7]"),
            r".row-\[span_2_\/_7\] {
  grid-row: span 2 / 7;
}"
        );
    }

    #[test]
    fn grid_template_columns() {
        assert_eq!(
            testing::generate_css("grid-cols-4"),
            ".grid-cols-4 {
  grid-template-columns: repeat(4, minmax(0, 1fr));
}"
        );
        assert_eq!(
            testing::generate_css("grid-cols-none"),
            ".grid-cols-none {
  grid-template-columns: none;
}"
        );
    }

    #[test]
    fn grid_template_rows() {
        assert_eq!(
            testing::generate_css("grid-rows-4"),
            ".grid-rows-4 {
  grid-template-rows: repeat(4, minmax(0, 1fr));
}"
        );
        assert_eq!(
            testing::generate_css("grid-rows-none"),
            ".grid-rows-none {
  grid-template-rows: none;
}"
        );
    }
}
