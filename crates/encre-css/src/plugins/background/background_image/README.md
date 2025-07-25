Utilities for controlling an element's background image.

<style>
.background-image-table > tr td:nth-child(3) {
  position: relative;
}

.background-image-table > tr {
  height: 4rem;
}

.background-image-table tr td:nth-child(3) div {
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
  <tbody class="background-image-table">
    <tr><td>bg-none</td><td>background-image: none;</td><td><div style="background-image: none;"></div></td></tr>
    <tr><td>bg-gradient-to-t</td><td>background-image: linear-gradient(to top in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-tr</td><td>background-image: linear-gradient(to top right in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top right in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-r</td><td>background-image: linear-gradient(to right in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to right in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-br</td><td>background-image: linear-gradient(to bottom right in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom right in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-b</td><td>background-image: linear-gradient(to bottom in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-bl</td><td>background-image: linear-gradient(to bottom left in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom left in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-l</td><td>background-image: linear-gradient(to left in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to left in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-gradient-to-tl</td><td>background-image: linear-gradient(to top left in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top left in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-t</td><td>background-image: linear-gradient(to top in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-tr</td><td>background-image: linear-gradient(to top right in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top right in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-r</td><td>background-image: linear-gradient(to right in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to right in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-br</td><td>background-image: linear-gradient(to bottom right in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom right in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-b</td><td>background-image: linear-gradient(to bottom in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-bl</td><td>background-image: linear-gradient(to bottom left in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to bottom left in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-l</td><td>background-image: linear-gradient(to left in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to left in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-to-tl</td><td>background-image: linear-gradient(to top left in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(to top left in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-linear-<i>&lt;integer&gt;</i></td><td>background-image: linear-gradient(<i>&lt;integer&gt;</i>deg in oklab, var(--en-gradient-stops));</td><td><div style="background-image: linear-gradient(190deg in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-radial</td><td>background-image: radial-gradient(in oklab, var(--en-gradient-stops));</td><td><div style="background-image: radial-gradient(in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-conic</td><td>background-image: conic-gradient(in oklab, var(--en-gradient-stops));</td><td><div style="background-image: conic-gradient(in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>bg-conic-<i>&lt;integer&gt;</i></td><td>background-image: conic-gradient(from <i>&lt;integer&gt;</i>deg in oklab, var(--en-gradient-stops));</td><td><div style="background-image: conic-gradient(from 190deg in oklab, var(--en-gradient-stops));"></div></td></tr>
  </tbody>
</table>

### Interpolation modes

You can choose the interpolation mode used by each background having a gradient by prefixing the class with `/<interpolation_mode>`, e.g `bg-linear-to-r/hsl` or `bg-radial/increasing`.

The following interpolation modes are available:

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Interpolation mode</th>
      <th style="text-align: center;">CSS argument used</th>
      <th style="text-align: center; width: 10rem;">Preview</th>
    </tr>
  </thead>
  <tbody class="background-image-table">
    <tr><td>srgb</td><td>srgb</td><td><div style="background-image: linear-gradient(to right in srgb, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>hsl</td><td>hsl</td><td><div style="background-image: linear-gradient(to right in hsl, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>oklab</td><td>oklab</td><td><div style="background-image: linear-gradient(to right in oklab, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>oklch</td><td>oklch</td><td><div style="background-image: linear-gradient(to right in oklch, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>longer</td><td>oklch longer hue</td><td><div style="background-image: linear-gradient(to right in oklch longer hue, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>shorter</td><td>oklch shorter hue</td><td><div style="background-image: linear-gradient(to right in oklch shorter hue, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>increasing</td><td>oklch increasing hue</td><td><div style="background-image: linear-gradient(to right in oklch increasing hue, var(--en-gradient-stops));"></div></td></tr>
    <tr><td>decreasing</td><td>oklch decreasing hue</td><td><div style="background-image: linear-gradient(to right in oklch decreasing hue, var(--en-gradient-stops));"></div></td></tr>
  </tbody>
</table>


### Arbitrary values

Any [`<image>`](crate::utils::value_matchers::is_matching_image) property is allowed as arbitrary value for `bg-`.
For example, `bg-[url('/hello.png')]`.

Any property is allowed as arbitrary value for `bg-linear-`.
For example, `bg-linear-[to_bottom_#fff,#000]`.

Any property is allowed as arbitrary value for `bg-radial-`.
For example, `bg-radial-[at_25%_25%]`.

Any property is allowed as arbitrary value for `bg-conic-`.
For example, `bg-conic-[from_22deg_in_oklab,#fff,#000]`.

[Tailwind reference](https://tailwindcss.com/docs/background-image)
