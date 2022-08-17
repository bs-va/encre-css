Utilities for controlling the thickness of text decorations.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>underline-offset-auto</td><td>text-underline-offset: auto;</td></tr>
    <tr><td>underline-offset-<i>&lt;integer&gt;</i></td><td>text-underline-offset: <i>&lt;integer&gt;</i>px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Offset values don't follow Tailwind's philosophy of limiting possible values and all numbers
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `underline-offset-[11%]`.

[Tailwind reference](https://tailwindcss.com/docs/text-underline-offset)
