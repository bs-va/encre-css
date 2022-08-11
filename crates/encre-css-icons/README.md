# encre-css-icons

> A plugin that provides a set of classes to use icons from [Iconify](https://iconify.design)
> based on the [`@unocss/preset-icons`](https://github.com/unocss/unocss/tree/main/packages/preset-icons)
> NodeJS package.

### Getting started

To integrate `encre-css-icons` with `encre-css`, add it in your `Cargo.toml`:

```toml
[dependencies]
encre-css-icons = { git = "https://gitlab.com/encre-org/encre-css.git", tag = "v0.6.0" }
```

Then, call the `register` function with a mutable reference to a `Config`
structure and some parameters (all parameters are optional, you can use `None`
to use the default value):

```rust
use encre_css::{Config, EncreGenerator};

let mut config = Config::from_file("encre-css.toml");
// Or let mut config = Config::default();

encre_css_icons::register(Some("i-"), None, Some(1.2), &mut config);
// First parameter (Option<&str>): a prefix applied to all icons (default is "")
// Second parameter (Option<&str>): a custom CDN used to fetch icons (default is "https://esm.sh")
// Third parameter (Option<f32>): the scale of icons (default is 1)

let mut generator = EncreGenerator::from_config(config);
generator.scan(r#"<div class="i-mdi-alarm"><span class="i-fa-solid-home"></span><span class="i-openmoji-bicycle"></span></div>"#);
// The convention is <prefix><collection>-<icon>

let css = generator.generate();
// Do something with the CSS
```

Please note that, in order to get the SVG definition of icons, this crate will make
requests to a (of course configurable) third-party CDN and will cache them in
the system's configured cache directory (`$XDG_CACHE_HOME` or `$HOME/.cache` on
GNU/Linux, `{FOLDERID_LocalAppData}` on Windows, `$HOME/Library/Caches` on MacOS),
in a directory named `encre-css-icons-cache`.

If searching the collection files takes too long (even if they are also cached
in memory), it is recommended to optimize the `encre-css-icons` crate even in
development, by adding the following in your `Cargo.toml` file:

```toml
[profile.dev.package.encre-css-icons]
opt-level = 3
```

Note that this plugin **does not support WebAssembly**.
