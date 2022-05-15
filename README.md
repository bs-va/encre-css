# encre-css (french word for `ink`)

> A TailwindCSS-compatible CSS generation library written in Rust

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
generator.scan_content(r#"class="bg-red-500""#);

let css_styles = generator.generate();

// Do something with the styles
```

### License

`encre-css` is published under the [MIT license](https://gitlab.com/encre-css/encre-css/-/blob/main/LICENSE).
