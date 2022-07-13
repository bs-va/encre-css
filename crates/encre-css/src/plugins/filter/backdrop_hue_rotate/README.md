Utilities for applying backdrop hue-rotate filters to an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>backdrop-hue-rotate-<i>&lt;integer&gt;</i></td><td>backdrop-filter: hue-rotate(<i>&lt;integer&gt;</i>deg);</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Hue rotate values don't follow Tailwind's philosophy of restraining possible values and all
values from 0 to 360 are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<angle>`](crate::utils::value_matchers::is_matching_angle) property is allowed as arbitrary value.
For example, `backdrop-hue-rotate-[1.5turn]`.

### Negative values

This plugin supports negative values. For example, `-backdrop-hue-rotate-90` or `hover:-backdrop-hue-rotate-90`.

[Tailwind reference](https://tailwindcss.com/docs/backdrop-hue-rotate)
