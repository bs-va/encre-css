Utilities for controlling gutters between grid and flexbox items.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>gap-<i>&lt;float&gt;</i></td><td>gap: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>gap-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>gap: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>gap-px</td><td>gap: 1px;</td></tr>
    <tr><td>gap-x-<i>&lt;float&gt;</i></td><td>column-gap: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>gap-x-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>column-gap: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>gap-x-px</td><td>column-gap: 1px;</td></tr>
    <tr><td>gap-y-<i>&lt;float&gt;</i></td><td>row-gap: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>gap-y-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>row-gap: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>gap-y-px</td><td>row-gap: 1px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Gap values don't follow Tailwind's philosophy of restraining possible values and all spacing values
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `gap-[0.5rem]`.

[Tailwind reference](https://tailwindcss.com/docs/gap)
