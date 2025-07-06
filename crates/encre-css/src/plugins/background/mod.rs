//! Background utilities
pub mod background_attachment;
pub mod background_clip;
pub mod background_color;
pub mod background_image;
pub mod background_origin;
pub mod background_position;
pub mod background_repeat;
pub mod background_size;
pub mod gradient_color_stops;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn background_attachment() {
        assert_eq!(
            generate(["bg-fixed"], &base_config()),
            ".bg-fixed {
  background-attachment: fixed;
}"
        );
    }

    #[test]
    fn background_clip() {
        assert_eq!(
            generate(["bg-clip-border"], &base_config()),
            ".bg-clip-border {
  background-clip: border-box;
}"
        );
        assert_eq!(
            generate(["bg-clip-padding"], &base_config()),
            ".bg-clip-padding {
  background-clip: padding-box;
}"
        );
        assert_eq!(
            generate(["bg-clip-text"], &base_config()),
            ".bg-clip-text {
  background-clip: text;
}"
        );
    }

    #[test]
    fn background_origin() {
        assert_eq!(
            generate(["bg-origin-padding"], &base_config()),
            ".bg-origin-padding {
  background-origin: padding-box;
}"
        );
    }

    #[test]
    fn background_repeat() {
        assert_eq!(
            generate(["bg-repeat-y"], &base_config()),
            ".bg-repeat-y {
  background-repeat: repeat-y;
}"
        );
    }

    #[test]
    fn background_size() {
        assert_eq!(
            generate(["bg-cover"], &base_config()),
            ".bg-cover {
  background-size: cover;
}"
        );
        assert_eq!(
            generate(["bg-[25%]"], &base_config()),
            r".bg-\[25\%\] {
  background-size: 25%;
}"
        );
    }

    #[test]
    fn background_position() {
        assert_eq!(
            generate(["bg-right-bottom"], &base_config()),
            ".bg-right-bottom {
  background-position: right bottom;
}"
        );
        assert_eq!(
            generate(["bg-[position:25%_100px]"], &base_config()),
            r".bg-\[position\:25\%_100px\] {
  background-position: 25% 100px;
}"
        );
    }

    #[test]
    fn background_image() {
        assert_eq!(
            generate(["bg-gradient-to-b"], &base_config()),
            ".bg-gradient-to-b {
  background-image: linear-gradient(to bottom, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-[url('/hello.png')]"], &base_config()),
            r".bg-\[url\(\'\/hello\.png\'\)\] {
  background-image: url('/hello.png');
}"
        );
        assert_eq!(
            generate(["bg-[url('/hello_with_underscores.png')]"], &base_config()),
            r".bg-\[url\(\'\/hello_with_underscores\.png\'\)\] {
  background-image: url('/hello_with_underscores.png');
}"
        );
    }

    #[test]
    fn background_color() {
        assert_eq!(
            generate(["bg-red-400"], &base_config()),
            ".bg-red-400 {
  background-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["bg-[rgb(12,12,12)]"], &base_config()),
            r".bg-\[rgb\(12\,12\,12\)\] {
  background-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["bg-[purple]"], &base_config()),
            r".bg-\[purple\] {
  background-color: purple;
}"
        );
    }
}
