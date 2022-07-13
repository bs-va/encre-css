Utilities for translating elements with transform.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>translate-x-<i>&lt;integer&gt;</i></td><td>transform: translateX(<i>&lt;integer&gt;</i>px);</td></tr>
    <tr><td>translate-y-<i>&lt;integer&gt;</i></td><td>transform: translateY(<i>&lt;integer&gt;</i>px);</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Translate values don't follow Tailwind's philosophy of restraining possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `translate-y-[1.45rem]`.

### Negative values

This plugin supports negative values. For example, `-translate-x-2` or `hover:-translate-x-2`.

[Tailwind reference](https://tailwindcss.com/docs/translate)
