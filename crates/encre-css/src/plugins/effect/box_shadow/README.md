Utilities for controlling the box shadow of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>shadow-sm</td><td>box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);</td></tr>
    <tr><td>shadow</td><td>box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);</td></tr>
    <tr><td>shadow-md</td><td>box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);</td></tr>
    <tr><td>shadow-lg</td><td>box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);</td></tr>
    <tr><td>shadow-xl</td><td>box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);</td></tr>
    <tr><td>shadow-2xl</td><td>box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);</td></tr>
    <tr><td>shadow-inner</td><td>box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);</td></tr>
    <tr><td>shadow-none</td><td>box-shadow: 0 0 #0000;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<shadow>`](crate::utils::value_matchers::is_matching_shadow) property is allowed as arbitrary value.
For example, `shadow-[2px_1px_20px_2px_rgb(12_12_42)]`.

[Tailwind reference](https://tailwindcss.com/docs/box-shadow)
