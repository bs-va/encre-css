Utilities for visually truncating text after a fixed number of lines.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>line-clamp-<i>&lt;integer&gt;</i></td><td>overflow: hidden;<br>display: -webkit-box;<br>-webkit-box-orient: vertical;<br>-webkit-line-clamp: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>line-clamp-none</td><td>-webkit-line-clamp: unset;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Line clamp values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

[Tailwind reference](https://github.com/tailwindlabs/tailwindcss-line-clamp)
