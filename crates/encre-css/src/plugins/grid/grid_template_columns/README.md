Utilities for specifying the columns in a grid layout.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>grid-cols-<i>&lt;integer&gt;</i></td><td>grid-template-columns: repeat(<i>&lt;integer&gt;</i>, minmax(0, 1fr));</td></tr>
    <tr><td>grid-cols-none</td><td>grid-template-columns: none;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Template column values don't follow Tailwind's philosophy of restraining possible values and all numbers
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `grid-cols-[100px_1fr]`.

[Tailwind reference](https://tailwindcss.com/docs/template-columns)
