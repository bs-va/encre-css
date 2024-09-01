# Changelog

All notable changes to this project will be documented in this file.

## [0.14.1] - 2024-09-01

### Bug Fixes

- Shadows using the full syntax not working

## [0.14.0] - 2024-08-25

### Bug Fixes

- Support negative `full` values for the rules that accepts them
- Bad selector ordering resulting in selectors being ignored
- Avoid panicking when encountering a `shadow` without any modifier
- Avoid panicking when encountering UTF-8 characters in the `font-family` utility
- Avoid panicking when a bracket is not closed in a `url` arbitrary value
- Strictly parse arbitrary modifiers

### Documentation

- Fix badly closed HTML tag

### Features

- Support variant groups without any namespace or modifier
- Support watching another directory following the extra input argument

### Miscellaneous Tasks

- Add fuzzing

### Styling

- Fix all Clippy warnings

### Testing

- Set default order on all rules because it is not checked in tests

## [0.13.0] - 2024-07-09

### Bug Fixes

- Prevent cycles during shortcut expansion by defining a maximum expansion depth

### Documentation

- Update crate documentation
- Fix typo in `line_clamp` documentation
- Fix typo
- Add documentation for ARIA state modifiers

### Features

- Add support for `caption_side`
- Add support for `whitespace-break-spaces`

### Miscellaneous Tasks

- Update copyright year

### Ci

- Fix failing test due to source change in an Iconify icon

## [0.12.0] - 2024-04-11

### Documentation

- Fix a typo

### Features

- Add support for nested shortcuts

## [0.11.0] - 2023-12-26

### Bug Fixes

- Support WebAssembly
- Use `ureq` instead of `reqwest`

### Documentation

- Add note about the order of configuration and registration of the plugin
- Better documentation for variants
- Fix some inconsistencies in documentation

### Features

- Add support for dynamic viewport units
- Support the `text-wrap` CSS property

### Miscellaneous Tasks

- Update dependencies
- Add `resolver = "2"` to appease Cargo
- Fix `encre-css-icons` pipeline

### Build

- Update all crates

## [0.10.1] - 2023-07-01

### Bug Fixes

- Remove dashed requirement for arbitrary values because it breaks arbitrary variants

### Documentation

- Use a more rusty way of importing generate

### Miscellaneous Tasks

- Prepare `encre-css-icons` and `encre-css-typography` to be published on crates.io

### Ci

- Use Rust stable
- Use musl target for grcov because of a glibc version conflict

## [0.10.0] - 2023-04-18

### Bug Fixes

- Arbitrary css property containing spaces

### Documentation

- Fix some typos

### Refactor

- [**breaking**] Replace the `EncreGenerator` structure with a single `generate` function

### Styling

- Run rustfmt

### Build

- Update dependencies

## [0.9.0] - 2023-02-24

### Bug Fixes

- `ring` plugin
- Try to read only files

### Features

- Support all logical properties (e.g. `border-inline-start-color`)

### Performance

- Improve performance of class name generation, use `phf` to generate hashed structures of colors and variants
- Reserve Vec with capacity when parsing variants
- Optimize scanning

### Ci

- Add grcov-based test coverage

## [0.8.1] - 2023-02-21

### Bug Fixes

- [**breaking**] Add a Serde tag to each configuration enum + place the safelist first in the configuration
- `max_width` and `grid_auto_flow` plugins not working as they should
- Some inconsistencies in plugins
- Some arbitrary values not working with spaces
- Remove support for <line width> arbitrary values in the `divide-width` plugin

### Documentation

- Fix a typo
- Fix some verb tense mistakes
- Fix capitalization MacOS -> macOS
- Fix hints rendering on mobile
- Fix badly closed <tr>
- Fix capitalization of "on-demand"

### Features

- Support arbitrary values containing several `url()`

### Miscellaneous Tasks

- Update copyright year
- Fix Clippy warnings
- Update dependencies

### Refactor

- Avoid cloning variants when parsing the modifier
- `hex_to_rgb` returns `Option` instead of `Result` + add tests

### Testing

- Add more tests for the preflight
- Add tests for each plugin, fix numerous bugs or inconsistencies

## [0.8.0] - 2022-11-04

### Bug Fixes

- Avoid losing classes when sorting them + deduplicate variant groups
- Improve sorting of selectors
- Ignore newlines and whitespaces in the `check_selectors` function
- [**breaking**] Use the config as the first argument of the `register` function
- Fix Clippy warnings
- `from-` utility classes

### Documentation

