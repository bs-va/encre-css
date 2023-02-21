//! Transform utilities
pub mod rotate;
pub mod scale;
pub mod skew;
pub mod transform_origin;
pub mod transform_type;
pub mod translate;

const CSS_TRANSFORM: &str = "transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));";

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn rotate() {
        assert_eq!(testing::generate_css("rotate-20"), ".rotate-20 {
  --en-rotate: 20deg;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(
            testing::generate_css("rotate-[1.25turn]"),
            r".rotate-\[1\.25turn\] {
  --en-rotate: 1.25turn;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}"
        );
    }

    #[test]
    fn scale() {
        assert_eq!(testing::generate_css("scale-150"), ".scale-150 {
  --en-scale-x: 1.5;
  --en-scale-y: 1.5;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(testing::generate_css("scale-x-20"), ".scale-x-20 {
  --en-scale-x: 0.2;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(testing::generate_css("scale-y-45"), ".scale-y-45 {
  --en-scale-y: 0.45;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
    }

    #[test]
    fn skew() {
        assert_eq!(testing::generate_css("skew-x-20"), ".skew-x-20 {
  --en-skew-x: 20deg;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(testing::generate_css("skew-y-20"), ".skew-y-20 {
  --en-skew-y: 20deg;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(
            testing::generate_css("skew-x-[1.25turn]"),
            r".skew-x-\[1\.25turn\] {
  --en-skew-x: 1.25turn;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}"
        );
        assert_eq!(
            testing::generate_css("skew-y-[1.25turn]"),
            r".skew-y-\[1\.25turn\] {
  --en-skew-y: 1.25turn;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}"
        );
    }

    #[test]
    fn translate() {
        assert_eq!(testing::generate_css("translate-x-20"), ".translate-x-20 {
  --en-translate-x: 5rem;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(testing::generate_css("-translate-x-20"), ".-translate-x-20 {
  --en-translate-x: -5rem;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(testing::generate_css("translate-y-20"), ".translate-y-20 {
  --en-translate-y: 5rem;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(testing::generate_css("translate-x-auto"), ".translate-x-auto {
  --en-translate-x: auto;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(testing::generate_css("translate-y-full"), ".translate-y-full {
  --en-translate-y: 100%;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(
            testing::generate_css("translate-x-[1.25%]"),
            r".translate-x-\[1\.25\%\] {
  --en-translate-x: 1.25%;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}"
        );
        assert_eq!(
            testing::generate_css("translate-y-[10px]"),
            r".translate-y-\[10px\] {
  --en-translate-y: 10px;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}"
        );
    }

    #[test]
    fn transform_origin() {
        assert_eq!(
            testing::generate_css("origin-center"),
            ".origin-center {
  transform-origin: center;
}"
        );
        assert_eq!(
            testing::generate_css("origin-[bottom_right_60px]"),
            r".origin-\[bottom_right_60px\] {
  transform-origin: bottom right 60px;
}"
        );
        assert_eq!(
            testing::generate_css("origin-[-100%_40%]"),
            r".origin-\[-100\%_40\%\] {
  transform-origin: -100% 40%;
}"
        );
    }

    #[test]
    fn transform_type() {
        assert_eq!(testing::generate_css("transform-gpu"), ".transform-gpu {
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), 0) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}");
        assert_eq!(
            testing::generate_css("transform-none"),
            ".transform-none {
  transform: none;
}"
        );
    }
}
