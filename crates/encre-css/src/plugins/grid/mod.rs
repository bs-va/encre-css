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
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn gap() {
        assert_eq!(
            generate(["gap-4"], &base_config()),
            ".gap-4 {
  gap: 1rem;
}"
        );
        assert_eq!(
            generate(["-gap-4"], &base_config()),
            ".-gap-4 {
  gap: -1rem;
}"
        );
        assert_eq!(
            generate(["gap-[40px]"], &base_config()),
            r".gap-\[40px\] {
  gap: 40px;
}"
        );
    }

    #[test]
    fn grid_auto_columns() {
        assert_eq!(
            generate(["auto-cols-auto"], &base_config()),
            ".auto-cols-auto {
  grid-auto-columns: auto;
}"
        );
        assert_eq!(
            generate(["auto-cols-fr"], &base_config()),
            ".auto-cols-fr {
  grid-auto-columns: minmax(0, 1fr);
}"
        );
        assert_eq!(
            generate(
                ["auto-cols-[100px_minmax(100px,_auto)_10%_0.5fr_fit-content(400px)]"],
                &base_config(),
            ),
            r".auto-cols-\[100px_minmax\(100px\,_auto\)_10\%_0\.5fr_fit-content\(400px\)\] {
  grid-auto-columns: 100px minmax(100px, auto) 10% 0.5fr fit-content(400px);
}"
        );
    }

    #[test]
    fn grid_auto_flow() {
        assert_eq!(
            generate(["grid-flow-row"], &base_config()),
            ".grid-flow-row {
  grid-auto-flow: row;
}"
        );
        assert_eq!(
            generate(["grid-flow-col-dense"], &base_config()),
            ".grid-flow-col-dense {
  grid-auto-flow: column dense;
}"
        );
    }

    #[test]
    fn grid_auto_rows() {
        assert_eq!(
            generate(["auto-rows-auto"], &base_config()),
            ".auto-rows-auto {
  grid-auto-rows: auto;
}"
        );
        assert_eq!(
            generate(["auto-rows-fr"], &base_config()),
            ".auto-rows-fr {
  grid-auto-rows: minmax(0, 1fr);
}"
        );
        assert_eq!(
            generate(
                ["auto-rows-[100px_minmax(100px,_auto)_10%_0.5fr_fit-content(400px)]"],
                &base_config(),
            ),
            r".auto-rows-\[100px_minmax\(100px\,_auto\)_10\%_0\.5fr_fit-content\(400px\)\] {
  grid-auto-rows: 100px minmax(100px, auto) 10% 0.5fr fit-content(400px);
}"
        );
    }

    #[test]
    fn grid_column() {
        assert_eq!(
            generate(["col-auto"], &base_config()),
            ".col-auto {
  grid-column: auto;
}"
        );
        assert_eq!(
            generate(["col-span-12"], &base_config()),
            ".col-span-12 {
  grid-column: span 12 / span 12;
}"
        );
        assert_eq!(
            generate(["col-span-full"], &base_config()),
            ".col-span-full {
  grid-column: 1 / -1;
}"
        );
        assert_eq!(
            generate(["col-start-2"], &base_config()),
            ".col-start-2 {
  grid-column-start: 2;
}"
        );
        assert_eq!(
            generate(["col-end-4"], &base_config()),
            ".col-end-4 {
  grid-column-end: 4;
}"
        );
        assert_eq!(
            generate(["col-[span_2_/_7]"], &base_config()),
            r".col-\[span_2_\/_7\] {
  grid-column: span 2 / 7;
}"
        );
    }

    #[test]
    fn grid_row() {
        assert_eq!(
            generate(["row-auto"], &base_config()),
            ".row-auto {
  grid-row: auto;
}"
        );
        assert_eq!(
            generate(["row-span-12"], &base_config()),
            ".row-span-12 {
  grid-row: span 12 / span 12;
}"
        );
        assert_eq!(
            generate(["row-span-full"], &base_config()),
            ".row-span-full {
  grid-row: 1 / -1;
}"
        );
        assert_eq!(
            generate(["row-start-2"], &base_config()),
            ".row-start-2 {
  grid-row-start: 2;
}"
        );
        assert_eq!(
            generate(["row-end-4"], &base_config()),
            ".row-end-4 {
  grid-row-end: 4;
}"
        );
        assert_eq!(
            generate(["row-[span_2_/_7]"], &base_config()),
            r".row-\[span_2_\/_7\] {
  grid-row: span 2 / 7;
}"
        );
    }

    #[test]
    fn grid_template_columns() {
        assert_eq!(
            generate(["grid-cols-4"], &base_config()),
            ".grid-cols-4 {
  grid-template-columns: repeat(4, minmax(0, 1fr));
}"
        );
        assert_eq!(
            generate(["grid-cols-none"], &base_config()),
            ".grid-cols-none {
  grid-template-columns: none;
}"
        );
    }

    #[test]
    fn grid_template_rows() {
        assert_eq!(
            generate(["grid-rows-4"], &base_config()),
            ".grid-rows-4 {
  grid-template-rows: repeat(4, minmax(0, 1fr));
}"
        );
        assert_eq!(
            generate(["grid-rows-none"], &base_config()),
            ".grid-rows-none {
  grid-template-rows: none;
}"
        );
    }
}
