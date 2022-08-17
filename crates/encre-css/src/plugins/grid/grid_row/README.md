Utilities for controlling how elements are sized and placed across grid rows.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>row-auto</td><td>grid-row: auto;</td></tr>
    <tr><td>row-start-auto</td><td>grid-row-start: auto;</td></tr>
    <tr><td>row-start-<i>&lt;integer&gt;</i></td><td>grid-row-start: <i>&lt;integer&gt;</i>;</td></tr>
    <tr><td>row-end-auto</td><td>grid-row-end: auto;</td></tr>
    <tr><td>row-end-<i>&lt;integer&gt;</i></td><td>grid-row-end: <i>&lt;integer&gt;</i>;</td></tr>
    <tr><td>row-span-full</td><td>grid-row: 1 / -1;</td></tr>
    <tr><td>row-span-<i>&lt;integer&gt;</i></td><td>grid-row: span <i>&lt;integer&gt;</i> / span <i>&lt;integer&gt;</i>;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Grid row start/end/span values don't follow Tailwind's philosophy of limiting possible values and all numbers
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `row-[span_1/7]`.

[Tailwind reference](https://tailwindcss.com/docs/grid-row)
