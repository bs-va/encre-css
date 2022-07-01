Utilities for controlling the font weight of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>font-thin</td><td>font-weight: 100;</td></tr>
    <tr><td>font-extralight</td><td>font-weight: 200;</td></tr>
    <tr><td>font-light</td><td>font-weight: 300;</td></tr>
    <tr><td>font-normal</td><td>font-weight: 400;</td></tr>
    <tr><td>font-medium</td><td>font-weight: 500;</td></tr>
    <tr><td>font-semibold</td><td>font-weight: 600;</td></tr>
    <tr><td>font-bold</td><td>font-weight: 700;</td></tr>
    <tr><td>font-extrabold</td><td>font-weight: 800;</td></tr>
    <tr><td>font-black</td><td>font-weight: 900;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<integer>`](crate::utils::value_matchers::is_matching_length) or [`var()`](crate::utils::value_matchers::is_matching_var) property or a keyword among `normal`, `bold`, `lighter` and `bolder` is allowed as arbitrary value.
For example, `font-[1100]`.

[Tailwind reference](https://tailwindcss.com/docs/font-weight)
