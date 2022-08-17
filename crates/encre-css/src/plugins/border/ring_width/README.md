Utilities for creating outline rings with box-shadows.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>ring-<i>&lt;integer&gt;</i></td><td>box-shadow: var(--en-ring-inset) 0 0 0 calc(<i>&lt;integer&gt;</i>px + var(--en-ring-offset-width)) var(--en-ring-color);</td></tr>
    <tr><td>ring-inset</td><td>--en-ring-inset: inset;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Ring width values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `ring-[1.2rem]`.

[Tailwind reference](https://tailwindcss.com/docs/ring-width)
