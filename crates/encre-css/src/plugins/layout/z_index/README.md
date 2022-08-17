Utilities for controlling the stack order of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>z-<i>&lt;integer&gt;</i></td><td>z-index: <i>&lt;integer&gt;</i>;</td></tr>
    <tr><td>z-auto</td><td>z-index: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Z-Index values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Negative values

This plugin supports negative values. For example, `-z-2` or `hover:-z-2`.

[Tailwind reference](https://tailwindcss.com/docs/z-index)
