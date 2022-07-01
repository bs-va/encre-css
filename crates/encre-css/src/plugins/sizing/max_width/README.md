Utilities for setting the maximum width of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>max-w-<i>&lt;float&gt;</i></td><td>max-width: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>max-w-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>max-width: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>max-w-px</td><td>max-width: 1px;</td></tr>
    <tr><td>max-w-screen</td><td>max-width: 100vw;</td></tr>
    <tr><td>max-w-none</td><td>max-width: none;</td></tr>
    <tr><td>max-w-xs</td><td>max-width: 20rem;</td></tr>
    <tr><td>max-w-sm</td><td>max-width: 24rem;</td></tr>
    <tr><td>max-w-md</td><td>max-width: 28rem;</td></tr>
    <tr><td>max-w-lg</td><td>max-width: 32rem;</td></tr>
    <tr><td>max-w-xl</td><td>max-width: 36rem;</td></tr>
    <tr><td>max-w-2xl</td><td>max-width: 42rem;</td></tr>
    <tr><td>max-w-3xl</td><td>max-width: 48rem;</td></tr>
    <tr><td>max-w-4xl</td><td>max-width: 56rem;</td></tr>
    <tr><td>max-w-5xl</td><td>max-width: 64rem;</td></tr>
    <tr><td>max-w-6xl</td><td>max-width: 72rem;</td></tr>
    <tr><td>max-w-7xl</td><td>max-width: 80rem;</td></tr>
    <tr><td>max-w-full</td><td>max-width: 100%;</td></tr>
    <tr><td>max-w-min</td><td>max-width: min-content;</td></tr>
    <tr><td>max-w-max</td><td>max-width: max-content;</td></tr>
    <tr><td>max-w-fit</td><td>max-width: fit-content;</td></tr>
    <tr><td>max-w-prose</td><td>max-width: 65ch;</td></tr>
    <tr><td>max-w-screen-sm</td><td>max-width: 640px;</td></tr>
    <tr><td>max-w-screen-md</td><td>max-width: 768px;</td></tr>
    <tr><td>max-w-screen-lg</td><td>max-width: 1024px;</td></tr>
    <tr><td>max-w-screen-xl</td><td>max-width: 1280px;</td></tr>
    <tr><td>max-w-screen-2xl</td><td>max-width: 1536px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Maximum width values don't follow Tailwind's philosophy of restraining possible values and all
spacing values are supported (`screen` also). They are however perfectly compatible with
Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `max-w-[1.2rem]`.

[Tailwind reference](https://tailwindcss.com/docs/max-width)
