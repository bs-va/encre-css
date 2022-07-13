Utilities for controlling the space between child elements.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>space-x-<i>&lt;float&gt;</i></td><td>margin-left: <i>&lt;float / 4&gt;</i>rem;<br>margin-right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>space-x-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;<br>margin-right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>space-x-px</td><td>margin-left: 1px;<br>margin-right: 1px;</td></tr>
    <tr><td>space-x-auto</td><td>margin-left: auto;<br>margin-right: auto;</td></tr>
    <tr><td>space-y-<i>&lt;float&gt;</i></td><td>margin-top: <i>&lt;float / 4&gt;</i>rem;<br>margin-bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>space-y-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;<br>margin-bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>space-y-px</td><td>margin-top: 1px;<br>margin-bottom: 1px;</td></tr>
    <tr><td>space-y-auto</td><td>margin-top: auto;<br>margin-bottom: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Spacing between values don't follow Tailwind's philosophy of restraining possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `space-x-[1rem]`.

### Negative values

This plugin supports negative values. For example, `-space-x-2` or `hover:-space-x-2`.

[Tailwind reference](https://tailwindcss.com/docs/space)
