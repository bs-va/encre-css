Utilities for controlling how elements are sized and placed across grid columns.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>col-auto</td><td>grid-column: auto;</td></tr>
    <tr><td>col-start-auto</td><td>grid-column-start: auto;</td></tr>
    <tr><td>col-start-<i>&lt;integer&gt;</i></td><td>grid-column-start: <i>&lt;integer&gt;</i>;</td></tr>
    <tr><td>col-end-auto</td><td>grid-column-end: auto;</td></tr>
    <tr><td>col-end-<i>&lt;integer&gt;</i></td><td>grid-column-end: <i>&lt;integer&gt;</i>;</td></tr>
    <tr><td>col-span-full</td><td>grid-column: 1 / -1;</td></tr>
    <tr><td>col-span-<i>&lt;integer&gt;</i></td><td>grid-column: span <i>&lt;integer&gt;</i> / span <i>&lt;integer&gt;</i>;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Grid column start/end/span values don't follow Tailwind's philosophy of restraining possible values and all numbers
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `col-[2/4]`.

[Tailwind reference](https://tailwindcss.com/docs/grid-column)
