Utilities for controlling how an element's background image should blend with its background color.

<style>
#background-blend-mode-table > tr td:nth-child(3) {
  position: relative;
}

#background-blend-mode-table > tr {
  height: 100px;
}

#background-blend-mode-table tr td:nth-child(3) div {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  margin: 10px;
  width: 80px;
  height: 80px;
  background-image: url("https://gitlab.com/encre-css/encre-css/raw/main/.assets/logo.svg");
  background-color: rgb(168 85 247);
}
</style>

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center; width: 100px;">Preview</th>
    </tr>
  </thead>
  <tbody id="background-blend-mode-table">
    <tr><td>bg-blend-normal</td><td>background-blend-mode: normal;</td><td><div style="background-blend-mode: normal;"></div></td></tr>
    <tr><td>bg-blend-multiply</td><td>background-blend-mode: multiply;</td><td><div style="background-blend-mode: multiply;"></div></td></tr>
    <tr><td>bg-blend-screen</td><td>background-blend-mode: screen;</td><td><div style="background-blend-mode: screen;"></div></td></tr>
    <tr><td>bg-blend-overlay</td><td>background-blend-mode: overlay;</td><td><div style="background-blend-mode: overlay;"></div></td></tr>
    <tr><td>bg-blend-darken</td><td>background-blend-mode: darken;</td><td><div style="background-blend-mode: darken;"></div></td></tr>
    <tr><td>bg-blend-lighten</td><td>background-blend-mode: lighten;</td><td><div style="background-blend-mode: lighten;"></div></td></tr>
    <tr><td>bg-blend-color-dodge</td><td>background-blend-mode: color-dodge;</td><td><div style="background-blend-mode: color-dodge;"></div></td></tr>
    <tr><td>bg-blend-color-burn</td><td>background-blend-mode: color-burn;</td><td><div style="background-blend-mode: color-burn;"></div></td></tr>
    <tr><td>bg-blend-hard-light</td><td>background-blend-mode: hard-light;</td><td><div style="background-blend-mode: hard-light;"></div></td></tr>
    <tr><td>bg-blend-soft-light</td><td>background-blend-mode: soft-light;</td><td><div style="background-blend-mode: soft-light;"></div></td></tr>
    <tr><td>bg-blend-difference</td><td>background-blend-mode: difference;</td><td><div style="background-blend-mode: difference;"></div></td></tr>
    <tr><td>bg-blend-exclusion</td><td>background-blend-mode: exclusion;</td><td><div style="background-blend-mode: exclusion;"></div></td></tr>
    <tr><td>bg-blend-hue</td><td>background-blend-mode: hue;</td><td><div style="background-blend-mode: hue;"></div></td></tr>
    <tr><td>bg-blend-saturation</td><td>background-blend-mode: saturation;</td><td><div style="background-blend-mode: saturation;"></div></td></tr>
    <tr><td>bg-blend-color</td><td>background-blend-mode: color;</td><td><div style="background-blend-mode: color;"></div></td></tr>
    <tr><td>bg-blend-luminosity</td><td>background-blend-mode: luminosity;</td><td><div style="background-blend-mode: luminosity;"></div></td></tr>
  </tbody>
</table>

[Tailwind reference](https://tailwindcss.com/docs/background-blend-mode)
