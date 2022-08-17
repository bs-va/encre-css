Utilities for controlling the initial size of flex items.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>basis-<i>&lt;float&gt;</i></td><td>flex-basis: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>basis-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>flex-basis: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>basis-px</td><td>flex-basis: 1px;</td></tr>
    <tr><td>basis-full</td><td>flex-basis: 100%;</td></tr>
    <tr><td>basis-auto</td><td>flex-basis: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Flex basis values don't follow Tailwind's philosophy of limiting possible values and all spacing values
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `basis-[1.45rem]`.

[Tailwind reference](https://tailwindcss.com/docs/flex-basis)
