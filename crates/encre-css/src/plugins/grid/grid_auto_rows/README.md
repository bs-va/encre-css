Utilities for controlling the size of implicitly-created grid rows.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>auto-rows-auto</td><td>grid-auto-rows: auto;</td></tr>
    <tr><td>auto-rows-min</td><td>grid-auto-rows: min-content;</td></tr>
    <tr><td>auto-rows-max</td><td>grid-auto-rows: max-content;</td></tr>
    <tr><td>auto-rows-fr</td><td>grid-auto-rows: minmax(0, 1fr);</td></tr>
  </tbody>
</table>

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `auto-rows-[minmax(100px,auto)]`.

[Tailwind reference](https://tailwindcss.com/docs/grid-auto-rows)
