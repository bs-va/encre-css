# encre-css-typography

> A plugin that provides a set of `prose` classes you can use to add beautiful typographic defaults to any vanilla HTML you don't control, like HTML rendered from Markdown, or pulled from a CMS.

### Getting started

To integrate `encre-css-typography` with `encre-css`, add it in your `Cargo.toml`:

```toml
[dependencies]
encre-css-typography = "0.10.0"
```

Then, call the `register` function with a mutable reference to a `Config` structure:

```rust
use encre_css::{Config, generate};

let mut config = Config::from_file("encre-css.toml")?;
// Or let mut config = Config::default();
encre_css_typography::register(&mut config);

let _css = generate([r#"<div class="prose prose-headings:text-blue-500 prose-slate lg:prose-lg dark:prose-invert"></div>"#], &config);
// Do something with the CSS
```
