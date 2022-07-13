Utilities for skewing elements with transform.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>skew-x-<i>&lt;integer&gt;</i></td><td>transform: skewX(<i>&lt;integer&gt;</i>deg);</td></tr>
    <tr><td>skew-y-<i>&lt;integer&gt;</i></td><td>transform: skewY(<i>&lt;integer&gt;</i>deg);</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Skew values don't follow Tailwind's philosophy of restraining possible values and all
numbers from 0 to 360 are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<angle>`](crate::utils::value_matchers::is_matching_angle) property is allowed as arbitrary value.
For example, `skew-x-[-0.312rad]`.

### Negative values

This plugin supports negative values. For example, `-skew-x-2` or `hover:-skew-x-2`.

[Tailwind reference](https://tailwindcss.com/docs/skew)
