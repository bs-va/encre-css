Utilities for applying blur filters to an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>blur-none</td><td>filter: blur(0);</td></tr>
    <tr><td>blur-sm</td><td>filter: blur(4px);</td></tr>
    <tr><td>blur</td><td>filter: blur(8px);</td></tr>
    <tr><td>blur-md</td><td>filter: blur(12px);</td></tr>
    <tr><td>blur-lg</td><td>filter: blur(16px);</td></tr>
    <tr><td>blur-xl</td><td>filter: blur(24px);</td></tr>
    <tr><td>blur-2xl</td><td>filter: blur(40px);</td></tr>
    <tr><td>blur-3xl</td><td>filter: blur(64px);</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `blur-[1.3em]`.

[Tailwind reference](https://tailwindcss.com/docs/blur)
