//! Typography utilities
pub mod content;
pub mod font_family;
pub mod font_size;
pub mod font_smoothing;
pub mod font_style;
pub mod font_variant_numeric;
pub mod font_weight;
pub mod letter_spacing;
pub mod line_clamp;
pub mod line_height;
pub mod list_style_position;
pub mod list_style_type;
pub mod text_align;
pub mod text_color;
pub mod text_decoration;
pub mod text_decoration_color;
pub mod text_decoration_style;
pub mod text_decoration_thickness;
pub mod text_indent;
pub mod text_opacity;
pub mod text_overflow;
pub mod text_transform;
pub mod text_underline_offset;
pub mod vertical_align;
pub mod whitespace;
pub mod word_break;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn content() {
        assert_eq!(
            testing::generate_css("before:content-none"),
            r".before\:content-none::before {
  --en-content: none;
  content: var(--en-content);
}"
        );
        assert_eq!(
            testing::generate_css("before:content-['1234_some_words']"),
            r".before\:content-\[\'1234_some_words\'\]::before {
  --en-content: '1234 some words';
  content: var(--en-content);
}"
        );
        assert_eq!(
            testing::generate_css("before:content-[':-><-:']"),
            r".before\:content-\[\'\:-\>\<-\:\'\]::before {
  --en-content: ':-><-:';
  content: var(--en-content);
}"
        );
    }

    #[test]
    fn font_family() {
        assert_eq!(
            testing::generate_css("font-mono"),
            r#".font-mono {
  font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}"#
        );
        assert_eq!(
            testing::generate_css("font-['Open_Sans',Roboto,sans-serif]"),
            r".font-\[\'Open_Sans\'\,Roboto\,sans-serif\] {
  font-family: 'Open Sans',Roboto,sans-serif;
}"
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn font_size() {
        assert_eq!(
            testing::generate_css("text-xs"),
            ".text-xs {
  font-size: 0.75rem;
  line-height: 1rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-sm"),
            ".text-sm {
  font-size: 0.875rem;
  line-height: 1.25rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-base"),
            ".text-base {
  font-size: 1rem;
  line-height: 1.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-lg"),
            ".text-lg {
  font-size: 1.125rem;
  line-height: 1.75rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-xl"),
            ".text-xl {
  font-size: 1.25rem;
  line-height: 1.75rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-2xl"),
            ".text-2xl {
  font-size: 1.5rem;
  line-height: 2rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-3xl"),
            ".text-3xl {
  font-size: 1.875rem;
  line-height: 2.25rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-4xl"),
            ".text-4xl {
  font-size: 2.25rem;
  line-height: 2.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("text-5xl"),
            ".text-5xl {
  font-size: 3rem;
  line-height: 1;
}"
        );
        assert_eq!(
            testing::generate_css("text-6xl"),
            ".text-6xl {
  font-size: 3.75rem;
  line-height: 1;
}"
        );
        assert_eq!(
            testing::generate_css("text-7xl"),
            ".text-7xl {
  font-size: 4.5rem;
  line-height: 1;
}"
        );
        assert_eq!(
            testing::generate_css("text-8xl"),
            ".text-8xl {
  font-size: 6rem;
  line-height: 1;
}"
        );
        assert_eq!(
            testing::generate_css("text-9xl"),
            ".text-9xl {
  font-size: 8rem;
  line-height: 1;
}"
        );
        assert_eq!(
            testing::generate_css("text-[18px]"),
            r".text-\[18px\] {
  font-size: 18px;
}"
        );
        assert_eq!(
            testing::generate_css("text-[10%]"),
            r".text-\[10\%\] {
  font-size: 10%;
}"
        );
        assert_eq!(
            testing::generate_css("text-[x-large]"),
            r".text-\[x-large\] {
  font-size: x-large;
}"
        );
        assert_eq!(
            testing::generate_css("text-[smaller]"),
            r".text-\[smaller\] {
  font-size: smaller;
}"
        );
    }

    #[test]
    fn font_smoothing() {
        assert_eq!(
            testing::generate_css("antialised"),
            ".antialised {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}"
        );
        assert_eq!(
            testing::generate_css("subpixel-antialised"),
            ".subpixel-antialised {
  -webkit-font-smoothing: auto;
  -moz-osx-font-smoothing: auto;
}"
        );
    }

    #[test]
    fn font_style() {
        assert_eq!(
            testing::generate_css("italic"),
            ".italic {
  font-style: italic;
}"
        );
        assert_eq!(
            testing::generate_css("not-italic"),
            ".not-italic {
  font-style: normal;
}"
        );
    }

    #[test]
    fn font_variant_numeric() {
        assert_eq!(
            testing::generate_css("normal-nums"),
            ".normal-nums {
  font-variant-numeric: normal;
}"
        );
        assert_eq!(
            testing::generate_css("ordinal"),
            ".ordinal {
  --en-ordinal: ordinal;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            testing::generate_css("slashed-zero"),
            ".slashed-zero {
  --en-slashed-zero: slashed-zero;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            testing::generate_css("lining-nums"),
            ".lining-nums {
  --en-numeric-figure: lining-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            testing::generate_css("oldstyle-nums"),
            ".oldstyle-nums {
  --en-numeric-figure: oldstyle-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            testing::generate_css("proportional-nums"),
            ".proportional-nums {
  --en-numeric-spacing: proportional-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            testing::generate_css("tabular-nums"),
            ".tabular-nums {
  --en-numeric-spacing: tabular-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            testing::generate_css("diagonal-fractions"),
            ".diagonal-fractions {
  --en-numeric-fraction: diagonal-fractions;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            testing::generate_css("stacked-fractions"),
            ".stacked-fractions {
  --en-numeric-fraction: stacked-fractions;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
    }

    #[test]
    fn font_weight() {
        assert_eq!(
            testing::generate_css("font-thin"),
            ".font-thin {
  font-weight: 100;
}"
        );
        assert_eq!(
            testing::generate_css("font-extralight"),
            ".font-extralight {
  font-weight: 200;
}"
        );
        assert_eq!(
            testing::generate_css("font-light"),
            ".font-light {
  font-weight: 300;
}"
        );
        assert_eq!(
            testing::generate_css("font-normal"),
            ".font-normal {
  font-weight: 400;
}"
        );
        assert_eq!(
            testing::generate_css("font-medium"),
            ".font-medium {
  font-weight: 500;
}"
        );
        assert_eq!(
            testing::generate_css("font-semibold"),
            ".font-semibold {
  font-weight: 600;
}"
        );
        assert_eq!(
            testing::generate_css("font-bold"),
            ".font-bold {
  font-weight: 700;
}"
        );
        assert_eq!(
            testing::generate_css("font-extrabold"),
            ".font-extrabold {
  font-weight: 800;
}"
        );
        assert_eq!(
            testing::generate_css("font-black"),
            ".font-black {
  font-weight: 900;
}"
        );
        assert_eq!(
            testing::generate_css("font-[50]"),
            r".font-\[50\] {
  font-weight: 50;
}"
        );
    }

    #[test]
    fn letter_spacing() {
        assert_eq!(
            testing::generate_css("tracking-tighter"),
            ".tracking-tighter {
  letter-spacing: -0.05em;
}"
        );
        assert_eq!(
            testing::generate_css("tracking-tight"),
            ".tracking-tight {
  letter-spacing: -0.025em;
}"
        );
        assert_eq!(
            testing::generate_css("tracking-normal"),
            ".tracking-normal {
  letter-spacing: 0;
}"
        );
        assert_eq!(
            testing::generate_css("tracking-wide"),
            ".tracking-wide {
  letter-spacing: 0.025em;
}"
        );
        assert_eq!(
            testing::generate_css("tracking-wider"),
            ".tracking-wider {
  letter-spacing: 0.05em;
}"
        );
        assert_eq!(
            testing::generate_css("tracking-widest"),
            ".tracking-widest {
  letter-spacing: 0.1em;
}"
        );
        assert_eq!(
            testing::generate_css("tracking-[10px]"),
            r".tracking-\[10px\] {
  letter-spacing: 10px;
}"
        );
    }

    #[test]
    fn line_clamp() {
        assert_eq!(
            testing::generate_css("line-clamp-none"),
            ".line-clamp-none {
  -webkit-line-clamp: unset;
}"
        );
        assert_eq!(
            testing::generate_css("line-clamp-12"),
            ".line-clamp-12 {
  overflow: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 12;
}"
        );
    }

    #[test]
    fn line_height() {
        assert_eq!(
            testing::generate_css("leading-none"),
            ".leading-none {
  line-height: 1;
}"
        );
        assert_eq!(
            testing::generate_css("leading-tight"),
            ".leading-tight {
  line-height: 1.25;
}"
        );
        assert_eq!(
            testing::generate_css("leading-snug"),
            ".leading-snug {
  line-height: 1.375;
}"
        );
        assert_eq!(
            testing::generate_css("leading-normal"),
            ".leading-normal {
  line-height: 1.5;
}"
        );
        assert_eq!(
            testing::generate_css("leading-relaxed"),
            ".leading-relaxed {
  line-height: 1.625;
}"
        );
        assert_eq!(
            testing::generate_css("leading-loose"),
            ".leading-loose {
  line-height: 2;
}"
        );
        assert_eq!(
            testing::generate_css("leading-4"),
            ".leading-4 {
  line-height: 1rem;
}"
        );
        assert_eq!(
            testing::generate_css("-leading-4"),
            ".-leading-4 {
  line-height: -1rem;
}"
        );
        assert_eq!(
            testing::generate_css("leading-1/2"),
            r".leading-1\/2 {
  line-height: 50%;
}"
        );
        assert_eq!(
            testing::generate_css("leading-[8]"),
            r".leading-\[8\] {
  line-height: 8;
}"
        );
        assert_eq!(
            testing::generate_css("leading-[16px]"),
            r".leading-\[16px\] {
  line-height: 16px;
}"
        );
        assert_eq!(
            testing::generate_css("leading-[22%]"),
            r".leading-\[22\%\] {
  line-height: 22%;
}"
        );
    }

    #[test]
    fn list_style_position() {
        assert_eq!(
            testing::generate_css("list-inside"),
            ".list-inside {
  list-style-position: inside;
}"
        );
        assert_eq!(
            testing::generate_css("list-outside"),
            ".list-outside {
  list-style-position: outside;
}"
        );
    }

    #[test]
    fn list_style_type() {
        assert_eq!(
            testing::generate_css("list-disc"),
            ".list-disc {
  list-style-type: disc;
}"
        );
        assert_eq!(
            testing::generate_css("list-decimal"),
            ".list-decimal {
  list-style-type: decimal;
}"
        );
        assert_eq!(
            testing::generate_css("list-none"),
            ".list-none {
  list-style-type: none;
}"
        );
        assert_eq!(
            testing::generate_css("list-[greek]"),
            r".list-\[greek\] {
  list-style-type: greek;
}"
        );
    }

    #[test]
    fn text_align() {
        assert_eq!(
            testing::generate_css("text-center"),
            ".text-center {
  text-align: center;
}"
        );
        assert_eq!(
            testing::generate_css("text-justify"),
            ".text-justify {
  text-align: justify;
}"
        );
    }

    #[test]
    fn text_color() {
        assert_eq!(
            testing::generate_css("text-red-400"),
            ".text-red-400 {
  --en-text-opacity: 1;
  color: rgb(248 113 113 / var(--en-text-opacity));
}"
        );
        assert_eq!(
            testing::generate_css("text-[rgb(12,12,12)]"),
            r".text-\[rgb\(12\,12\,12\)\] {
  color: rgb(12,12,12);
}"
        );
        assert_eq!(
            testing::generate_css("text-[purple]"),
            r".text-\[purple\] {
  color: purple;
}"
        );
    }

    #[test]
    fn text_decoration() {
        assert_eq!(
            testing::generate_css("underline"),
            ".underline {
  -webkit-text-decoration-line: underline;
  text-decoration-line: underline;
}"
        );
        assert_eq!(
            testing::generate_css("no-underline"),
            ".no-underline {
  -webkit-text-decoration-line: none;
  text-decoration-line: none;
}"
        );
    }

    #[test]
    fn text_decoration_color() {
        assert_eq!(
            testing::generate_css("decoration-red-400"),
            ".decoration-red-400 {
  -webkit-text-decoration-color: rgb(248 113 113);
  text-decoration-color: rgb(248 113 113);
}"
        );
        assert_eq!(
            testing::generate_css("decoration-[rgb(12,12,12)]"),
            r".decoration-\[rgb\(12\,12\,12\)\] {
  -webkit-text-decoration-color: rgb(12,12,12);
  text-decoration-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            testing::generate_css("decoration-[purple]"),
            r".decoration-\[purple\] {
  -webkit-text-decoration-color: purple;
  text-decoration-color: purple;
}"
        );
    }

    #[test]
    fn text_decoration_style() {
        assert_eq!(
            testing::generate_css("decoration-wavy"),
            ".decoration-wavy {
  text-decoration-style: wavy;
}"
        );
    }

    #[test]
    fn text_decoration_thickness() {
        assert_eq!(
            testing::generate_css("decoration-auto"),
            ".decoration-auto {
  text-decoration-thickness: auto;
}"
        );
        assert_eq!(
            testing::generate_css("decoration-from-font"),
            ".decoration-from-font {
  text-decoration-thickness: from-font;
}"
        );
        assert_eq!(
            testing::generate_css("decoration-12"),
            ".decoration-12 {
  text-decoration-thickness: 12px;
}"
        );
        assert_eq!(
            testing::generate_css("decoration-[4.2rem]"),
            r".decoration-\[4\.2rem\] {
  text-decoration-thickness: 4.2rem;
}"
        );
        assert_eq!(
            testing::generate_css("decoration-[2%]"),
            r".decoration-\[2\%\] {
  text-decoration-thickness: 2%;
}"
        );
    }

    #[test]
    fn text_indent() {
        assert_eq!(
            testing::generate_css("indent-2"),
            ".indent-2 {
  text-indent: 0.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("-indent-2"),
            ".-indent-2 {
  text-indent: -0.5rem;
}"
        );
        assert_eq!(
            testing::generate_css("indent-[20px]"),
            r".indent-\[20px\] {
  text-indent: 20px;
}"
        );
    }

    #[test]
    fn text_opacity() {
        assert_eq!(
            testing::generate_css("text-red-400/12"),
            r".text-red-400\/12 {
  color: rgb(248 113 113 / 0.12);
}"
        );
        assert_eq!(
            testing::generate_css("text-opacity-12"),
            ".text-opacity-12 {
  --en-text-opacity: 0.12;
}"
        );
    }

    #[test]
    fn text_overflow() {
        assert_eq!(
            testing::generate_css("truncate"),
            ".truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}"
        );
        assert_eq!(
            testing::generate_css("text-ellipsis"),
            ".text-ellipsis {
  text-overflow: ellipsis;
}"
        );
        assert_eq!(
            testing::generate_css("text-clip"),
            ".text-clip {
  text-overflow: clip;
}"
        );
    }

    #[test]
    fn text_transform() {
        assert_eq!(
            testing::generate_css("normal-case"),
            ".normal-case {
  text-transform: none;
}"
        );
        assert_eq!(
            testing::generate_css("uppercase"),
            ".uppercase {
  text-transform: uppercase;
}"
        );
    }

    #[test]
    fn text_underline_offset() {
        assert_eq!(
            testing::generate_css("underline-offset-auto"),
            ".underline-offset-auto {
  text-underline-offset: auto;
}"
        );
        assert_eq!(
            testing::generate_css("underline-offset-12"),
            ".underline-offset-12 {
  text-underline-offset: 12px;
}"
        );
        assert_eq!(
            testing::generate_css("underline-offset-[2rem]"),
            r".underline-offset-\[2rem\] {
  text-underline-offset: 2rem;
}"
        );
        assert_eq!(
            testing::generate_css("underline-offset-[10%]"),
            r".underline-offset-\[10\%\] {
  text-underline-offset: 10%;
}"
        );
    }

    #[test]
    fn vertical_align() {
        assert_eq!(
            testing::generate_css("align-sub"),
            ".align-sub {
  vertical-align: sub;
}"
        );
        assert_eq!(
            testing::generate_css("align-text-top"),
            ".align-text-top {
  vertical-align: text-top;
}"
        );
    }

    #[test]
    fn whitespace() {
        assert_eq!(
            testing::generate_css("whitespace-normal"),
            ".whitespace-normal {
  white-space: normal;
}"
        );
        assert_eq!(
            testing::generate_css("whitespace-pre-wrap"),
            ".whitespace-pre-wrap {
  white-space: pre-wrap;
}"
        );
    }

    #[test]
    fn word_break() {
        assert_eq!(
            testing::generate_css("break-normal"),
            ".break-normal {
  overflow-wrap: normal;
  word-break: normal;
}"
        );
        assert_eq!(
            testing::generate_css("break-keep"),
            ".break-keep {
  word-break: keep-all;
}"
        );
    }
}
