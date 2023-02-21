Utilities for setting the minimum height of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>min-h-<i>&lt;float&gt;</i></td><td>min-height: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>min-h-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>min-height: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>min-h-px</td><td>min-height: 1px;</td></tr>
    <tr><td>min-h-full</td><td>min-height: 100%;</td></tr>
    <tr><td>min-h-auto</td><td>min-height: auto;</td></tr>
    <tr><td>min-h-screen</td><td>min-height: 100vh;</td></tr>
    <tr><td>min-h-min</td><td>min-height: min-content;</td></tr>
    <tr><td>min-h-max</td><td>min-height: max-content;</td></tr>
    <tr><td>min-h-fit</td><td>min-height: fit-content</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Minimum height values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with
Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `min-h-[1.2em]`.

[Tailwind reference](https://tailwindcss.com/docs/min-height)
