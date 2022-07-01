Utilities for animating elements with CSS animations.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>animate-none</td><td>animation: none;</td></tr>
    <tr><td>animate-spin</td><td style="white-space: pre;">animation: spin 1s linear infinite;<br><br>@keyframes spin {<br>  from {<br>  transform: rotate(0deg);<br>  }<br>  to {<br>  transform: rotate(360deg);<br>  }<br>}<br></td></tr>
    <tr><td>animate-ping</td><td style="white-space: pre;">animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;<br><br>@keyframes ping {<br>  75%, 100% {<br>  transform: scale(2);<br>  opacity: 0;<br>  }<br>}<br></td></tr>
    <tr><td>animate-pulse</td><td style="white-space: pre;">animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;<br><br>@keyframes pulse {<br>  0%, 100% {<br>  opacity: 1;<br>  }<br>  50% {<br>  opacity: .5;<br>  }<br>}<br></td></tr>
    <tr><td>animate-bounce</td><td style="white-space: pre;">animation: bounce 1s infinite;<br><br>@keyframes bounce {<br>  0%, 100% {<br>  transform: translateY(-25%);<br>  animation-timing-function: cubic-bezier(0.8, 0, 1, 1);<br>  }<br>  50% {<br>  transform: translateY(0);<br>  animation-timing-function: cubic-bezier(0, 0, 0.2, 1);<br>  }<br>}<br></td></tr>
  </tbody>
</table>

### Arbitrary values

Any property is allowed as arbitrary value.
For example, `animate-[3s_ease-in_1s_2_reverse_both_paused_slidein]`.

[Tailwind reference](https://tailwindcss.com/docs/animation)
