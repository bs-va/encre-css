Utilities for improving accessibility with screen readers.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>sr-only</td>
      <td>
        position: absolute;<br>
        width: 1px;<br>
        height: 1px;<br>
        padding: 0;<br>
        margin: -1px;<br>
        overflow: hidden;<br>
        clip: rect(0, 0, 0, 0);<br>
        white-space: nowrap;<br>
        border-width: 0;<br>
      </td>
    </tr>
    <tr>
      <td>not-sr-only</td>
      <td>
        position: static;<br>
        width: auto;<br>
        height: auto;<br>
        padding: 0;<br>
        margin: 0;<br>
        overflow: visible;<br>
        clip: auto;<br>
        white-space: normal;<br>
      </td>
    </tr>
  </tbody>
</table>

[Tailwind reference](https://tailwindcss.com/docs/screen-readers)
