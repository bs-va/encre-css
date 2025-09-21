Utilities for controlling the content of the before and after pseudo-elements.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>content-none</td><td>content: none;</td></tr>
  </tbody>
</table>

### Note

The `content` utility must be used in combination with either the `before` or
the `after` variant to work.

### Arbitrary values

Any property is allowed as arbitrary value. It's the only way of setting the content.
For example, `before:content-[&#39;Hello_world!&#39;]` or `after:content-[url(foobar.png)]`.

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

[Tailwind reference](https://tailwindcss.com/docs/content)
