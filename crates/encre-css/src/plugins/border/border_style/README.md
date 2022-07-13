Utilities for controlling the style of an element's borders.

<style>
#border-style-table > tr td:nth-child(3) {
  position: relative;
}

#border-style-table > tr {
  height: 4rem;
}

#border-style-table tr td:nth-child(3) div {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  margin: 1rem;
  border-color: rgb(209 213 219);
  border-width: 4px;
}
</style>

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center; width: 10rem;">Preview</th>
    </tr>
  </thead>
  <tbody id="border-style-table">
    <tr><td>border-solid</td><td>border-style: solid;</td><td><div style="border-style: solid;"></div></td></tr>
    <tr><td>border-dashed</td><td>border-style: dashed;</td><td><div style="border-style: dashed;"></div></td></tr>
    <tr><td>border-dotted</td><td>border-style: dotted;</td><td><div style="border-style: dotted;"></div></td></tr>
    <tr><td>border-double</td><td>border-style: double;</td><td><div style="border-style: double;"></div></td></tr>
    <tr><td>border-hidden</td><td>border-style: hidden;</td><td><div style="border-style: hidden;"></div></td></tr>
    <tr><td>border-none</td><td>border-style: none;</td><td><div style="border-style: none;"></div></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`line style`](crate::utils::value_matchers::is_matching_line_style) property is allowed as arbitrary value.
For example, `border-[dotted_solid_none_dashed]`.

[Tailwind reference](https://tailwindcss.com/docs/border-style)
