Utilities for controlling the border width between children.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>divide-x-<i>&lt;integer&gt;</i></td>border-inline-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>divide-x-reverse</td><td>--en-divide-x-reverse: 1;</td></tr>
    <tr><td>divide-y-<i>&lt;integer&gt;</i></td><td>border-block-width: <i>&lt;integer&gt;</i>px;</td></tr>
    <tr><td>divide-y-reverse</td><td>--en-divide-y-reverse: 1;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Divide width values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `divide-y-[0.1rem]`.

[Tailwind reference](https://tailwindcss.com/docs/divide-width)
