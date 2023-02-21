Utilities for controlling the background size of an element's background image.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>bg-auto</td><td>background-size: auto;</td></tr>
    <tr><td>bg-cover</td><td>background-size: cover;</td></tr>
    <tr><td>bg-contain</td><td>background-size: contain;</td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property or a keyword among `contain`, `cover` and `auto` is allowed as arbitrary value.
For example, `bg-[12px_50%,6px,contain]`.

### Tailwind compatibility

When using arbitrary values, you **don't need to specify the `length` hint**.
Unlike Tailwind, in `encre-css`, `background-size` is preferred over `background-position`.

[Tailwind reference](https://tailwindcss.com/docs/background-size)
