Utilities for controlling an element's margin.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>m-<i>&lt;float&gt;</i></td><td>margin: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>m-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>m-px</td><td>margin: 1px;</td></tr>
    <tr><td>m-auto</td><td>margin: auto;</td></tr>
    <tr><td>mx-<i>&lt;float&gt;</i></td><td>margin-left: <i>&lt;float / 4&gt;</i>rem;<br>margin-right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>mx-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;<br>margin-right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>mx-px</td><td>margin-left: 1px;<br>margin-right: 1px;</td></tr>
    <tr><td>mx-auto</td><td>margin-left: auto;<br>margin-right: auto;</td></tr>
    <tr><td>my-<i>&lt;float&gt;</i></td><td>margin-top: <i>&lt;float / 4&gt;</i>rem;<br>margin-bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>my-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;<br>margin-bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>my-px</td><td>margin-top: 1px;<br>margin-bottom: 1px;</td></tr>
    <tr><td>my-auto</td><td>margin-top: auto;<br>margin-bottom: auto;</td></tr>
    <tr><td>ms-<i>&lt;float&gt;</i></td><td>margin-inline-start: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>ms-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-inline-start: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>ms-px</td><td>margin-inline-start: 1px;</td></tr>
    <tr><td>ms-auto</td><td>margin-inline-start: auto;</td></tr>
    <tr><td>me-<i>&lt;float&gt;</i></td><td>margin-inline-end: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>me-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-inline-end: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>me-px</td><td>margin-inline-end: 1px;</td></tr>
    <tr><td>me-auto</td><td>margin-inline-end: auto;</td></tr>
    <tr><td>mt-<i>&lt;float&gt;</i></td><td>margin-top: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>mt-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>mt-px</td><td>margin-top: 1px;</td></tr>
    <tr><td>mt-auto</td><td>margin-top: auto;</td></tr>
    <tr><td>mr-<i>&lt;float&gt;</i></td><td>margin-right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>mr-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>mr-px</td><td>margin-right: 1px;</td></tr>
    <tr><td>mr-auto</td><td>margin-right: auto;</td></tr>
    <tr><td>mb-<i>&lt;float&gt;</i></td><td>margin-bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>mb-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>mb-px</td><td>margin-bottom: 1px;</td></tr>
    <tr><td>mb-auto</td><td>margin-bottom: auto;</td></tr>
    <tr><td>ml-<i>&lt;float&gt;</i></td><td>margin-left: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>ml-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>margin-left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>ml-px</td><td>margin-left: 1px;</td></tr>
    <tr><td>ml-auto</td><td>margin-left: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Margin values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `mt-[5vw]`.

### Negative values

This plugin supports negative values. For example, `-mt-2` or `hover:-mt-2`.

[Tailwind reference](https://tailwindcss.com/docs/margin)
