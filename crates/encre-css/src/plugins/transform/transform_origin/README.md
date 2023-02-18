Utilities for specifying the origin for an element's transformations.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>origin-top</td><td>transform-origin: top;</td></tr>
    <tr><td>origin-top-right</td><td>transform-origin: top right;</td></tr>
    <tr><td>origin-right</td><td>transform-origin: right;</td></tr>
    <tr><td>origin-bottom-right</td><td>transform-origin: bottom right;</td></tr>
    <tr><td>origin-bottom</td><td>transform-origin: bottom;</td></tr>
    <tr><td>origin-bottom-left</td><td>transform-origin: bottom left;</td></tr>
    <tr><td>origin-left</td><td>transform-origin: left;</td></tr>
    <tr><td>origin-top-left</td><td>transform-origin: top left;-duration: <i>&lt;integer&gt;</i>ms;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<position>`](crate::utils::value_matchers::is_matching_position) property is allowed as arbitrary value.
For example, `origin-[bottom_right_2cm]`.

[Tailwind reference](https://tailwindcss.com/docs/transform-origin)
