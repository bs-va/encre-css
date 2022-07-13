<div align="center">
  <img src=".assets/logo.svg" />
  <h1>encre-css</h1>
  <p>A TailwindCSS-compatible utility-first CSS generation library written in Rust</p>

  <a href="https://gitlab.com/encre-css/encre-css/blob/main/LICENSE">
    <img alt="MIT License" src="https://img.shields.io/badge/license-MIT-success" />
  </a>

  <a href="https://gitlab.com/encre-css/encre-css/-/pipelines">
    <img alt="Pipeline status" src="https://gitlab.com/encre-css/encre-css/badges/main/pipeline.svg" />
  </a>

  <a href="https://deps.rs/repo/gitlab/encre-css/encre-css">
    <img alt="Dependency status" src="https://deps.rs/repo/gitlab/encre-css/encre-css/status.svg" />
  </a>

  <a href="https://encre-css.gitlab.io/encre-css/encre_css">
    <img alt="Documentation on gitlab.io" src="https://img.shields.io/static/v1?label=docs&message=gitlab.io&color=blue" />
  </a>

  <br>

  <a href="https://gitlab.com/encre-css/encre-css">
    <img alt="Number of files" src="https://tokei.rs/b1/gitlab/encre-css/encre-css?category=files" />
  </a>

  <a href="https://gitlab.com/encre-css/encre-css">
    <img alt="Number of lines of code" src="https://tokei.rs/b1/gitlab/encre-css/encre-css?category=code" />
  </a>

  <a href="https://gitlab.com/encre-css/encre-css">
    <img alt="Number of lines of comments" src="https://tokei.rs/b1/gitlab/encre-css/encre-css?category=comments" />
  </a>

  <a href="https://gitlab.com/encre-css/encre-css">
    <img alt="Total number of lines" src="https://tokei.rs/b1/gitlab/encre-css/encre-css" />
  </a>
</div>

## Table of contents

- [A brief introduction to utility-first CSS frameworks](#a-brief-introduction-to-utility-first-css-frameworks)
- [Getting started](#getting-started)
- [Cargo features](#cargo-features)
- [Command line interface](#command-line-interface)
- [About the name](#about-the-name)
- [License](#license)

## A brief introduction to utility-first CSS frameworks

Traditionally, whenever you need to style something on the web, you write CSS in a dedicated
file and apply the rules using classes in your HTML, like that:

```html
<div class="notification">
  <div class="notification-header">
    <div class="app-icon"></div>
    A new Javascript library was released!
  </div>
  <div class="notification-body">
    The library <code>react</code> was just released, did you know it?
    It is <i>a JavaScript library for creating user interfaces</i>.
  </div>
  <div class="notification-footer">
    <a href="#" class="dismiss-button">Dismiss</a>
    <div class="blank-space"></div>
    <a href="#" class="try-button">Try it here!</a>
  </div>
</div>
```

However styling this way is pretty boring because you need to think about good class names and
to repeatedly switch between several files, it could be better. Utility-first CSS frameworks
takes a new approach by using minimal and pre-defined class names directly linked to its CSS
rule content. The CSS file will then be generated
[On-demand](https://antfu.me/posts/reimagine-atomic-css#on-demand-way) allowing the classes
to be very flexible and customizable. This approach lets you quickly prototype visual HTML
elements and encourages you to turn them into components using your favorite web framework. It
also makes building a responsive website easier and forces it to be closer to your design
system (if you have one):

```html
<div class="w-128 text-md shadow-[1px_1px_10px_2px_#e5e7eb] rounded-xl">
  <div class="p-3 flex items-center">
    <div class="bg-blue-500 rounded-full w-5 h-5 mr-3"></div>
    A new Javascript library was released!
  </div>
  <div class="p-6 pt-4">
    The library <code>react</code> was just released, did you know it?
    It is <i>a JavaScript library for creating user interfaces</i>.
  </div>
  <div class="flex">
    <a href="#" class="p-3 text-rose-600">Dismiss</a>
    <div class="flex-1"></div>
    <a href="#" class="p-3 bg-blue-600 text-white rounded-br-xl rounded-tl-xl shadow shadow-blue-600">Try it here!</a>
  </div>
</div>
```

There is already a lot of utility-first frameworks like [Tailwind
CSS](https://tailwindcss.com), [Windi CSS](https://windicss.org), [Twind](https://twind.dev)
and [Uno CSS](https://uno.antfu.me), but `encre-css` is unique because it is written in Rust and
uses a new architecture, making it **the fastest utility-first framework**.

## Getting started

Add `encre-css` to your `Cargo.toml`:

```toml
[dependencies]
encre-css = { git = "https://gitlab.com/encre-css/encre-css.git", tag = "v0.3.0" }
```

Generating styles takes three steps:
- First, you need to _configure_ the main `EncreGenerator` structure
either by manually making a `Config` structure and calling
`EncreGenerator::from_config` or by reading a [TOML](https://toml.io) file using
`EncreGenerator::new`;
- Then, you need to _scan content_ to extract and collect all useful atomic classes using
`EncreGenerator::scan` or `EncreGenerator::add_selector` to manually add **a
single** previously scanned selector or `EncreGenerator::add_selectors` to manually add
**several** previously scanned selectors;
- Finally, you need to _generate the styles_ using `EncreGenerator::generate`.

### Example

```rust
use encre_css::{EncreGenerator, Config};

let mut generator = EncreGenerator::from_config(Config::default());
generator.scan(r#"<p class="w-auto bg-red-200 rounded-md">Hello world!</p>"#);

assert!(generator.generate().expect("failed to generate the CSS").contains(r#"
.w-auto {
  width: auto;
}

.rounded-md {
  border-radius: 0.375rem;
}

.bg-red-200 {
  --en-bg-opacity: 1;
  background-color: rgb(254 202 202 / var(--en-bg-opacity));
}"#));
```

## Command line interface

A command line interface is also available. Install it using:

```bash
cargo install --git https://gitlab.com/encre-css/encre-css.git
```

Then run `encre --help` for instructions on how to use it.

## About the name

`encre` means `ink` in French.

## License

`encre-css` is published under the [MIT license](https://gitlab.com/encre-css/encre-css/blob/main/LICENSE).
