Utilities for controlling the font family of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>font-sans</td><td>font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";</td></tr>
    <tr><td>font-serif</td><td>font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif;</td></tr>
    <tr><td>font-mono</td><td>font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any property **not starting with a number** is allowed as arbitrary value.
For example, `font-[Roboto,&#39;Open_Sans&#39;,sans-serif]`.

Because the default scanner splits by some special characters, you **must** use the following escape codes in arbitrary values:

- `&#34;` for `"`
- `&#39;` for `'`
- `&#40;` for `(`
- `&#41;` for `)`
- `&#91;` for `[`
- `&#92;` for `\`
- `&#93;` for `]`
- `&#95;` for `_` (because by default `_` is replaced by a space in arbitrary values, so `&#95;` prevents this behavior)
- `&#96;` for `` ` ``

[Tailwind reference](https://tailwindcss.com/docs/font-family)
