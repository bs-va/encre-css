Utilities for applying hue-rotate filters to an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>hue-rotate-<i>&lt;integer&gt;</i></td><td>filter: hue-rotate(<i>&lt;integer&gt;</i>deg);</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Hue rotate values don't follow Tailwind's philosophy of restraining possible values and all
values from 0 to 360 are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<angle>`](crate::utils::value_matchers::is_matching_angle) property is allowed as arbitrary value.
For example, `hue-rotate-[38.8grad]`.

[Tailwind reference](https://tailwindcss.com/docs/hue-rotate)
