Utilities for controlling the position of an element's background image.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>bg-bottom</td><td>background-position: bottom;</td></tr>
    <tr><td>bg-center</td><td>background-position: center;</td></tr>
    <tr><td>bg-left</td><td>background-position: left;</td></tr>
    <tr><td>bg-left-bottom</td><td>background-position: left bottom;</td></tr>
    <tr><td>bg-left-top</td><td>background-position: left top;</td></tr>
    <tr><td>bg-right</td><td>background-position: right;</td></tr>
    <tr><td>bg-right-bottom</td><td>background-position: right bottom;</td></tr>
    <tr><td>bg-right-top</td><td>background-position: right top;</td></tr>
    <tr><td>bg-top</td><td>background-position: top;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<position>`](crate::utils::value_matchers::is_matching_position) property is allowed as arbitrary value.
For example, `bg-[position:0px_0px,bottom_12cm_right_-6px]`.

### Tailwind compatibility

When using arbitrary values, you **must specify the `position` hint**
to disambiguate between this plugin and the [`background_size`](crate::plugins::background::background_size) plugin.
That's not the case in Tailwind: `background-position` is preferred over `background-size`.

[Tailwind reference](https://tailwindcss.com/docs/background-position)
