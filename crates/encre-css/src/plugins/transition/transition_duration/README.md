Utilities for controlling the duration of CSS transitions.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>duration-<i>&lt;integer&gt;</i></td><td>transition-duration: <i>&lt;integer&gt;</i>ms;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Duration values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<time>`](crate::utils::value_matchers::is_matching_time) property is allowed as arbitrary value.
For example, `delay-[2.2s]`.

[Tailwind reference](https://tailwindcss.com/docs/transition-duration)
