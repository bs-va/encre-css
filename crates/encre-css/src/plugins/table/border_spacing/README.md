Utilities for controlling the spacing between table borders.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>border-spacing-<i>&lt;float&gt;</i></td><td>border-spacing: <i>&lt;float / 4&gt;</i>rem <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>border-spacing-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>border-spacing: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)% (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>border-spacing-px</td><td>border-spacing: 1px 1px;</td></tr>
    <tr><td>border-spacing-x-<i>&lt;float&gt;</i></td><td>border-spacing: <i>&lt;float / 4&gt;</i>rem var(--en-border-spacing-y);</td></tr>
    <tr><td>border-spacing-x-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>border-spacing: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)% var(--en-border-spacing-y);</td></tr>
    <tr><td>border-spacing-x-px</td><td>border-spacing: 1px var(--en-border-spacing-y);</td></tr>
    <tr><td>border-spacing-y-<i>&lt;float&gt;</i></td><td>border-spacing: var(--en-border-spacing-x) <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>border-spacing-y-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>border-spacing: var(--en-border-spacing-x) (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>border-spacing-y-px</td><td>border-spacing: var(--en-border-spacing-x) 1px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Border spacing values don't follow Tailwind's philosophy of restraining possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `border-spacing-[1.3em]`.

[Tailwind reference](https://tailwindcss.com/docs/border-spacing)
