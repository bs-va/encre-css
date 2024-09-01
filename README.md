<div align="center">
  <img src="https://gitlab.com/encre-org/encre-css/raw/main/.assets/logo.png" width="100" />
  <h1>encre-css</h1>
  <p>A TailwindCSS-compatible utility-first CSS generation library written in Rust</p>

  <a href="https://gitlab.com/encre-org/encre-css/blob/main/LICENSE">
    <img alt="MIT License" src="https://img.shields.io/badge/license-MIT-success" />
  </a>

  <a href="https://gitlab.com/encre-org/encre-css/-/pipelines">
    <img alt="Pipeline status" src="https://gitlab.com/encre-org/encre-css/badges/main/pipeline.svg" />
  </a>

  <a href="https://gitlab.com/encre-org/encre-css/-/pipelines">
    <img alt="Coverage report" src="https://gitlab.com/encre-org/encre-css/badges/main/coverage.svg" />
  </a>

  <a href="https://deps.rs/repo/gitlab/encre-org/encre-css">
    <img alt="Dependency status" src="https://deps.rs/repo/gitlab/encre-org/encre-css/status.svg" />
  </a>

  <a href="https://crates.io/crates/encre-css">
    <img alt="Published on crates.io" src="https://img.shields.io/crates/v/encre-css" />
  </a>

  <a href="https://docs.rs/encre-css">
    <img alt="Documentation on docs.rs" src="https://img.shields.io/docsrs/encre-css" />
  </a>

  <br>

  <a href="https://gitlab.com/encre-org/encre-css">
    <img alt="Number of files" src="https://tokei.ekzhang.com/b1/gitlab/encre-org/encre-css?category=files" />
  </a>

  <a href="https://gitlab.com/encre-org/encre-css">
    <img alt="Number of lines of code" src="https://tokei.ekzhang.com/b1/gitlab/encre-org/encre-css?category=code" />
  </a>

  <a href="https://gitlab.com/encre-org/encre-css">
    <img alt="Number of lines of comments" src="https://tokei.ekzhang.com/b1/gitlab/encre-org/encre-css?category=comments" />
  </a>

  <a href="https://gitlab.com/encre-org/encre-css">
    <img alt="Total number of lines" src="https://tokei.ekzhang.com/b1/gitlab/encre-org/encre-css" />
  </a>
</div>

## Table of contents

- [A brief introduction to utility-first CSS frameworks](#a-brief-introduction-to-utility-first-css-frameworks)
- [Getting started](#getting-started)
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
    A new Javascript library has been released!
  </div>
  <div class="notification-body">
    The library <code>react</code> has just been released, did you know it?
    It is <i>a JavaScript library for creating user interfaces</i>.
  </div>
  <div class="notification-footer">
    <a href="#" class="dismiss-button">Dismiss</a>
    <a href="#" class="try-button">Try it here!</a>
  </div>
</div>
```

However styling this way is pretty boring because you need to think about good class names and
to repeatedly switch between several files, it could be better. Utility-first CSS frameworks
takes a new approach by using minimal and pre-defined class names directly linked to its CSS
rule content. The CSS file will then be generated
[on-demand](https://antfu.me/posts/reimagine-atomic-css#on-demand-way) allowing the classes
to be very flexible and customizable. This approach lets you quickly prototype visual HTML
elements and encourages you to turn them into components using your favorite web framework. It
also makes building a responsive website easier and forces it to be closer to your design
system (if you have one):

```html
<div class="w-128 text-md shadow-[1px_1px_10px_2px_#e5e7eb] rounded-xl">
  <div class="p-3 flex items-center">
    <div class="bg-blue-500 rounded-full w-5 h-5 mr-3"></div>
    A new Javascript library has been released!
  </div>
  <div class="p-6 pt-4">
    The library <code>react</code> has just been released, did you know it?
    It is <i>a JavaScript library for creating user interfaces</i>.
  </div>
  <div class="flex justify-between">
    <a href="#" class="p-3 text-rose-600">Dismiss</a>
    <a href="#" class="p-3 bg-blue-600 text-white rounded-br-xl rounded-tl-xl shadow shadow-blue-600">Try it here!</a>
  </div>
</div>
```

There is already a lot of utility-first frameworks like [Tailwind
CSS](https://tailwindcss.com), [Windi CSS](https://windicss.org), [Twind](https://twind.dev)
and [Uno CSS](https://uno.antfu.me), but `encre-css` is unique because it is written in Rust and
uses a new architecture, making it **the fastest utility-first framework** (according to the
benchmark [here](https://gitlab.com/encre-org/encre-css-bench) based on
[Uno CSS' benchmark](https://github.com/unocss/unocss/tree/main/bench)).

## Getting started

Add `encre-css` to your `Cargo.toml`:

```toml
[dependencies]
encre-css = "0.14.1"
```

Generating styles takes two steps:
- You need to _configure_ the CSS generation by making a `Config` structure.
It can be created by reading a [TOML](https://toml.io) file using
`Config::from_file` or by using the default values with `Config::default`;
- Then, you need to _generate the styles_ based on some sources using the `generate`
function. This function will scan the content of the sources, extract atomic classes and
generate the style needed for each class.

### Example

```rust
use encre_css::Config;

let config = Config::default();
let css = encre_css::generate(
    [r#"<p class="w-auto bg-red-200 rounded-md">Hello world!</p>"#],
    &config,
);

assert!(css.expect("failed to generate the CSS").ends_with(r#"
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
cargo install encre-css-cli
```

Then run `encrecss --help` for instructions on how to use it.

## Plugins

`encre-css` was built with modularity in mind and it is possible to write or use
custom plugins. [Learn more](https://docs.rs/encre-css/latest/encre_css/plugins)

## About the name

`encre` means `ink` in French.

## License

`encre-css` is published under the [MIT license](https://gitlab.com/encre-org/encre-css/blob/main/LICENSE).
