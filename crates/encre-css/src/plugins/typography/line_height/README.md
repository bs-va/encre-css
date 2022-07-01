Utilities for controlling the leading (line height) of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>leading-<i>&lt;float&gt;</i></td><td>line-height: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>leading-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>line-height: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>leading-px</td><td>line-height: 1px;</td></tr>
    <tr><td>leading-none</td><td>line-height: 1;</td></tr>
    <tr><td>leading-tight</td><td>line-height: 1.25;</td></tr>
    <tr><td>leading-snug</td><td>line-height: 1.375;</td></tr>
    <tr><td>leading-normal</td><td>line-height: 1.5;</td></tr>
    <tr><td>leading-relaxed</td><td>line-height: 1.625;</td></tr>
    <tr><td>leading-loose</td><td>line-height: 2;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Line height values don't follow Tailwind's philosophy of restraining possible values and all spacing values
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length), [`<number>`](crate::utils::value_matchers::is_matching_number) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `leading-[1.12rem]`.

[Tailwind reference](https://tailwindcss.com/docs/line-height)
