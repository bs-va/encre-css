Utilities for setting the width of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>w-<i>&lt;float&gt;</i></td><td>width: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>w-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>width: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>w-px</td><td>width: 1px;</td></tr>
    <tr><td>w-full</td><td>width: 100%;</td></tr>
    <tr><td>w-auto</td><td>width: auto;</td></tr>
    <tr><td>w-screen</td><td>width: 100vw;</td></tr>
    <tr><td>w-min</td><td>width: min-content;</td></tr>
    <tr><td>w-max</td><td>width: max-content;</td></tr>
    <tr><td>w-fit</td><td>width: fit-content</td></tr>
    <tr><td>w-svw</td><td>width: 100svw;</td></tr>
    <tr><td>w-lvw</td><td>width: 100lvw;</td></tr>
    <tr><td>w-dvw</td><td>width: 100dvw;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Width values don't follow Tailwind's philosophy of limiting possible values and all spacing values
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `w-[42mm]`.

[Tailwind reference](https://tailwindcss.com/docs/width)
