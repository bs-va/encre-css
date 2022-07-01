Utilities for specifying the rows in a grid layout.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>grid-rows-<i>&lt;integer&gt;</i></td><td>grid-template-rows: repeat(<i>&lt;integer&gt;</i>, minmax(0, 1fr));</td></tr>
    <tr><td>grid-rows-none</td><td>grid-template-rows: none;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Template row values don't follow Tailwind's philosophy of restraining possible values and all numbers
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `grid-rows-[200px_repeat(auto-fill,100px)_300px]`.

[Tailwind reference](https://tailwindcss.com/docs/template-rows)
