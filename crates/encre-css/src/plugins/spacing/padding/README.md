Utilities for controlling an element's padding.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>p-<i>&lt;float&gt;</i></td><td>padding: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>p-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>p-px</td><td>padding: 1px;</td></tr>
    <tr><td>p-auto</td><td>padding: auto;</td></tr>
    <tr><td>px-<i>&lt;float&gt;</i></td><td>padding-inline: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>px-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-inline: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>px-px</td><td>padding-inline: 1px;</td></tr>
    <tr><td>px-auto</td><td>padding-inline: auto;</td></tr>
    <tr><td>py-<i>&lt;float&gt;</i></td><td>padding-block: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>py-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-block: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>py-px</td><td>padding-block: 1px;</td></tr>
    <tr><td>py-auto</td><td>padding-block: auto;</td></tr>
    <tr><td>ps-<i>&lt;float&gt;</i></td><td>padding-inline-start: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>ps-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-inline-start: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>ps-px</td><td>padding-inline-start: 1px;</td></tr>
    <tr><td>ps-auto</td><td>padding-inline-start: auto;</td></tr>
    <tr><td>pe-<i>&lt;float&gt;</i></td><td>padding-inline-end: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>pe-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-inline-end: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>pe-px</td><td>padding-inline-end: 1px;</td></tr>
    <tr><td>pe-auto</td><td>padding-inline-end: auto;</td></tr>
    <tr><td>pt-<i>&lt;float&gt;</i></td><td>padding-top: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>pt-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-top: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>pt-px</td><td>padding-top: 1px;</td></tr>
    <tr><td>pt-auto</td><td>padding-top: auto;</td></tr>
    <tr><td>pr-<i>&lt;float&gt;</i></td><td>padding-right: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>pr-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-right: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>pr-px</td><td>padding-right: 1px;</td></tr>
    <tr><td>pr-auto</td><td>padding-right: auto;</td></tr>
    <tr><td>pb-<i>&lt;float&gt;</i></td><td>padding-bottom: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>pb-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-bottom: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>pb-px</td><td>padding-bottom: 1px;</td></tr>
    <tr><td>pb-auto</td><td>padding-bottom: auto;</td></tr>
    <tr><td>pl-<i>&lt;float&gt;</i></td><td>padding-left: <i>&lt;float / 4&gt;</i>rem;</td></tr>
    <tr><td>pl-<i>&lt;integer&gt;/&lt;integer&gt;</i></td><td>padding-left: (<i>&lt;integer&gt;/&lt;integer&gt;</i>)%;</td></tr>
    <tr><td>pl-px</td><td>padding-left: 1px;</td></tr>
    <tr><td>pl-auto</td><td>padding-left: auto;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Padding values don't follow Tailwind's philosophy of limiting possible values and all
spacing values are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) property is allowed as arbitrary value.
For example, `pb-[2pt]`.

[Tailwind reference](https://tailwindcss.com/docs/padding)
