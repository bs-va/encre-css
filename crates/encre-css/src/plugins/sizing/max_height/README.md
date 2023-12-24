Utilities for setting the maximum height of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>max-h-<i>&lt;float&gt;</i></td><td>max-height: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>max-h-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>max-height: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>max-h-px</td><td>max-height: 1px;</td></tr>
    <tr><td>max-h-none</td><td>max-height: none;</td></tr>
    <tr><td>min-h-full</td><td>min-height: 100%;</td></tr>
    <tr><td>min-h-auto</td><td>min-height: auto;</td></tr>
    <tr><td>min-h-screen</td><td>min-height: 100vh;</td></tr>
    <tr><td>min-h-min</td><td>min-height: min-content;</td></tr>
    <tr><td>min-h-max</td><td>min-height: max-content;</td></tr>
    <tr><td>min-h-fit</td><td>min-height: fit-content</td></tr>
    <tr><td>max-h-svh</td><td>max-height: 100svh;</td></tr>
    <tr><td>max-h-lvh</td><td>max-height: 100lvh;</td></tr>
    <tr><td>max-h-dvh</td><td>max-height: 100dvh;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Maximum height values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with
Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `max-h-[3rem]`.

[Tailwind reference](https://tailwindcss.com/docs/max-height)
