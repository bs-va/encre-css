Utilities for applying backdrop blur filters to an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>backdrop-blur-none</td><td>backdrop-filter: blur(0);</td></tr>
    <tr><td>backdrop-blur-sm</td><td>backdrop-filter: blur(4px);</td></tr>
    <tr><td>backdrop-blur</td><td>backdrop-filter: blur(8px);</td></tr>
    <tr><td>backdrop-blur-md</td><td>backdrop-filter: blur(12px);</td></tr>
    <tr><td>backdrop-blur-lg</td><td>backdrop-filter: blur(16px);</td></tr>
    <tr><td>backdrop-blur-xl</td><td>backdrop-filter: blur(24px);</td></tr>
    <tr><td>backdrop-blur-2xl</td><td>backdrop-filter: blur(40px);</td></tr>
    <tr><td>backdrop-blur-3xl</td><td>backdrop-filter: blur(64px);</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `backdrop-blur-[1.2em]`.

[Tailwind reference](https://tailwindcss.com/docs/backdrop-blur)