- Fix some typos
- Use another instance of tokei
- Fix vocabulary mistake
- Improve documentation + add `display: inline-block;` to all icons by default
- Add keywork aliases to all plugins
- Handle errors when calling `Config::from_file`
- Update the links
- Improve the use of Flexbox in the notification footer example
- Explain arbitrary variants used to generated CSS at rules, add a link to a benchmark

### Features

- Implement `Serialize` for `Config`
- Support omitting the dash after the first modifier, run `cargo fmt`, fix clippy warnings
- Sort selectors in variant groups in `sort_selectors` (+ fix splitting when parenthesis are wrapped in brackets)
- [**breaking**] Define a `Buffer` structure handling the CSS generated and the indentation
- Support shortcuts
- Support the safelist configuration + inline some configuration functions
- [**breaking**] Add a configuration field `extra`, `encre-css-icons` now uses the `extra` field for configuration, improve documentation

### Miscellaneous Tasks

- Commit `Cargo.lock` and fix links in `Cargo.toml`
- Clean up documentation, update dependencies
- Remove TODOs about better matching arbitrary values
- Add a CHANGELOG generated with git-cliff
- Update dependencies
- Remove `rayon`
- Rename the `encre` binary to `encrecss` and update the documentation

### Performance

- [**breaking**] Improve indentation (~5% speedup)
- [**breaking**] Simplify the `Pattern` trait: its method `is_matching` takes a `char` instead of an `&str` (~17% speedup)
- Optimize if-else blocks of the `selector::parser::underscores_to_spaces` function (~5% speedup)
- [**breaking**] Use a static namespace: remove the `Plugin::namespace` method, add a `namespace` argument to the `Config::register_plugin` method (~20% speedup)

### Refactor

- Remove `ANIMATIONS_ALREADY_DEFINED` and `encre_css_typography::ALREADY_DEFINED` which were actually useless
- Add a prelude used to build new plugins
- [**breaking**] `EncreGenerator::generate` consumes the `EncreGenerator` structure
- Rewrite some plugins using the `matches!` macro + simplify some `match` blocks
- [**breaking**] New way of creating an `EncreGenerator` + improve documentation tests
- [**breaking**] Privatize the `selector::parser::{unescape, underscores_to_spaces}` functions

### Styling

- Run `cargo fmt`

## [0.7.0] - 2022-08-12

### Bug Fixes

- Watch `Chmod` events
- Support changing opacity for the `black` and `white` colors

### Documentation

- Fix link
- Update README

### Features

- Add a function to sort selectors
- Add `encre-css-icons` providing a plugin to easily use pure CSS icons
- [**breaking**] Add a `check_selectors` function used to get errors occurred when parsing selectors, selectors using unknown variant are now fully ignored

## [0.6.0] - 2022-08-03

### Bug Fixes

- Color parsing when the opacity suffix is empty
- Font family parsing when the arbitrary value is empty
- Typo in the `will-change` plugin
- `content-none` utility
- Don't ignore parenthesis in the default scanner
- Animation utility with arbitrary values
- Wrong CSS variables
- Manually implement Default for the `config::DarkMode` enum for compatibility reasons

### Documentation

- Include `Preflight::new_none` in the documentation
- Add documentation for the rest of the `VariantType` enum
- Update the readme

### Features

- Support the `line-clamp` utilities by default
- Support parent and peer variants
- Better sorting of variants
- [**breaking**] Allow creating custom plugins and variants

### Refactor

- [**breaking**] Return a `Vec` instead of a `BTreeSet` in the closure passed to a `Scanner`

### Testing

- Fix tests

### Revert

- "refactor!: return a `Vec` instead of a `BTreeSet` in the closure passed to a `Scanner`"

## [0.5.0] - 2022-07-13

### Bug Fixes

- Trim scanned selectors

### Documentation

- Fix formatting
- Fix color contrast

### Features

- Support the `::backdrop` variant
- Improve compatibility with the latest Tailwind

### Styling

- Run `cargo fmt`

## [0.4.0] - 2022-07-01

### Bug Fixes

- Negative values with variants
- Support TailwindCSS-compatible sorting of selectors + support latest Tailwind plugins + fix various bugs with plugins
- Extra input not watched
- `current` and `inherit` colors
- `transparent` color
- Extractor when the feature `rayon` is set

### Features

- Implement `EncreGenerator::add_selectors` for adding several selectors previously scanned
- Implement a configurable selector extractor
- Sort breakpoints in `ContainerPlugin`
- Support screen variants in `ContainerPlugin`
- Support wrapping class in variants
- [**breaking**] More universal values + document all items + some bug fixes

### Refactor

- Use context structures in `can_handle` and `handle`

