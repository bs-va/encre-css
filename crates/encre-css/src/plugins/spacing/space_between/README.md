Utilities for controlling the space between child elements.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>space-x-<i>&lt;float&gt;</i></td><td>margin-inline: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>space-x-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-inline: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>space-x-px</td><td>margin-inline: 1px;</td></tr>
    <tr><td>space-x-auto</td><td>margin-inline: auto;</td></tr>
    <tr><td>space-y-<i>&lt;float&gt;</i></td><td>margin-block: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>space-y-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-block: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>space-y-px</td><td>margin-block: 1px;</td></tr>
    <tr><td>space-y-auto</td><td>margin-block: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Spacing between values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `space-x-[1rem]`.

### Negative values

This plugin supports negative values. For example, `-space-x-2` or `hover:-space-x-2`.

[Tailwind reference](https://tailwindcss.com/docs/space)
