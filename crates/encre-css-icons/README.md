# encre-css-icons

> A plugin that provides a set of classes to use icons from [Iconify](https://iconify.design)
> based on the [`@unocss/preset-icons`](https://github.com/unocss/unocss/tree/main/packages/preset-icons)
> NodeJS package.

### Features

- A **lot** of icon sets (you can use [Icônes](https://icones.netlify.app) to search
  among them)
- Pure CSS icons
- Icons follow the text size
- Icons follow the text color
- Support colorful and monochrome icons
- No request is issued client-side when not using WebAssembly
- Supports WebAssembly (SVG icons will be directly fetch client-side)

### Getting started

To integrate `encre-css-icons` with `encre-css`, add it in your `Cargo.toml`:

```toml
[dependencies]
encre-css-icons = "0.1.4"
```

Then, call the `register` function with a mutable reference to a `Config`
structure and some parameters (all parameters are optional, you can use `None`
to use the default value):

```rust
use encre_css::Config;

let mut config = Config::from_file("encre-css.toml")?;
// Or let mut config = Config::default();

encre_css_icons::register(&mut config);

// The convention is <prefix><collection>-<icon>
let _css = encre_css::generate(
    [r#"<h1 class="text-xl text-gray-600">Hello <span class="subway-world-1"></span>!</h1><div class="mdi-alarm block"></div><span class="fa-solid-home"></span><span class="openmoji-automobile hover:openmoji-autonomous-car"></span>"#],
    &config,
);
// Do something with the CSS
```

### Configuration

This plugin has some configuration options, to set them simply add an extra field `icons` in
the configuration with the fields you want to change. If configuring using Rust, the extra
configuration must be added **before** registering the plugin. For example in TOML:

```toml
[extra.icons]
prefix = "i-"
custom-cdn = "https://cdn.skypack.dev"
scale = 1.2
```

Or in Rust:

```rust
use encre_css::{Config, toml};

let mut config = Config::default();
// Or let mut config = Config::from_file("encre-css.toml")?;

config.extra.add("icons", toml! {
    prefix = "i-"
    custom-cdn = "https://cdn.skypack.dev"
    scale = 1.2
});

// Then register the plugin
encre_css_icons::register(&mut config);
```

Configuration fields:

- `prefix` (default: `""`): a static prefix added to all icons
- `custom-cdn` (default: `https://esm.sh`): the CDN used to get the SVG definition of icons (see the section below)
- `scale` (default: `1.0`): the scale of icons used to change their size

### Network requests and caching

Please note that, in order to get the SVG definition of icons, this crate will make
requests to a (of course configurable) third-party CDN (the default CDN is
`https://esm.sh`) and will cache them in the system's configured cache directory
(`$XDG_CACHE_HOME` or `$HOME/.cache` on GNU/Linux, `{FOLDERID_LocalAppData}` on
Windows, `$HOME/Library/Caches` on macOS), in a directory named
`encre-css-icons-cache`.

### Various tips and tricks

If searching the collection files takes too long (even if they are also cached
in memory), it is recommended to optimize the `encre-css-icons` crate even in
development, by adding the following in your `Cargo.toml` file:

```toml
[profile.dev.package.encre-css-icons]
opt-level = 3
```

By default, each icon has the `display: inline-block;` CSS property applied
(even on `div` elements) otherwise they would be unsized when used in `span`
elements. If you need to turn them into block elements, you can use the `block`
utility class on each icon, e.g. `<div class="fa-pencil block"></div>`.

### License

The code itself is under the [MIT License](../../LICENSE).
The collections of icons are under various licenses, see
[collections.md](https://github.com/iconify/icon-sets/blob/master/collections.md)
for a list of collections and their licenses.