## [0.3.0] - 2022-06-24

### Bug Fixes

- Fix typo in `border-left-width`
- Re-enable support for the WASM target
- Show real duration of commands
- Fix badly generated font families due to spaces

### Documentation

- Update the readme and add a logo

### Performance

- Check that a plugin can handle a selector in Selector::new, so bad selectors are not stored in memory + declare builtin plugins as static

### Refactor

- Better way of handling opacity in `default_colors`
- Make `wax` optional
- Make `rayon` optional
- Use `once_cell` instead of `lazy_static`
- [**breaking**] Optimize selector scanning, remove some dependencies

### Styling

- Run `cargo fmt`

### Ci

- Cache the cargo directory

## [0.2.0] - 2022-06-15

### Bug Fixes

- Rename prefix -> variant
- Fix binary glob pattern for simple paths
- Really support all border utilities
- Use `is_matching_auto` instead of `== "auto"`
- Add some tests for stacking variants, fix some bugs with them and add a better support for indentation
- Fix some bugs and support WASM targets
- Use new API
- Fix the example
- Use the `derive` feature of the `serde` crate instead of using `serde_derive`
- Use `absolute_size`, `relative_size` and `line_width` value matchers
- Fix color value matcher
- Fix shadow color matcher
- Fix color regex
- Reload only when a watched file is changed + handle configuration file changes
- Better support for the `calc` and `url` functions + remove some `.clone()`s + avoid `.replace`ing a lot in `gen_css_rule`
- Remove some `.to_string()` + fix bad indentation
- Fix Clippy warnings
- Bump wax to `0.5.0` + fix extra newlines in `space-x` and `space-y`

### Documentation

- Rename the project as `encre_css` and add a README
- Add a license and update the readme
- Fix a typo
- Fix a typo

### Features

- Full support for the `bg` namespace
- Full support for all layout utilities
- Full support for all filter utilities (+ huge performance improvements)
- Implement various utilities
- Display time in the cli, better performances, support more utilities
- Add a function for generating CSS from the content of a file
- Support more variants
- Support important and negative modifiers
- Support the `content` CSS property (+ run `cargo fmt`)
- Remove scope name in plugins, fix some todos, support all filter, SVG and table utilities (+ support the color `inherit` everywhere)
- Add a structure handling CSS generation (+ add more documentation)
- Support all alignment utilities
- Support all border, filter and spacing utilities properly
- Support all effect utilities
- Support all grid utilities
- Support all accessibility utilities
- Support all interactivity utilities
- Support all transform utilities
- Support all typography utilities
- Support all transition utilities
- Support stacking variants
- Support all values in the stroke utility
- Create a basic CLI interface using `clap`, start creating a basic configuration file using `toml` and `serde`, start defining errors using `thiserror`
- Add a file watcher to trigger rebuilds automatically
- Add a subcommand for quickly prototyping ideas
- Support changing the dark mode using a configuration option
- Use the config in each plugin, support overriding screen breakpoints and colors in the config
- Support extending and overriding configuration
- Support animations and fix backdrop-filter bugs
- Use hexadecimal colors in the configuration structure (+ fix color shorthand parsing)
- [**breaking**] New plugin internal API + fallible CSS generation

### Miscellaneous Tasks

- Initial commit
- Set up tracing and eyre

### Performance

- Huge performance improvements
- Ignore `class` and `className` selectors during scan
- Optimize filtering scanned selectors
- Add some benchmarks with Criterion
- Avoid copying custom css + better `if`s and `for`s sorting
- Use Option for variants, inline some functions, fix parsing of variants
- Use parallel iterators when possible + better sorting of selectors + align navigator prefixes with their non-prefixed version
- Use smol_str in selectors

### Refactor

- Use write! and a shared buffer for generating the CSS
- Clean up code
- Add a `Modifier` structure for handling negative values better (+ fix some TODOs)
- Use a workspace
- Rename `tw-` prefix to `en-`
- [**breaking**] Rename `scan_content` to `scan_raw` and `clear_scanned_selectors` to `reset`
- Separate EncreGenerator and Extractor
- Replace some `PathBuf`s with `AsRef<Path>`

### Styling

- Fix clippy warnings and run rustfmt
- Run `cargo fmt`

### Testing

- Add some tests
- Add tests for configuring screen breakpoints and colors
- Add tests for all value matchers
- Add a test for arbitrary values (+ fix background urls)
- Fix failing tests due to different sorting

### Ci

- Add Gitlab CI configuration file
- Cache cargo build artifacts
- Fix Gitlab CI configuration file
- Print rustc and cargo version

<!-- generated by git-cliff -->
