Utilities for styling the stroke width of SVG elements.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>stroke-<i>&lt;integer&gt;</i></td><td>stroke-width: <i>&lt;integer&gt;</i>px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Stroke width values don't follow Tailwind's philosophy of restraining possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `stroke-[1.8em]`.

[Tailwind reference](https://tailwindcss.com/docs/stroke-width)
