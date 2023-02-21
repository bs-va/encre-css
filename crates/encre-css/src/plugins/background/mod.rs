//! Background utilities
pub mod background_attachment;
pub mod background_clip;
pub mod background_color;
pub mod background_image;
pub mod background_opacity;
pub mod background_origin;
pub mod background_position;
pub mod background_repeat;
pub mod background_size;
pub mod gradient_color_stops;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn background_attachment() {
        assert_eq!(
            testing::generate_css("bg-fixed"),
            ".bg-fixed {
  background-attachment: fixed;
}"
        );
    }

    #[test]
    fn background_clip() {
        assert_eq!(
            testing::generate_css("bg-clip-border"),
            ".bg-clip-border {
  background-clip: border-box;
}"
        );
        assert_eq!(
            testing::generate_css("bg-clip-padding"),
            ".bg-clip-padding {
  background-clip: padding-box;
}"
        );
        assert_eq!(
            testing::generate_css("bg-clip-text"),
            ".bg-clip-text {
  background-clip: text;
}"
        );
    }

    #[test]
    fn background_origin() {
        assert_eq!(
            testing::generate_css("bg-origin-padding"),
            ".bg-origin-padding {
  background-origin: padding-box;
}"
        );
    }

    #[test]
    fn background_repeat() {
        assert_eq!(
            testing::generate_css("bg-repeat-y"),
            ".bg-repeat-y {
  background-repeat: repeat-y;
}"
        );
    }

    #[test]
    fn background_size() {
        assert_eq!(
            testing::generate_css("bg-cover"),
            ".bg-cover {
  background-size: cover;
}"
        );
        assert_eq!(
            testing::generate_css("bg-[25%]"),
            r".bg-\[25\%\] {
  background-size: 25%;
}"
        );
    }

    #[test]
    fn background_position() {
        assert_eq!(
            testing::generate_css("bg-right-bottom"),
            ".bg-right-bottom {
  background-position: right bottom;
}"
        );
        assert_eq!(
            testing::generate_css("bg-[position:25%_100px]"),
            r".bg-\[position\:25\%_100px\] {
  background-position: 25% 100px;
}"
        );
    }

    #[test]
    fn background_image() {
        assert_eq!(
            testing::generate_css("bg-gradient-to-b"),
            ".bg-gradient-to-b {
  background-image: linear-gradient(to bottom, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            testing::generate_css("bg-[url('/hello.png')]"),
            r".bg-\[url\(\'\/hello\.png\'\)\] {
  background-image: url('/hello.png');
}"
        );
        assert_eq!(
            testing::generate_css("bg-[url('/hello_with_underscores.png')]"),
            r".bg-\[url\(\'\/hello_with_underscores\.png\'\)\] {
  background-image: url('/hello_with_underscores.png');
}"
        );
    }

    #[test]
    fn background_color() {
        assert_eq!(
            testing::generate_css("bg-red-400"),
            ".bg-red-400 {
  --en-bg-opacity: 1;
  background-color: rgb(248 113 113 / var(--en-bg-opacity));
}"
        );
        assert_eq!(
            testing::generate_css("bg-[rgb(12,12,12)]"),
            r".bg-\[rgb\(12\,12\,12\)\] {
  background-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            testing::generate_css("bg-[purple]"),
            r".bg-\[purple\] {
  background-color: purple;
}"
        );
    }

    #[test]
    fn background_opacity() {
        assert_eq!(
            testing::generate_css("bg-red-400/12"),
            r".bg-red-400\/12 {
  background-color: rgb(248 113 113 / 0.12);
}"
        );
        assert_eq!(
            testing::generate_css("bg-opacity-12"),
            ".bg-opacity-12 {
  --en-bg-opacity: 0.12;
}"
        );
    }
}
