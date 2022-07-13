Utilities for controlling the font size of an element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center; width: 10rem;">Preview</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>text-xs</td><td>font-size: 0.75rem;<br>line-height: 1rem;</td><td><p style="font-size: 0.75rem;<br>line-height: 1rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-sm</td><td>font-size: 0.875rem;<br>line-height: 1.25rem;</td><td><p style="font-size: 0.875rem;<br>line-height: 1.25rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-base</td><td>font-size: 1rem;<br>line-height: 1.5rem;</td><td><p style="font-size: 1rem;<br>line-height: 1.5rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-lg</td><td>font-size: 1.125rem;<br>line-height: 1.75rem;</td><td><p style="font-size: 1.125rem;<br>line-height: 1.75rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-xl</td><td>font-size: 1.25rem;<br>line-height: 1.75rem;</td><td><p style="font-size: 1.25rem;<br>line-height: 1.75rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-2xl</td><td>font-size: 1.5rem;<br>line-height: 2rem;</td><td><p style="font-size: 1.5rem;<br>line-height: 2rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-3xl</td><td>font-size: 1.875rem;<br>line-height: 2.25rem;</td><td><p style="font-size: 1.875rem;<br>line-height: 2.25rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-4xl</td><td>font-size: 2.25rem;<br>line-height: 2.5rem;</td><td><p style="font-size: 2.25rem;<br>line-height: 2.5rem;">Lorem ipsum</p></td></tr>
    <tr><td>text-5xl</td><td>font-size: 3rem;<br>line-height: 1;</td><td><p style="font-size: 3rem;<br>line-height: 1;">Lorem ipsum</p></td></tr>
    <tr><td>text-6xl</td><td>font-size: 3.75rem;<br>line-height: 1;</td><td><p style="font-size: 3.75rem;<br>line-height: 1;">Lorem ipsum</p></td></tr>
    <tr><td>text-7xl</td><td>font-size: 4.5rem;<br>line-height: 1;</td><td><p style="font-size: 4.5rem;<br>line-height: 1;">Lorem ipsum</p></td></tr>
    <tr><td>text-8xl</td><td>font-size: 6rem;<br>line-height: 1;</td><td><p style="font-size: 6rem;<br>line-height: 1;">Lorem ipsum</p></td></tr>
    <tr><td>text-9xl</td><td>font-size: 8rem;<br>line-height: 1;</td><td><p style="font-size: 8rem;<br>line-height: 1;">Lorem ipsum</p></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length), [`<percentage>`](crate::utils::value_matchers::is_matching_percentage), [`absolute size`](crate::utils::value_matchers::is_matching_absolute_size) or [`relative size`](crate::utils::value_matchers::is_matching_relative_size) property is allowed as arbitrary value.
For example, `text-[12%]`.

[Tailwind reference](https://tailwindcss.com/docs/font-size)
