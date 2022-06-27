//! A TailwindCSS-compatible CSS generation library written in Rust
//!
//! ### Example
//!
//! ```rust
//! use encre_css::{EncreGenerator, Config};
//!
//! let mut generator = EncreGenerator::from_config(Config::default());
//! // Or let mut generator = EncreGenerator::new("encre.toml"); if your current directory contains an `encre.toml` file
//! generator.scan(r#"class="bg-red-500""#);
//!
//! assert!(generator.generate().expect("failed to generate the CSS").contains(r#".bg-red-500 {
//!   --en-bg-opacity: 1;
//!   background-color: rgb(239 68 68 / var(--en-bg-opacity));
//! }"#));
//! ```
//!
//! ### Cargo features
//!
//! - `rayon`: enables [rayon](https://docs.rs/rayon/latest/rayon) parallel iterators

pub mod config;
pub mod context;
pub mod error;
pub mod extractor;
pub mod generator;
pub mod plugins;
pub mod preflight;
pub mod selector;
pub mod utils;
pub mod variant;

pub use config::Config;
pub use error::Error;
pub use generator::EncreGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::DarkModeConfig, selector::Selector};

    use pretty_assertions::assert_eq;
    use std::{
        borrow::Cow,
        collections::{BTreeMap, BTreeSet},
        fs,
    };

    #[test]
    fn scan_test() {
        let config = Config::default();
        let expected = BTreeSet::from([
            Selector::new("flex", &config).unwrap(),
            Selector::new("w-full", &config).unwrap(),
            Selector::new("h-full", &config).unwrap(),
            Selector::new("absolute", &config).unwrap(),
            Selector::new("bg-blue-500", &config).unwrap(),
            Selector::new("border-[#333]", &config).unwrap(),
            Selector::new("text-[color:var(--hello)]", &config).unwrap(),
            Selector::new("sm:focus:ring", &config).unwrap(),
            Selector::new("hover:bg-black", &config).unwrap(),
        ]);

        let mut generator = EncreGenerator::from_config(config);
        generator.scan(
            r#"<div class="flex w-full h-full absolute bg-blue-500 foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"></div>"#
        );

        assert_eq!(expected, generator.scanned_selectors);
    }

    #[test]
    fn gen_selector_css_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("w-full");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.w-full {{
  width: 100%;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_with_custom_css_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("animate-pulse");
        generator.add_selector("animate-pulse");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

@-webkit-keyframes pulse {{
  50% {{
    opacity: .5;
  }}
}}

@keyframes pulse {{
  0%, 100% {{
    opacity: 1;
  }}
  50% {{
    opacity: .5;
  }}
}}

.animate-pulse {{
  -webkit-animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_css_arbitrary_value_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("w-[12px]");
        generator.add_selector("bg-[red]");
        generator.add_selector("bg-[url('../img/image_with_underscores.png')]");
        generator.add_selector("mt-[calc(100%-10px)]");
        generator.add_selector("2xl:pb-[calc((100%/2)-10px+2rem)]");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.mt-\[calc\(100\%-10px\)\] {{
  margin-top: calc(100% - 10px);
}}

.w-\[12px\] {{
  width: 12px;
}}

.bg-\[red\] {{
  background-color: red;
}}

.bg-\[url\(\'\.\.\/img\/image_with_underscores\.png\'\)\] {{
  background-image: url('../img/image_with_underscores.png');
}}

@media (min-width: 1536px) {{
  .\32xl\:pb-\[calc\(\(100\%\/2\)-10px\+2rem\)\] {{
    padding-bottom: calc((100% / 2) - 10px + 2rem);
  }}
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_css_arbitrary_value_hint_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("bg-[color:red]");
        generator.add_selector("hover:bg-[color:red]");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.bg-\[color\:red\] {{
  background-color: red;
}}

.hover\:bg-\[color\:red\]:hover {{
  background-color: red;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_with_variant_selector_css_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("focus:w-full");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.focus\:w-full:focus {{
  width: 100%;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS,
            )
        );
    }

    #[test]
    fn gen_selector_css_variants_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("sm:hover:bg-red-400");
        generator.add_selector("focus:hover:bg-red-600");
        generator.add_selector("active:rtl:bg-red-800");
        generator.add_selector("md:focus:selection:bg-blue-100");
        generator.add_selector("rtl:active:focus:lg:underline");
        generator.add_selector("print:ltr:xl:hover:focus:active:text-yellow-300");
        generator.add_selector("2xl:motion-safe:landscape:focus-within:visited:first:odd:checked:open:rtl:bg-purple-100");
        generator.add_selector("hover:file:bg-pink-600");
        generator.add_selector("file:hover:bg-pink-600");
        generator.add_selector("sm:before:target:content-[Hello_world!]");
        generator.add_selector("marker:selection:hover:bg-green-200");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

@media (min-width: 1536px) {{
  @media (prefers-reduced-motion: no-preference) {{
    @media (orientation: landscape) {{
      [dir="rtl"] .\32xl\:motion-safe\:landscape\:focus-within\:visited\:first\:odd\:checked\:open\:rtl\:bg-purple-100[open]:checked:nth-child(odd):first-child:visited:focus-within {{
        --en-bg-opacity: 1;
        background-color: rgb(243 232 255 / var(--en-bg-opacity));
      }}
    }}
  }}
}}

[dir="rtl"] .active\:rtl\:bg-red-800:active {{
  --en-bg-opacity: 1;
  background-color: rgb(153 27 27 / var(--en-bg-opacity));
}}

.file\:hover\:bg-pink-600:hover::file-selector-button {{
  --en-bg-opacity: 1;
  background-color: rgb(219 39 119 / var(--en-bg-opacity));
}}

.focus\:hover\:bg-red-600:hover:focus {{
  --en-bg-opacity: 1;
  background-color: rgb(220 38 38 / var(--en-bg-opacity));
}}

.hover\:file\:bg-pink-600::file-selector-button:hover {{
  --en-bg-opacity: 1;
  background-color: rgb(219 39 119 / var(--en-bg-opacity));
}}

.marker\:selection\:hover\:bg-green-200:hover::selection::marker {{
  --en-bg-opacity: 1;
  background-color: rgb(187 247 208 / var(--en-bg-opacity));
}}

@media (min-width: 768px) {{
  .md\:focus\:selection\:bg-blue-100::selection:focus {{
    --en-bg-opacity: 1;
    background-color: rgb(219 234 254 / var(--en-bg-opacity));
  }}
}}

@media (min-width: 640px) {{
  .sm\:hover\:bg-red-400:hover {{
    --en-bg-opacity: 1;
    background-color: rgb(248 113 113 / var(--en-bg-opacity));
  }}
}}

@media print {{
  @media (min-width: 1280px) {{
    [dir="ltr"] .print\:ltr\:xl\:hover\:focus\:active\:text-yellow-300:active:focus:hover {{
      --en-text-opacity: 1;
      color: rgb(253 224 71 / var(--en-text-opacity));
    }}
  }}
}}

@media (min-width: 1024px) {{
  [dir="rtl"] .rtl\:active\:focus\:lg\:underline:focus:active {{
    -webkit-text-decoration-line: underline;
    text-decoration-line: underline;
  }}
}}

@media (min-width: 640px) {{
  .sm\:before\:target\:content-\[Hello_world\!\]:target::before {{
    --en-content: "Hello world!";
    content: var(--en-content);
  }}
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_css_negative_values_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("-translate-x-52");
        generator.add_selector("-mb-8");
        generator.add_selector("-hue-rotate-60");
        generator.add_selector("hover:-hue-rotate-60");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.-mb-8 {{
  margin-bottom: -2rem;
}}

.-translate-x-52 {{
  --en-translate-x: -13rem;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}}

.-hue-rotate-60 {{
  --en-hue-rotate: hue-rotate(-60deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}}

.hover\:-hue-rotate-60:hover {{
  --en-hue-rotate: hue-rotate(-60deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_css_prevent_duplication_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("bg-red-500");
        generator.add_selector("bg-red-500");
        generator.add_selector("bg-red-500");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.bg-red-500 {{
  --en-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--en-bg-opacity));
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn default_modifier_values_for_rounded_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.scan("rounded-tr rounded-tr-md rounded rounded-md rounded-t-sm rounded-bl-xl border-x border border-4 border-t-2");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.rounded {{
  border-radius: 0.25rem;
}}

.rounded-md {{
  border-radius: 0.375rem;
}}

.rounded-t-sm {{
  border-top-left-radius: 0.125rem;
  border-top-right-radius: 0.125rem;
}}

.rounded-tr {{
  border-top-right-radius: 0.25rem;
}}

.rounded-tr-md {{
  border-top-right-radius: 0.375rem;
}}

.rounded-bl-xl {{
  border-bottom-left-radius: 0.75rem;
}}

.border {{
  border-width: 1px;
}}

.border-4 {{
  border-width: 4px;
}}

.border-x {{
  border-left-width: 1px;
  border-right-width: 1px;
}}

.border-t-2 {{
  border-top-width: 2px;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_css_for_font_with_spaces_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("font-[Times_New_Roman,Helvetica,serif]");
        generator.add_selector("font-[Roboto,_sans-serif]");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.font-\[Roboto\,_sans-serif\] {{
  font-family: Roboto, sans-serif;
}}

.font-\[Times_New_Roman\,Helvetica\,serif\] {{
  font-family: "Times New Roman",Helvetica,serif;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS,
            )
        );
    }

    #[test]
    fn gen_selector_css_for_container_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("container");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.container {{
  width: 100%;
}}

@media (min-width: 640px) {{
  .container {{
    max-width: 640px;
  }}
}}

@media (min-width: 768px) {{
  .container {{
    max-width: 768px;
  }}
}}

@media (min-width: 1024px) {{
  .container {{
    max-width: 1024px;
  }}
}}

@media (min-width: 1280px) {{
  .container {{
    max-width: 1280px;
  }}
}}

@media (min-width: 1536px) {{
  .container {{
    max-width: 1536px;
  }}
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS,
            )
        );

        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("md:container");
        generator.add_selector("md:mx-auto");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

@media (min-width: 768px) {{
  .md\:container {{
    width: 100%;
  }}
}}

@media (min-width: 768px) {{
  @media (min-width: 640px) {{
    .md\:container {{
      max-width: 640px;
    }}
  }}

  @media (min-width: 768px) {{
    .md\:container {{
      max-width: 768px;
    }}
  }}

  @media (min-width: 1024px) {{
    .md\:container {{
      max-width: 1024px;
    }}
  }}

  @media (min-width: 1280px) {{
    .md\:container {{
      max-width: 1280px;
    }}
  }}

  @media (min-width: 1536px) {{
    .md\:container {{
      max-width: 1536px;
    }}
  }}
}}

@media (min-width: 768px) {{
  .md\:mx-auto {{
    margin-left: auto;
    margin-right: auto;
  }}
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS,
            )
        );
    }

    #[test]
    fn gen_selector_css_with_dark_variant_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("dark:mt-px");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

@media (prefers-color-scheme: dark) {{
  .dark\:mt-px {{
    margin-top: 1px;
  }}
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );

        let mut config = Config::default();
        config.theme.dark_mode = DarkModeConfig::Class(Cow::from(".dark"));

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("dark:mt-px");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.dark .dark\:mt-px {{
  margin-top: 1px;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_css_with_custom_config_test() {
        let mut colors = BTreeMap::new();
        colors.insert(Cow::from("rosa-500"), Cow::from("#e5186a"));

        let mut screens = BTreeMap::new();
        screens.insert(Cow::from("3xl"), Cow::from("1600px"));

        let mut config = Config::default();
        config.theme.colors = colors;
        config.theme.screens = screens;

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("3xl:text-rosa-500");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

@media (min-width: 1600px) {{
  .\33xl\:text-rosa-500 {{
    --en-text-opacity: 1;
    color: rgb(229 24 106 / var(--en-text-opacity));
  }}
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn parse_config_file_test() {
        let mut config = Config::default();
        config
            .theme
            .colors
            .insert(Cow::from("rosa-500"), Cow::from("#e5186a"));
        config
            .theme
            .colors
            .insert(Cow::from("yellow-400"), Cow::from("#ffef0e"));
        config.theme.screens.remove(&Cow::from("lg"));
        config
            .theme
            .screens
            .insert(Cow::from("lg"), Cow::from("2000px"));
        config
            .theme
            .screens
            .insert(Cow::from("3xl"), Cow::from("1600px"));
        config.theme.dark_mode = DarkModeConfig::Class(Cow::from(".dark"));

        assert_eq!(
            Config::from_file("tests/fixtures/custom_config.toml").unwrap(),
            config
        );
    }

    #[test]
    fn config_is_extended_and_overridden_test() {
        let config = Config::from_file("tests/fixtures/custom_config.toml").unwrap();

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("bg-rosa-500");
        generator.add_selector("bg-yellow-400");
        generator.add_selector("bg-yellow-100");
        generator.add_selector("3xl:underline");
        generator.add_selector("lg:text-rosa-500");

        assert_eq!(
            generator.generate().unwrap(),
            format!(
                r#"{}

.bg-rosa-500 {{
  --en-bg-opacity: 1;
  background-color: rgb(229 24 106 / var(--en-bg-opacity));
}}

.bg-yellow-100 {{
  --en-bg-opacity: 1;
  background-color: rgb(254 249 195 / var(--en-bg-opacity));
}}

.bg-yellow-400 {{
  --en-bg-opacity: 1;
  background-color: rgb(255 239 14 / var(--en-bg-opacity));
}}

@media (min-width: 2000px) {{
  .lg\:text-rosa-500 {{
    --en-text-opacity: 1;
    color: rgb(229 24 106 / var(--en-text-opacity));
  }}
}}

@media (min-width: 1600px) {{
  .\33xl\:underline {{
    -webkit-text-decoration-line: underline;
    text-decoration-line: underline;
  }}
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn arbitrary_values_test() {
        let file_content = fs::read_to_string("tests/fixtures/arbitrary-values.html").unwrap();
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.scan(&file_content);
        generator.generate().unwrap();
    }
}
