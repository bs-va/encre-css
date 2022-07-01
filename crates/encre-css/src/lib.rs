//! A TailwindCSS-compatible CSS generation library written in Rust.
//!
//! ## A brief introduction to utility-first CSS frameworks
//!
//! Traditionally, whenever you need to style something on the web, you write CSS in a dedicated
//! file and apply the rules using classes in your HTML, like that:
//!
//! <div style="margin-top: 1rem; margin-bottom: 1rem;"></div>
//!
//! <div class="notification">
//!   <div class="notification-header">
//!     <div class="app-icon"></div>
//!     A new Javascript library was released!
//!   </div>
//!   <div class="notification-body">
//!     The library <code>react</code> was just released, do you know it?
//!     It is <i>a JavaScript library for creating user interfaces</i>.
//!   </div>
//!   <div class="notification-footer">
//!     <a href="#" class="dismiss-button">Dismiss</a>
//!     <div class="blank-space"></div>
//!     <a href="#" class="try-button">Try it here!</a>
//!   </div>
//! </div>
//!
//! <div style="margin-top: 1rem; margin-bottom: 1rem;"></div>
//!
//! <style>
//! .notification {
//!   width: 30rem;
//!   box-shadow: 1px 1px 5px 2px #e5e7eb;
//!   border-radius: 0.8rem;
//!   font-family: sans-serif;
//! }
//!
//! .notification-header {
//!   padding: 0.5rem 1rem;
//!   display: flex;
//!   align-items: center;
//! }
//!
//! .app-icon {
//!   background-color: rgb(37 99 235);
//!   border-radius: 50%;
//!   width: 1.2rem;
//!   height: 1.2rem;
//!   margin-right: 1rem;
//! }
//!
//! .notification-body {
//!   padding: 1rem 1.5rem 1.5rem 1.5rem;
//! }
//!
//! .notification-footer {
//!   display: flex;
//! }
//!
//! .dismiss-button {
//!   color: rgb(225 29 72);
//!   padding: 0.5rem 1rem;
//! }
//!
//! .blank-space {
//!   flex: 1;
//! }
//!
//! .try-button {
//!   background-color: rgb(37 99 235);
//!   color: white;
//!   border-bottom-right-radius: 0.8rem;
//!   border-top-left-radius: 0.8rem;
//!   padding: 0.5rem 1rem;
//!   box-shadow: 1px 1px 5px 1px rgb(37 99 235);
//! }
//! </style>
//
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">&lt;div</span> class=<span class="string">"notification"</span><span class="kw">&gt;
//!   &lt;div</span> class=<span class="string">"notification-header"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;div</span> class=<span class="string">"app-icon"</span><span class="kw">&gt;&lt;/div&gt;</span>
//!     A new Javascript library was released!
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"notification-body"</span><span class="kw">&gt;</span>
//!     The library <span class="kw">&lt;code&gt;</span>react<span class="kw">&lt;/code&gt;</span> was just released, did you know it?
//!     It is <span class="kw">&lt;i&gt;</span>a JavaScript library for creating user interfaces<span class="kw">&lt;/i&gt;</span>.
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"notification-footer"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"dismiss-button"</span><span class="kw">&gt;</span>Dismiss<span class="kw">&lt;/a&gt;</span>
//!     <span class="kw">&lt;div</span> class=<span class="string">"blank-space"</span><span class="kw">&gt;&lt;/div&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"try-button"</span>&gt;Try it here!<span class="kw">&lt;/a&gt;</span>
//!   <span class="kw">&lt;/div&gt;</span>
//! <span class="kw">&lt;/div&gt;</span>
//! </code></pre></div>
//!
//! However styling this way is pretty boring because you need to think of good class names and
//! you have to repeatedly switch between several files, it could be better. Utility-first CSS
//! frameworks takes a new approach by using minimal and pre-defined class names directly linked to
//! its CSS rule content. The CSS file will then be generated
//! [On-demand](https://antfu.me/posts/reimagine-atomic-css#on-demand-way) allowing the classes
//! to be very flexible and customizable. This approach lets you quickly prototype visual HTML
//! elements and encourages you to turn them later into components using your favorite web framework.
//! It also makes building a responsive website easier and forces it to be closer to your design
//! system:
//
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">&lt;div</span> class=<span class="string">"w-128 text-md shadow-[1px_1px_10px_2px_#e5e7eb] rounded-xl"</span><span class="kw">&gt;
//!   &lt;div</span> class=<span class="string">"p-3 flex items-center"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;div</span> class=<span class="string">"bg-blue-500 rounded-full w-5 h-5 mr-3"</span><span class="kw">&gt;&lt;/div&gt;</span>
//!     A new Javascript library was released!
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"p-6 pt-4"</span><span class="kw">&gt;</span>
//!     The library <span class="kw">&lt;code&gt;</span>react<span class="kw">&lt;/code&gt;</span> was just released, did you know it?
//!     It is <span class="kw">&lt;i&gt;</span>a JavaScript library for creating user interfaces<span class="kw">&lt;/i&gt;</span>.
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"flex"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"p-3 text-rose-600"</span><span class="kw">&gt;</span>Dismiss<span class="kw">&lt;/a&gt;</span>
//!     <span class="kw">&lt;div</span> class=<span class="string">"flex-1"</span><span class="kw">&gt;&lt;/div&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"p-3 bg-blue-600 text-white rounded-br-xl rounded-tl-xl shadow shadow-blue-600"</span>&gt;Try it here!<span class="kw">&lt;/a&gt;</span>
//!   <span class="kw">&lt;/div&gt;</span>
//! <span class="kw">&lt;/div&gt;</span>
//! </code></pre></div>
//!
//! There is already a lot of utility-first frameworks like [Tailwind
//! CSS](https://tailwindcss.com), [Windi CSS](https://windicss.org), [Twind](https://twind.dev)
//! and [Uno CSS](https://uno.antfu.me), but `encre` is unique because it is written in Rust and
//! uses a new architecture, making it **the fastest utility-first framework**.
//!
//! ## Getting started
//!
//! Add `encre-css` to your `Cargo.toml`:
//!
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[dependencies]</span>
//! encre-css = { git = <span class="string">"https://gitlab.com/encre-css/encre-css.git"</span>, tag = <span class="string">"v0.4.0"</span> }</code></pre></div>
//!
//! Generating styles takes three steps:
//! - First, you need to _configure_ the main [`EncreGenerator`] structure
//! either by manually making a [`Config`] structure and calling
//! [`EncreGenerator::from_config`] or by reading a [TOML](https://toml.io) file using
//! [`EncreGenerator::new`];
//! - Then, you need to _scan content_ to extract and collect all useful atomic classes using
//! [`EncreGenerator::scan`] or [`EncreGenerator::add_selector`] to manually add **a
//! single** previously scanned selector or [`EncreGenerator::add_selectors`] to manually add
//! **several** previously scanned selectors;
//! - Finally, you need to _generate the styles_ using [`EncreGenerator::generate`].
//!
//! ### Example
//!
//! ```rust
//! use encre_css::{EncreGenerator, Config};
//!
//! let mut generator = EncreGenerator::from_config(Config::default());
//! generator.scan(r#"<p class="w-auto bg-red-200 rounded-md">Hello world!</p>"#);
//!
//! assert!(generator.generate().expect("failed to generate the CSS").contains(r#"
//! .w-auto {
//!   width: auto;
//! }
//!
//! .rounded-md {
//!   border-radius: 0.375rem;
//! }
//!
//! .bg-red-200 {
//!   --en-bg-opacity: 1;
//!   background-color: rgb(254 202 202 / var(--en-bg-opacity));
//! }"#));
//! ```
//!
//! ### What to do next
//!
//! - The documentation about all utility classes (also named plugins) is [here](crate::plugins).
//! - The documentation about the composition of a class (also named selector) is [here](crate::selector).
//!
//! ## Cargo features
//!
//! - `rayon`: enables [rayon](https://docs.rs/rayon/latest/rayon) parallel iterators
//!
//! ## Command line interface
//!
//! A command line interface is also available. Install it using:
//!
//! ```bash
//! cargo install --git https://gitlab.com/encre-css/encre-css.git
//! ```
//!
//! Then run `encre --help` for instructions on how to use it.
#![doc(html_logo_url = "http://gitlab.com/encre-css/encre-css/raw/main/.assets/logo.svg")]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    rustdoc::private_doc_tests,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    clippy::unnecessary_wraps,
    clippy::too_many_lines,
    clippy::string_to_string,
    clippy::explicit_iter_loop,
    clippy::unnecessary_cast,
    clippy::missing_errors_doc,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::non_ascii_literal,
    clippy::dbg_macro,
    clippy::map_err_ignore,
    clippy::use_debug,
    clippy::map_err_ignore,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo
)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

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
pub use error::{Error, Result};
pub use extractor::Extractor;
pub use generator::EncreGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::DarkMode, selector::Selector};

    use pretty_assertions::assert_eq;
    use std::{collections::BTreeSet, fs};

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

.marker\:selection\:hover\:bg-green-200:hover *::selection, .marker\:selection\:hover\:bg-green-200:hover::selection *::marker, .marker\:selection\:hover\:bg-green-200:hover *::selection, .marker\:selection\:hover\:bg-green-200:hover::selection::marker {{
  --en-bg-opacity: 1;
  background-color: rgb(187 247 208 / var(--en-bg-opacity));
}}

@media (min-width: 768px) {{
  .md\:focus\:selection\:bg-blue-100 *::selection, .md\:focus\:selection\:bg-blue-100::selection:focus {{
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
        config.theme.dark_mode = DarkMode::new_class(".dark");

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
        let mut config = Config::default();
        config.theme.colors.add("rosa-500", "#e5186a");
        config.theme.screens.add("3xl", "1600px");

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
        config.theme.colors.add("rosa-500", "#e5186a");
        config.theme.colors.add("yellow-400", "#ffef0e");
        config.theme.screens.add("lg", "2000px");
        config.theme.screens.add("3xl", "1600px");
        config.theme.dark_mode = DarkMode::new_class(".dark");

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
