Utilities for controlling the placement of positioned elements.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>inset-<i>&lt;float&gt;</i></td><td>inset: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>inset-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>inset: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>inset-px</td><td>inset: 1px;</td></tr>
    <tr><td>inset-auto</td><td>inset: auto;</td></tr>
    <tr><td>inset-full</td><td>inset: 100%;</td></tr>
    <tr><td>inset-x-<i>&lt;float&gt;</i></td><td>inset-inline: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>inset-x-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>inset-inline: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>inset-x-px</td><td>inset-inline: 1px;</td></tr>
    <tr><td>inset-x-auto</td><td>inset-inline: auto;</td></tr>
    <tr><td>inset-x-full</td><td>inset-inline: 100%;</td></tr>
    <tr><td>inset-y-<i>&lt;float&gt;</i></td><td>inset-block: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>inset-y-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>inset-block: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>inset-y-px</td><td>inset-block: 1px;</td></tr>
    <tr><td>inset-y-auto</td><td>inset-block: auto;</td></tr>
    <tr><td>inset-y-full</td><td>inset-block: 100%;</td></tr>
    <tr><td>start-<i>&lt;float&gt;</i></td><td>inset-inline-start: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>start-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>inset-inline-start: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>start-px</td><td>inset-inline-start: 1px;</td></tr>
    <tr><td>start-auto</td><td>inset-inline-start: auto;</td></tr>
    <tr><td>start-full</td><td>inset-inline-start: 100%;</td></tr>
    <tr><td>end-<i>&lt;float&gt;</i></td><td>inset-inline-end: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>end-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>inset-inline-end: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>end-px</td><td>inset-inline-end: 1px;</td></tr>
    <tr><td>end-auto</td><td>inset-inline-end: auto;</td></tr>
    <tr><td>end-full</td><td>inset-inline-end: 100%;</td></tr>
    <tr><td>top-<i>&lt;float&gt;</i></td><td>top: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>top-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>top-px</td><td>top: 1px;</td></tr>
    <tr><td>top-auto</td><td>top: auto;</td></tr>
    <tr><td>top-full</td><td>top: 100%;</td></tr>
    <tr><td>bottom-<i>&lt;float&gt;</i></td><td>bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>bottom-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>bottom-px</td><td>bottom: 1px;</td></tr>
    <tr><td>bottom-auto</td><td>bottom: auto;</td></tr>
    <tr><td>bottom-full</td><td>bottom: 100%;</td></tr>
    <tr><td>left-<i>&lt;float&gt;</i></td><td>left: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>left-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>left-px</td><td>left: 1px;</td></tr>
    <tr><td>left-auto</td><td>left: auto;</td></tr>
    <tr><td>left-full</td><td>left: 100%;</td></tr>
    <tr><td>right-<i>&lt;float&gt;</i></td><td>right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>right-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>right-px</td><td>right: 1px;</td></tr>
    <tr><td>right-auto</td><td>right: auto;</td></tr>
    <tr><td>right-full</td><td>right: 100%;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Top/right/bottom/left values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property or the keyword `auto` is allowed as arbitrary value.
For example, `top-[1.12rem]`.

### Negative values

This plugin supports negative values. For example, `-top-2` or `hover:-top-2`.

[Tailwind reference](https://tailwindcss.com/docs/top-right-bottom-left)
