Utilities for controlling an element's background image.

<style>
#background-image-table > tr td:nth-child(3) {
  position: relative;
}

#background-image-table > tr {
  height: 4rem;
}

#background-image-table tr td:nth-child(3) div {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  margin: 0.75rem;
  --en-gradient-from: rgb(99 102 241);
  --en-gradient-to: rgb(236 72 153);
  --en-gradient-stops: var(--en-gradient-from), rgb(168 85 247), var(--en-gradient-to, rgb(168 85 247/ 0));
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
  <tbody id="background-image-table">
    <tr><td>bg-none</td><td>background-image: none;</td><td><div style="background-image: none;"></div></td></tr>
    <tr><td>bg-gradient-to-t</td><td>background-image: linear-gradient(to top, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-tr</td><td>background-image: linear-gradient(to top right, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top right, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-r</td><td>background-image: linear-gradient(to right, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to right, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-br</td><td>background-image: linear-gradient(to bottom right, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom right, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-b</td><td>background-image: linear-gradient(to bottom, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-bl</td><td>background-image: linear-gradient(to bottom left, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom left, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-l</td><td>background-image: linear-gradient(to left, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to left, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-tl</td><td>background-image: linear-gradient(to top left, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top left, var(--en-gradient-stops));"></div></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<image>`](crate::utils::value_matchers::is_matching_image) property is allowed as arbitrary value.
For example, `bg-[url('/hello.png')]`.

[Tailwind reference](https://tailwindcss.com/docs/background-image)
