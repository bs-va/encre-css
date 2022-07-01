Utilities for controlling transform behavior.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>transform</td><td>filter: transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));</td></tr>
    <tr><td>transform-cpu</td><td>filter: transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));</td></tr>
    <tr><td>transform-gpu</td><td>filter: transform: translate3d(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));</td></tr>
    <tr><td>transform-none</td><td>transform: none;</td></tr>
  </tbody>
</table>

### Deprecation notice

It is not required to use the `transform` class to use transforms, this class
only exists for compatibility reasons.
