Utilities for controlling which CSS properties transition.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>transition-none</td><td>transition-property: none;</td></tr>
    <tr><td>transition-all</td><td>transition-property: all;<br>transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);<br>transition-duration: 150ms;</td></tr>
    <tr><td>transition</td><td>transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;<br>transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);<br>transition-duration: 150ms;</td></tr>
    <tr><td>transition-colors</td><td>transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;<br>transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);<br>transition-duration: 150ms;</td></tr>
    <tr><td>transition-opacity</td><td>transition-property: opacity;<br>transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);<br>transition-duration: 150ms;</td></tr>
    <tr><td>transition-shadow</td><td>transition-property: box-shadow;<br>transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);<br>transition-duration: 150ms;</td></tr>
    <tr><td>transition-transform</td><td>transition-property: transform;<br>transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);<br>transition-duration: 150ms;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `transition-[opacity,background-color]`.

[Tailwind reference](https://tailwindcss.com/docs/transition-property)
