Utilities for controlling the amount of empty space shown before text in a block.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>indent-<i>&lt;float&gt;</i></td><td>text-indent: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>indent-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>text-indent: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>indent-px</td><td>text-indent: 1px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Indent values don't follow Tailwind's philosophy of limiting possible values and all spacing values
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `indent-[1.2rem]`.

### Negative values

This plugin supports negative values. For example, `-indent-2` or `hover:-indent-2`.

[Tailwind reference](https://tailwindcss.com/docs/text-indent)
