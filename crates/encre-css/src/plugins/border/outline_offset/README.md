Utilities for controlling the offset of an element's outline.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>outline-offset-<i>&lt;integer&gt;</i></td><td>outline-offset: <i>&lt;integer&gt;</i>px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Outline offset values don't follow Tailwind's philosophy of restraining possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `outline-offset-[2rem]`.

[Tailwind reference](https://tailwindcss.com/docs/outline-offset)
