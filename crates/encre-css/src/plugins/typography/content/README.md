Utilities for controlling the content of the before and after pseudo-elements.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>content-none</td><td>content: none;</td></tr>
  </tbody>
</table>

### Note

The `content` utility must be used in combination with either the `before` or
the `after` variant to work.

### Arbitrary values

Any property is allowed as arbitrary value. It is more over the only way of
setting the content.
For example, `before:content-['Hello_world!']` or `after:content-[url('foobar.png')]`.

[Tailwind reference](https://tailwindcss.com/docs/content)
