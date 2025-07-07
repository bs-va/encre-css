Utilities for controlling an element's scroll offset within a snap container.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>scroll-p-<i>&lt;float&gt;</i></td><td>scroll-padding: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-p-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-p-px</td><td>scroll-padding: 1px;</td></tr>
    <tr><td>scroll-p-auto</td><td>scroll-padding: auto;</td></tr>
    <tr><td>scroll-px-<i>&lt;float&gt;</i></td><td>scroll-padding-inline: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-px-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-inline: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-px-px</td><td>scroll-padding-inline: 1px;</td></tr>
    <tr><td>scroll-px-auto</td><td>scroll-padding-inline: auto;</td></tr>
    <tr><td>scroll-py-<i>&lt;float&gt;</i></td><td>scroll-padding-block: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-py-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-block: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-py-px</td><td>scroll-padding-block: 1px;</td></tr>
    <tr><td>scroll-py-auto</td><td>scroll-padding-block: auto;</td></tr>
    <tr><td>scroll-ps-<i>&lt;float&gt;</i></td><td>scroll-padding-inline-start: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-ps-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-inline-start: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-ps-px</td><td>scroll-padding-inline-start: 1px;</td></tr>
    <tr><td>scroll-ps-auto</td><td>scroll-padding-inline-start: auto;</td></tr>
    <tr><td>scroll-pe-<i>&lt;float&gt;</i></td><td>scroll-padding-inline-end: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-pe-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-inline-end: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-pe-px</td><td>scroll-padding-inline-end: 1px;</td></tr>
    <tr><td>scroll-pe-auto</td><td>scroll-padding-inline-end: auto;</td></tr>
    <tr><td>scroll-pt-<i>&lt;float&gt;</i></td><td>scroll-padding-top: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-pt-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-pt-px</td><td>scroll-padding-top: 1px;</td></tr>
    <tr><td>scroll-pt-auto</td><td>scroll-padding-top: auto;</td></tr>
    <tr><td>scroll-pr-<i>&lt;float&gt;</i></td><td>scroll-padding-right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-pr-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-pr-px</td><td>scroll-padding-right: 1px;</td></tr>
    <tr><td>scroll-pr-auto</td><td>scroll-padding-right: auto;</td></tr>
    <tr><td>scroll-pb-<i>&lt;float&gt;</i></td><td>scroll-padding-bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-pb-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-pb-px</td><td>scroll-padding-bottom: 1px;</td></tr>
    <tr><td>scroll-pb-auto</td><td>scroll-padding-bottom: auto;</td></tr>
    <tr><td>scroll-pl-<i>&lt;float&gt;</i></td><td>scroll-padding-left: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>scroll-pl-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>scroll-padding-left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>scroll-pl-px</td><td>scroll-padding-left: 1px;</td></tr>
    <tr><td>scroll-pl-auto</td><td>scroll-padding-left: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Scroll padding values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `scroll-py-[1cm]`.

[Tailwind reference](https://tailwindcss.com/docs/scroll-padding)
