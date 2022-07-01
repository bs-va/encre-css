Utilities for controlling the style of an element's borders.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>border-solid</td><td>border-style: solid;</td></tr>
    <tr><td>border-dashed</td><td>border-style: dashed;</td></tr>
    <tr><td>border-dotted</td><td>border-style: dotted;</td></tr>
    <tr><td>border-double</td><td>border-style: double;</td></tr>
    <tr><td>border-hidden</td><td>border-style: hidden;</td></tr>
    <tr><td>border-none</td><td>border-style: none;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`line style`](crate::utils::value_matchers::is_matching_line_style) property is allowed as arbitrary value.
For example, `border-[dotted solid none dashed]`.

[Tailwind reference](https://tailwindcss.com/docs/border-style)
