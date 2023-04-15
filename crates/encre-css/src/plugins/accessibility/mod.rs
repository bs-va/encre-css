//! Accessibility utilities
pub mod screen_reader;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn screen_reader() {
        assert_eq!(
            &generate(["sr-only"], &base_config()),
            ".sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}"
        );
    }
}
