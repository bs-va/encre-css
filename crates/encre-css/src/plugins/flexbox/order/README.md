Utilities for controlling the order of flex and grid items.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>order-<i>&lt;integer&gt;</i></td><td>order: <i>&lt;integer&gt;</i>;</td></tr>
    <tr><td>order-first</td><td>order: -9999;</td></tr>
    <tr><td>order-last</td><td>order: 9999;</td></tr>
    <tr><td>order-none</td><td>order: 0;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Order values don't follow Tailwind's philosophy of limiting possible values and all numbers
are supported. They are however perfectly compatible with Tailwind's values.

### Negative values

This plugin supports negative values. For example, `-order-2` or `hover:-order-2`.

[Tailwind reference](https://tailwindcss.com/docs/order)
