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
    <tr><td>max-h-screen</td><td>max-height: 100vh;</td></tr>
    <tr><td>max-h-none</td><td>max-height: none;</td></tr>
    <tr><td>max-h-xs</td><td>max-height: 20rem;</td></tr>
    <tr><td>max-h-sm</td><td>max-height: 24rem;</td></tr>
    <tr><td>max-h-md</td><td>max-height: 28rem;</td></tr>
    <tr><td>max-h-lg</td><td>max-height: 32rem;</td></tr>
    <tr><td>max-h-xl</td><td>max-height: 36rem;</td></tr>
    <tr><td>max-h-2xl</td><td>max-height: 42rem;</td></tr>
    <tr><td>max-h-3xl</td><td>max-height: 48rem;</td></tr>
    <tr><td>max-h-4xl</td><td>max-height: 56rem;</td></tr>
    <tr><td>max-h-5xl</td><td>max-height: 64rem;</td></tr>
    <tr><td>max-h-6xl</td><td>max-height: 72rem;</td></tr>
    <tr><td>max-h-7xl</td><td>max-height: 80rem;</td></tr>
    <tr><td>max-h-full</td><td>max-height: 100%;</td></tr>
    <tr><td>max-h-min</td><td>max-height: min-content;</td></tr>
    <tr><td>max-h-max</td><td>max-height: max-content;</td></tr>
    <tr><td>max-h-fit</td><td>max-height: fit-content;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Maximum height values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported (`screen` also). They are however perfectly compatible with
Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `max-h-[3rem]`.

[Tailwind reference](https://tailwindcss.com/docs/max-height)
