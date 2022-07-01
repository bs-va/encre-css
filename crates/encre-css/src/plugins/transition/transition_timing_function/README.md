Utilities for controlling the easing of CSS transitions.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>ease-linear</td><td>transition-timing-function: linear;</td></tr>
    <tr><td>ease-in</td><td>transition-timing-function: cubic-bezier(0.4, 0, 1, 1);</td></tr>
    <tr><td>ease-out</td><td>transition-timing-function: cubic-bezier(0, 0, 0.2, 1);</td></tr>
    <tr><td>ease-in-out</td><td>transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);</td></tr>
  </tbody>
</table>

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `ease-[cubic-bezier(0.55,0.72,0.83,0.42)]`.

[Tailwind reference](https://tailwindcss.com/docs/transition-timing-function)
