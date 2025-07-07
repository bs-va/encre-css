Utilities for controlling the width of an element's borders.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>border-<i>&lt;integer&gt;</i></td><td>border-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border</td><td>border-width: 1px;</td></tr>
    <tr><td>border-x-<i>&lt;integer&gt;</i></td><td>border-inline-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-x</td><td>border-inline-width: 1px;</td></tr>
    <tr><td>border-y-<i>&lt;integer&gt;</i></td><td>border-block-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-y</td><td>border-block-width: 1px;</td></tr>
    <tr><td>border-s-<i>&lt;integer&gt;</i></td><td>border-inline-start-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-s</td><td>border-inline-start-width: 1px;</td></tr>
    <tr><td>border-e-<i>&lt;integer&gt;</i></td><td>border-inline-end-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-e</td><td>border-inline-end-width: 1px;</td></tr>
    <tr><td>border-t-<i>&lt;integer&gt;</i></td><td>border-top-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-t</td><td>border-top-width: 1px;</td></tr>
    <tr><td>border-r-<i>&lt;integer&gt;</i></td><td>border-right-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-r</td><td>border-right-width: 1px;</td></tr>
    <tr><td>border-b-<i>&lt;integer&gt;</i></td><td>border-bottom-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-b</td><td>border-bottom-width: 1px;</td></tr>
    <tr><td>border-l-<i>&lt;integer&gt;</i></td><td>border-left-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>border-l</td><td>border-left-width: 1px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Border width values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`line width`](crate::utils::value_matchers::is_matching_line_width) property is allowed as arbitrary value.
For example, `border-x-[1.2rem]`.

[Tailwind reference](https://tailwindcss.com/docs/border-width)
