Utilities for controlling the width of an element's borders.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>outline-<i>&lt;integer&gt;</i></td><td>outline-width: <i>&lt;integer&gt;</i>px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Outline width values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`line width`](crate::utils::value_matchers::is_matching_line_width) property is allowed as arbitrary value.
For example, `outline-[2em]`.

[Tailwind reference](https://tailwindcss.com/docs/outline-width)
