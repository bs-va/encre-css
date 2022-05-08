# EncreCSS

> A TailwindCSS-compatible CSS generation library written in Rust

### Getting started

Add `encre_css` to your `Cargo.toml`:

```toml
[dependencies]
encre_css = { git = "https://gitlab.com/encre-css/encre-css" }
```

Then, use the `EncreGenerator` structure to generate CSS styles:

```rust
use encre_css::EncreGenerator;

let mut generator = EncreGenerator::new();
generator.scan_content(r#"class="bg-red-500""#);

let css_styles = generator.generate();

// Do something with the styles
```
