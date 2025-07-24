Utilities for scaling elements with transform.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>scale-<i>&lt;integer&gt;</i></td><td>transform: scale(<i>&lt;integer / 100&gt;</i>);</td></tr>
    <tr><td>scale-x-<i>&lt;integer&gt;</i></td><td>transform: scaleX(<i>&lt;integer / 100&gt;</i>);</td></tr>
    <tr><td>scale-y-<i>&lt;integer&gt;</i></td><td>transform: scaleY(<i>&lt;integer / 100&gt;</i>);</td></tr>
    <tr><td>scale-z-<i>&lt;integer&gt;</i></td><td>transform: scaleZ(<i>&lt;integer / 100&gt;</i>);</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Scale values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) or [`<number>`](crate::utils::value_matchers::is_matching_number) property is allowed as arbitrary value.
For example, `scale-y-[84.42%]`.

### Negative values

This plugin supports negative values. For example, `-scale-50` or `hover:-scale-50`.

[Tailwind reference](https://tailwindcss.com/docs/scale)
