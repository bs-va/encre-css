Utilities for controlling how a replaced element's content should be positioned within its container.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>object-bottom</td><td>object-position: bottom;</td></tr>
    <tr><td>object-center</td><td>object-position: center;</td></tr>
    <tr><td>object-left</td><td>object-position: left;</td></tr>
    <tr><td>object-left-bottom</td><td>object-position: left bottom;</td></tr>
    <tr><td>object-left-top</td><td>object-position: left top;</td></tr>
    <tr><td>object-right</td><td>object-position: right;</td></tr>
    <tr><td>object-right-bottom</td><td>object-position: right bottom;</td></tr>
    <tr><td>object-right-top</td><td>object-position: right top;</td></tr>
    <tr><td>object-top</td><td>object-position: top;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<position>`](crate::utils::value_matchers::is_matching_position) property is allowed as arbitrary value.
For example, `object-[bottom_10px_right_20px]`.

[Tailwind reference](https://tailwindcss.com/docs/object-position)
