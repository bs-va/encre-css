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

### Tailwind compatibility

Family arbitrary values are **not** compatible with Tailwind's ones because
quotes are not needed in properties containing spaces. For example, the Tailwind
class `font-['Open_Sans']` will become `font-[Open_Sans]`.

### Arbitrary values

Any property **not starting with a number** is allowed as arbitrary value.
For example, `font-[Roboto,Open_Sans,sans-serif]`.

[Tailwind reference](https://tailwindcss.com/docs/font-family)
