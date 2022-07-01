Utilities for controlling an element's background image.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>bg-none</td><td>background-image: none;</td></tr>
    <tr><td>bg-gradient-to-t</td><td>background-image: linear-gradient(to top, var(--en-gradient-stops));</td></tr>
    <tr><td>bg-gradient-to-tr</td><td>background-image: linear-gradient(to top right, var(--en-gradient-stops));</td></tr>
    <tr><td>bg-gradient-to-r</td><td>background-image: linear-gradient(to right, var(--en-gradient-stops));</td></tr>
    <tr><td>bg-gradient-to-br</td><td>background-image: linear-gradient(to bottom right, var(--en-gradient-stops));</td></tr>
    <tr><td>bg-gradient-to-b</td><td>background-image: linear-gradient(to bottom, var(--en-gradient-stops));</td></tr>
    <tr><td>bg-gradient-to-bl</td><td>background-image: linear-gradient(to bottom left, var(--en-gradient-stops));</td></tr>
    <tr><td>bg-gradient-to-l</td><td>background-image: linear-gradient(to left, var(--en-gradient-stops));</td></tr>
    <tr><td>bg-gradient-to-tl</td><td>background-image: linear-gradient(to top left, var(--en-gradient-stops));</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<image>`](crate::utils::value_matchers::is_matching_image) property is allowed as arbitrary value.
For example, `bg-[url(/hello.png)]`.

[Tailwind reference](https://tailwindcss.com/docs/background-image)
