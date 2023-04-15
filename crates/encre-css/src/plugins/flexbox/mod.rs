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
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn align_content() {
        assert_eq!(
            generate(["content-center"], &base_config()),
            ".content-center {
  align-content: center;
}"
        );

        assert_eq!(
            generate(["content-evenly"], &base_config()),
            ".content-evenly {
  align-content: space-evenly;
}"
        );
    }

    #[test]
    fn align_items() {
        assert_eq!(
            generate(["items-center"], &base_config()),
            ".items-center {
  align-items: center;
}"
        );

        assert_eq!(
            generate(["items-baseline"], &base_config()),
            ".items-baseline {
  align-items: baseline;
}"
        );
    }

    #[test]
    fn align_self() {
        assert_eq!(
            generate(["self-center"], &base_config()),
            ".self-center {
  align-self: center;
}"
        );

        assert_eq!(
            generate(["self-stretch"], &base_config()),
            ".self-stretch {
  align-self: stretch;
}"
        );
    }

    #[test]
    fn flex() {
        assert_eq!(
            generate(["flex-1"], &base_config()),
            ".flex-1 {
  flex: 1 1 0%;
}"
        );
        assert_eq!(
            generate(["flex-none"], &base_config()),
            ".flex-none {
  flex: none;
}"
        );
        assert_eq!(
            generate(["flex-[1_2_100px]"], &base_config()),
            r".flex-\[1_2_100px\] {
  flex: 1 2 100px;
}"
        );
    }

    #[test]
    fn flex_basis() {
        assert_eq!(
            generate(["basis-4"], &base_config()),
            ".basis-4 {
  flex-basis: 1rem;
}"
        );
        assert_eq!(
            generate(["basis-auto"], &base_config()),
            ".basis-auto {
  flex-basis: auto;
}"
        );
        assert_eq!(
            generate(["-basis-4"], &base_config()),
            ".-basis-4 {
  flex-basis: -1rem;
}"
        );
        assert_eq!(
            generate(["basis-[10px]"], &base_config()),
            r".basis-\[10px\] {
  flex-basis: 10px;
}"
        );
    }

    #[test]
    fn flex_direction() {
        assert_eq!(
            generate(["flex-row"], &base_config()),
            ".flex-row {
  flex-direction: row;
}"
        );

        assert_eq!(
            generate(["flex-col-reverse"], &base_config()),
            ".flex-col-reverse {
  flex-direction: column-reverse;
}"
        );
    }

    #[test]
    fn flex_grow() {
        assert_eq!(
            generate(["grow"], &base_config()),
            ".grow {
  flex-grow: 1;
}"
        );
        assert_eq!(
            generate(["grow-12"], &base_config()),
            ".grow-12 {
  flex-grow: 12;
}"
        );
    }

    #[test]
    fn flex_shrink() {
        assert_eq!(
            generate(["shrink"], &base_config()),
            ".shrink {
  flex-shrink: 1;
}"
        );
        assert_eq!(
            generate(["shrink-12"], &base_config()),
            ".shrink-12 {
  flex-shrink: 12;
}"
        );
    }

    #[test]
    fn flex_wrap() {
        assert_eq!(
            generate(["flex-nowrap"], &base_config()),
            ".flex-nowrap {
  flex-wrap: nowrap;
}"
        );
        assert_eq!(
            generate(["flex-wrap-reverse"], &base_config()),
            ".flex-wrap-reverse {
  flex-wrap: wrap-reverse;
}"
        );
    }

    #[test]
    fn justify_content() {
        assert_eq!(
            generate(["justify-center"], &base_config()),
            ".justify-center {
  justify-content: center;
}"
        );

        assert_eq!(
            generate(["justify-evenly"], &base_config()),
            ".justify-evenly {
  justify-content: space-evenly;
}"
        );
    }

    #[test]
    fn justify_items() {
        assert_eq!(
            generate(["justify-items-center"], &base_config()),
            ".justify-items-center {
  justify-items: center;
}"
        );

        assert_eq!(
            generate(["justify-items-end"], &base_config()),
            ".justify-items-end {
  justify-items: end;
}"
        );
    }

    #[test]
    fn justify_self() {
        assert_eq!(
            generate(["justify-self-center"], &base_config()),
            ".justify-self-center {
  justify-self: center;
}"
        );

        assert_eq!(
            generate(["justify-self-auto"], &base_config()),
            ".justify-self-auto {
  justify-self: auto;
}"
        );
    }

    #[test]
    fn place_content() {
        assert_eq!(
            generate(["place-content-center"], &base_config()),
            ".place-content-center {
  place-content: center;
}"
        );

        assert_eq!(
            generate(["place-content-evenly"], &base_config()),
            ".place-content-evenly {
  place-content: space-evenly;
}"
        );
    }

    #[test]
    fn place_items() {
        assert_eq!(
            generate(["place-items-center"], &base_config()),
            ".place-items-center {
  place-items: center;
}"
        );

        assert_eq!(
            generate(["place-items-end"], &base_config()),
            ".place-items-end {
  place-items: end;
}"
        );
    }

    #[test]
    fn place_self() {
        assert_eq!(
            generate(["place-self-center"], &base_config()),
            ".place-self-center {
  place-self: center;
}"
        );

        assert_eq!(
            generate(["place-self-auto"], &base_config()),
            ".place-self-auto {
  place-self: auto;
}"
        );
    }
}
