Utilities for controlling the size of implicitly-created grid columns.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>auto-cols-auto</td><td>grid-auto-columns: auto;</td></tr>
    <tr><td>auto-cols-min</td><td>grid-auto-columns: min-content;</td></tr>
    <tr><td>auto-cols-max</td><td>grid-auto-columns: max-content;</td></tr>
    <tr><td>auto-cols-fr</td><td>grid-auto-columns: minmax(0, 1fr);</td></tr>
  </tbody>
</table>

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `auto-cols-[10%_33.3%]`.

[Tailwind reference](https://tailwindcss.com/docs/grid-auto-columns)
