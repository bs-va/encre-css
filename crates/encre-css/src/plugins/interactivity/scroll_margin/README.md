Utilities for controlling the scroll offset around items in a snap container.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>scroll-m-<i>&lt;float&gt;</i></td><td>margin: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-m-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-m-px</td><td>margin: 1px;</td></tr>
    <tr><td>scroll-m-auto</td><td>margin: auto;</td></tr>
    <tr><td>scroll-mx-<i>&lt;float&gt;</i></td><td>scroll-margin-left: <i>&lt;float / 4&gt;</i>rem;<br>scroll-margin-right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-mx-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;<br>scroll-margin-right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-mx-px</td><td>scroll-margin-left: 1px;<br>scroll-margin-right: 1px;</td></tr>
    <tr><td>scroll-mx-auto</td><td>scroll-margin-left: auto;<br>scroll-margin-right: auto;</td></tr>
    <tr><td>scroll-my-<i>&lt;float&gt;</i></td><td>scroll-margin-top: <i>&lt;float / 4&gt;</i>rem;<br>scroll-margin-bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-my-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;<br>scroll-margin-bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-my-px</td><td>scroll-margin-top: 1px;<br>scroll-margin-bottom: 1px;</td></tr>
    <tr><td>scroll-my-auto</td><td>scroll-margin-top: auto;<br>scroll-margin-bottom: auto;</td></tr>
    <tr><td>scroll-ms-<i>&lt;float&gt;</i></td><td>scroll-margin-inline-start: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-ms-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-inline-start: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-ms-px</td><td>scroll-margin-inline-start: 1px;</td></tr>
    <tr><td>scroll-ms-auto</td><td>scroll-margin-inline-start: auto;</td></tr>
    <tr><td>scroll-me-<i>&lt;float&gt;</i></td><td>scroll-margin-inline-end: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-me-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-inline-end: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-me-px</td><td>scroll-margin-inline-end: 1px;</td></tr>
    <tr><td>scroll-me-auto</td><td>scroll-margin-inline-end: auto;</td></tr>
    <tr><td>scroll-mt-<i>&lt;float&gt;</i></td><td>scroll-margin-top: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-mt-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-mt-px</td><td>scroll-margin-top: 1px;</td></tr>
    <tr><td>scroll-mt-auto</td><td>scroll-margin-top: auto;</td></tr>
    <tr><td>scroll-mr-<i>&lt;float&gt;</i></td><td>scroll-margin-right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-mr-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-mr-px</td><td>scroll-margin-right: 1px;</td></tr>
    <tr><td>scroll-mr-auto</td><td>scroll-margin-right: auto;</td></tr>
    <tr><td>scroll-mb-<i>&lt;float&gt;</i></td><td>scroll-margin-bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-mb-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-mb-px</td><td>scroll-margin-bottom: 1px;</td></tr>
    <tr><td>scroll-mb-auto</td><td>scroll-margin-bottom: auto;</td></tr>
    <tr><td>scroll-ml-<i>&lt;float&gt;</i></td><td>scroll-margin-left: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-ml-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-margin-left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-ml-px</td><td>scroll-margin-left: 1px;</td></tr>
    <tr><td>scroll-ml-auto</td><td>scroll-margin-left: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Scroll margin values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `scroll-mx-[1.42rem]`.

### Negative values

This plugin supports negative values. For example, `-scroll-mt-2` or `hover:-scroll-mt-2`.

[Tailwind reference](https://tailwindcss.com/docs/scroll-margin)
