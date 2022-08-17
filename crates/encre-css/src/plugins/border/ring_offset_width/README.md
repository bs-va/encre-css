Utilities for simulating an offset when adding outline rings.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>ring-offset-<i>&lt;integer&gt;</i></td><td>--en-ring-offset-width: <i>&lt;integer&gt;</i>px;<br>box-shadow: 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color), var(--en-ring-shadow);</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Ring offset width values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `ring-offset-[1.2rem]`.

[Tailwind reference](https://tailwindcss.com/docs/ring-offset-width)
