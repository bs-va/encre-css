Utilities for rotating elements with transform.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>rotate-<i>&lt;integer&gt;</i></td><td>transform: rotate(<i>&lt;integer&gt;</i>deg);</td></tr>
    <tr><td>rotate-x-<i>&lt;integer&gt;</i></td><td>transform: rotateX(<i>&lt;integer&gt;</i>deg);</td></tr>
    <tr><td>rotate-y-<i>&lt;integer&gt;</i></td><td>transform: rotateY(<i>&lt;integer&gt;</i>deg);</td></tr>
    <tr><td>rotate-z-<i>&lt;integer&gt;</i></td><td>transform: rotateZ(<i>&lt;integer&gt;</i>deg);</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Rotate values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<angle>`](crate::utils::value_matchers::is_matching_angle) property is allowed as arbitrary value.
For example, `rotate-[0.42turn]`.

### Negative values

This plugin supports negative values. For example, `-rotate-90` or `hover:-rotate-90`.

[Tailwind reference](https://tailwindcss.com/docs/rotate)
