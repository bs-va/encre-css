Utilities for setting the height of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>h-<i>&lt;float&gt;</i></td><td>height: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>h-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>height: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>h-px</td><td>height: 1px;</td></tr>
    <tr><td>h-full</td><td>height: 100%;</td></tr>
    <tr><td>h-auto</td><td>height: auto;</td></tr>
    <tr><td>h-screen</td><td>height: 100vw;</td></tr>
    <tr><td>h-min</td><td>height: min-content;</td></tr>
    <tr><td>h-max</td><td>height: max-content;</td></tr>
    <tr><td>h-fit</td><td>height: fit-content</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Height values don't follow Tailwind's philosophy of limiting possible values and all spacing values
are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `h-[3em]`.

[Tailwind reference](https://tailwindcss.com/docs/height)
