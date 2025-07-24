Utilities for controlling the text-shadow of a text element.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center;">Preview</th>
    </tr>
  </thead>
  <tbody id="text-shadow-table">
    <tr><td>text-shadow-2xs</td><td>text-shadow: 0px 1px 0px rgb(0 0 0 / 0.15);</td><td><p style="font-weight: bolder; text-shadow: 0px 1px 0px rgb(0 0 0 / 0.15);">Lorem</p></td></tr>
    <tr><td>text-shadow-xs</td><td>text-shadow: 0px 1px 1px rgb(0 0 0 / 0.2);</td><td><div style="font-weight: bolder; text-shadow: 0px 1px 1px rgb(0 0 0 / 0.2);">Lorem</div></td></tr>
    <tr><td>text-shadow-sm</td><td>text-shadow: 0px 1px 0px rgb(0 0 0 / 0.075), 0px 1px 1px rgb(0 0 0 / 0.075), 0px 2px 2px rgb(0 0 0 / 0.075);</td><td><div style="font-weight: bolder; text-shadow: 0px 1px 0px rgb(0 0 0 / 0.075), 0px 1px 1px rgb(0 0 0 / 0.075), 0px 2px 2px rgb(0 0 0 / 0.075);">Lorem</div></td></tr>
    <tr><td>text-shadow-md</td><td>text-shadow: 0px 1px 1px rgb(0 0 0 / 0.1), 0px 1px 2px rgb(0 0 0 / 0.1), 0px 2px 4px rgb(0 0 0 / 0.1);</td><td><div style="font-weight: bolder; text-shadow: 0px 1px 1px rgb(0 0 0 / 0.1), 0px 1px 2px rgb(0 0 0 / 0.1), 0px 2px 4px rgb(0 0 0 / 0.1);">Lorem</div></td></tr>
    <tr><td>text-shadow-lg</td><td>text-shadow: 0px 1px 2px rgb(0 0 0 / 0.1), 0px 3px 2px rgb(0 0 0 / 0.1), 0px 4px 8px rgb(0 0 0 / 0.1);</td><td><div style="font-weight: bolder; text-shadow: 0px 1px 2px rgb(0 0 0 / 0.1), 0px 3px 2px rgb(0 0 0 / 0.1), 0px 4px 8px rgb(0 0 0 / 0.1);">Lorem</div></td></tr>
    <tr><td>text-shadow-none</td><td>text-shadow: none;</td><td><div style="font-weight: bolder; text-shadow: none;">Lorem</div></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<shadow>`](crate::utils::value_matchers::is_matching_shadow) property is allowed as arbitrary value.
For example, `text-shadow-[2px_1px_20px_2px_rgb(12_12_42)]`.

[Tailwind reference](https://tailwindcss.com/docs/text-shadow)
