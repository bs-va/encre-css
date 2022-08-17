Utilities for setting the minimum width of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>min-w-<i>&lt;float&gt;</i></td><td>min-width: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>min-w-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>min-width: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>min-w-px</td><td>min-width: 1px;</td></tr>
    <tr><td>min-w-full</td><td>min-width: 100%;</td></tr>
    <tr><td>min-w-screen</td><td>min-width: 100vw;</td></tr>
    <tr><td>min-w-min</td><td>min-width: min-content;</td></tr>
    <tr><td>min-w-max</td><td>min-width: max-content;</td></tr>
    <tr><td>min-w-fit</td><td>min-width: fit-content</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Minimum width values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported (`screen` also). They are however perfectly compatible with
Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `min-w-[4.2in]`.

[Tailwind reference](https://tailwindcss.com/docs/min-width)
