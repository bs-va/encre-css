<div align="center">
  <img src=".assets/logo.svg" />
  <h1>encre-css</h1>
  <p>A TailwindCSS-compatible CSS generation library written in Rust</p>

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

### Getting started

Add `encre-css` to your `Cargo.toml`:

```toml
[dependencies]
encre-css = { git = "https://gitlab.com/encre-css/encre-css.git" }
```

Then, use the `EncreGenerator` structure to generate CSS styles:

```rust
use encre_css::EncreGenerator;

let mut generator = EncreGenerator::new();
generator.scan_raw(r#"class="bg-red-500""#);

let css_styles = generator.generate();

// Do something with the styles
```

### About the name

`encre` means `ink` in French.

### License

`encre-css` is published under the [MIT license](https://gitlab.com/encre-css/encre-css/blob/main/LICENSE).
