//! Flexbox utilities
pub mod align_content;
pub mod align_items;
pub mod align_self;
pub mod flex;
pub mod flex_basis;
pub mod flex_direction;
pub mod flex_grow;
pub mod flex_shrink;
pub mod flex_wrap;
pub mod justify_content;
pub mod justify_items;
pub mod justify_self;
pub mod order;
pub mod place_content;
pub mod place_items;
pub mod place_self;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn align_content() {
        assert_eq!(
            testing::generate_css("content-center"),
            ".content-center {
  align-content: center;
}"
        );

        assert_eq!(
            testing::generate_css("content-evenly"),
            ".content-evenly {
  align-content: space-evenly;
}"
        );
    }

    #[test]
    fn align_items() {
        assert_eq!(
            testing::generate_css("items-center"),
            ".items-center {
  align-items: center;
}"
        );

        assert_eq!(
            testing::generate_css("items-baseline"),
            ".items-baseline {
  align-items: baseline;
}"
        );
    }

    #[test]
    fn align_self() {
        assert_eq!(
            testing::generate_css("self-center"),
            ".self-center {
  align-self: center;
}"
        );

        assert_eq!(
            testing::generate_css("self-stretch"),
            ".self-stretch {
  align-self: stretch;
}"
        );
    }

    #[test]
    fn flex() {
        assert_eq!(
            testing::generate_css("flex-1"),
            ".flex-1 {
  flex: 1 1 0%;
}"
        );
        assert_eq!(
            testing::generate_css("flex-none"),
            ".flex-none {
  flex: none;
}"
        );
        assert_eq!(
            testing::generate_css("flex-[1_2_100px]"),
            r".flex-\[1_2_100px\] {
  flex: 1 2 100px;
}"
        );
    }

    #[test]
    fn flex_basis() {
        assert_eq!(
            testing::generate_css("basis-4"),
            ".basis-4 {
  flex-basis: 1rem;
}"
        );
        assert_eq!(
            testing::generate_css("basis-auto"),
            ".basis-auto {
  flex-basis: auto;
}"
        );
        assert_eq!(
            testing::generate_css("-basis-4"),
            ".-basis-4 {
  flex-basis: -1rem;
}"
        );
        assert_eq!(
            testing::generate_css("basis-[10px]"),
            r".basis-\[10px\] {
  flex-basis: 10px;
}"
        );
    }

    #[test]
    fn flex_direction() {
        assert_eq!(
            testing::generate_css("flex-row"),
            ".flex-row {
  flex-direction: row;
}"
        );

        assert_eq!(
            testing::generate_css("flex-col-reverse"),
            ".flex-col-reverse {
  flex-direction: column-reverse;
}"
        );
    }

    #[test]
    fn flex_grow() {
        assert_eq!(
            testing::generate_css("grow"),
            ".grow {
  flex-grow: 1;
}"
        );
        assert_eq!(
            testing::generate_css("grow-12"),
            ".grow-12 {
  flex-grow: 12;
}"
        );
    }

    #[test]
    fn flex_shrink() {
        assert_eq!(
            testing::generate_css("shrink"),
            ".shrink {
  flex-shrink: 1;
}"
        );
        assert_eq!(
            testing::generate_css("shrink-12"),
            ".shrink-12 {
  flex-shrink: 12;
}"
        );
    }

    #[test]
    fn flex_wrap() {
        assert_eq!(
            testing::generate_css("flex-nowrap"),
            ".flex-nowrap {
  flex-wrap: nowrap;
}"
        );
        assert_eq!(
            testing::generate_css("flex-wrap-reverse"),
            ".flex-wrap-reverse {
  flex-wrap: wrap-reverse;
}"
        );
    }

    #[test]
    fn justify_content() {
        assert_eq!(
            testing::generate_css("justify-center"),
            ".justify-center {
  justify-content: center;
}"
        );

        assert_eq!(
            testing::generate_css("justify-evenly"),
            ".justify-evenly {
  justify-content: space-evenly;
}"
        );
    }

    #[test]
    fn justify_items() {
        assert_eq!(
            testing::generate_css("justify-items-center"),
            ".justify-items-center {
  justify-items: center;
}"
        );

        assert_eq!(
            testing::generate_css("justify-items-end"),
            ".justify-items-end {
  justify-items: end;
}"
        );
    }

    #[test]
    fn justify_self() {
        assert_eq!(
            testing::generate_css("justify-self-center"),
            ".justify-self-center {
  justify-self: center;
}"
        );

        assert_eq!(
            testing::generate_css("justify-self-auto"),
            ".justify-self-auto {
  justify-self: auto;
}"
        );
    }

    #[test]
    fn place_content() {
        assert_eq!(
            testing::generate_css("place-content-center"),
            ".place-content-center {
  place-content: center;
}"
        );

        assert_eq!(
            testing::generate_css("place-content-evenly"),
            ".place-content-evenly {
  place-content: space-evenly;
}"
        );
    }

    #[test]
    fn place_items() {
        assert_eq!(
            testing::generate_css("place-items-center"),
            ".place-items-center {
  place-items: center;
}"
        );

        assert_eq!(
            testing::generate_css("place-items-end"),
            ".place-items-end {
  place-items: end;
}"
        );
    }

    #[test]
    fn place_self() {
        assert_eq!(
            testing::generate_css("place-self-center"),
            ".place-self-center {
  place-self: center;
}"
        );

        assert_eq!(
            testing::generate_css("place-self-auto"),
            ".place-self-auto {
  place-self: auto;
}"
        );
    }
}
