Utilities for controlling the tracking (letter spacing) of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>tracking-tighter</td><td>letter-spacing: -0.05em;</td></tr>
    <tr><td>tracking-tight</td><td>letter-spacing: -0.025em;</td></tr>
    <tr><td>tracking-normal</td><td>letter-spacing: 0em;</td></tr>
    <tr><td>tracking-wide</td><td>letter-spacing: 0.025em;</td></tr>
    <tr><td>tracking-wider</td><td>letter-spacing: 0.05em;</td></tr>
    <tr><td>tracking-widest</td><td>letter-spacing: 0.1em;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property or the keyword `normal` is allowed as arbitrary value.
For example, `tracking-[1.2rem]`.

[Tailwind reference](https://tailwindcss.com/docs/letter-spacing)
