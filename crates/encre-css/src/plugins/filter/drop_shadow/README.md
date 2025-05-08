Utilities for applying drop-shadow filters to an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>drop-shadow-xs</td><td>filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));</td></tr>
    <tr><td>drop-shadow-sm</td><td>filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));</td></tr>
    <tr><td>drop-shadow-md</td><td>filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));</td></tr>
    <tr><td>drop-shadow-lg</td><td>filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));</td></tr>
    <tr><td>drop-shadow-xl</td><td>filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));</td></tr>
    <tr><td>drop-shadow-2xl</td><td>filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));</td></tr>
    <tr><td>drop-shadow-none</td><td>filter: drop-shadow(0 0 #0000);</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<shadow>`](crate::utils::value_matchers::is_matching_shadow) property is allowed as arbitrary value.
For example, `drop-shadow-[1.2rem_1em_20px_10px_#f00]`.

[Tailwind reference](https://tailwindcss.com/docs/drop-shadow)
