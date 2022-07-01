Utilities for styling the stroke of SVG elements.

<style>
#stroke-color-table > tr td:nth-child(3) svg {
  width: 100%;
  fill: transparent;
}
</style>

<svg style="display: none;" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <symbol viewBox="0 0 32 32" id="svg-logo"><path d="M30 23h-6a2 2 0 0 1-2-2V11a2 2 0 0 1 2-2h6v2h-6v10h4v-4h-2v-2h4zM18 9l-2 13l-2-13h-2l2.52 14h2.96L20 9h-2zM8 23H2v-2h6v-4H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h6v2H4v4h4a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2z"></path></symbol>
  </defs>
</svg>

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center; width: 7.5rem;">Color</th>
    </tr>
  </thead>
  <tbody id="stroke-color-table">
    <tr><td>stroke-inherit</td><td>stroke: inherit;</td><td><svg style="stroke: inherit;"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-current</td><td>stroke: currentColor;</td><td><svg style="stroke: currentColor;"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-transparent</td><td>stroke: transparent;</td><td><svg style="stroke: transparent;"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-black</td><td>stroke: rgb(0 0 0);</td><td><svg style="stroke: rgb(0 0 0);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-white</td><td>stroke: rgb(255 255 255);</td><td><svg style="stroke: rgb(255 255 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-50</td><td>stroke: rgb(248 250 252);</td><td><svg style="stroke: rgb(248 250 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-100</td><td>stroke: rgb(241 245 249);</td><td><svg style="stroke: rgb(241 245 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-200</td><td>stroke: rgb(226 232 240);</td><td><svg style="stroke: rgb(226 232 240);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-300</td><td>stroke: rgb(203 213 225);</td><td><svg style="stroke: rgb(203 213 225);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-400</td><td>stroke: rgb(148 163 184);</td><td><svg style="stroke: rgb(148 163 184);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-500</td><td>stroke: rgb(100 116 139);</td><td><svg style="stroke: rgb(100 116 139);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-600</td><td>stroke: rgb(71 85 105);</td><td><svg style="stroke: rgb(71 85 105);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-700</td><td>stroke: rgb(51 65 85);</td><td><svg style="stroke: rgb(51 65 85);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-800</td><td>stroke: rgb(30 41 59);</td><td><svg style="stroke: rgb(30 41 59);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-slate-900</td><td>stroke: rgb(15 23 42);</td><td><svg style="stroke: rgb(15 23 42);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-50</td><td>stroke: rgb(249 250 251);</td><td><svg style="stroke: rgb(249 250 251);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-100</td><td>stroke: rgb(243 244 246);</td><td><svg style="stroke: rgb(243 244 246);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-200</td><td>stroke: rgb(229 231 235);</td><td><svg style="stroke: rgb(229 231 235);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-300</td><td>stroke: rgb(209 213 219);</td><td><svg style="stroke: rgb(209 213 219);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-400</td><td>stroke: rgb(156 163 175);</td><td><svg style="stroke: rgb(156 163 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-500</td><td>stroke: rgb(107 114 128);</td><td><svg style="stroke: rgb(107 114 128);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-600</td><td>stroke: rgb(75 85 99);</td><td><svg style="stroke: rgb(75 85 99);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-700</td><td>stroke: rgb(55 65 81);</td><td><svg style="stroke: rgb(55 65 81);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-800</td><td>stroke: rgb(31 41 55);</td><td><svg style="stroke: rgb(31 41 55);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-gray-900</td><td>stroke: rgb(17 24 39);</td><td><svg style="stroke: rgb(17 24 39);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-50</td><td>stroke: rgb(250 250 250);</td><td><svg style="stroke: rgb(250 250 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-100</td><td>stroke: rgb(244 244 245);</td><td><svg style="stroke: rgb(244 244 245);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-200</td><td>stroke: rgb(228 228 231);</td><td><svg style="stroke: rgb(228 228 231);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-300</td><td>stroke: rgb(212 212 216);</td><td><svg style="stroke: rgb(212 212 216);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-400</td><td>stroke: rgb(161 161 170);</td><td><svg style="stroke: rgb(161 161 170);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-500</td><td>stroke: rgb(113 113 122);</td><td><svg style="stroke: rgb(113 113 122);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-600</td><td>stroke: rgb(82 82 91);</td><td><svg style="stroke: rgb(82 82 91);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-700</td><td>stroke: rgb(63 63 70);</td><td><svg style="stroke: rgb(63 63 70);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-800</td><td>stroke: rgb(39 39 42);</td><td><svg style="stroke: rgb(39 39 42);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-zinc-900</td><td>stroke: rgb(24 24 27);</td><td><svg style="stroke: rgb(24 24 27);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-50</td><td>stroke: rgb(250 250 250);</td><td><svg style="stroke: rgb(250 250 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-100</td><td>stroke: rgb(245 245 245);</td><td><svg style="stroke: rgb(245 245 245);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-200</td><td>stroke: rgb(229 229 229);</td><td><svg style="stroke: rgb(229 229 229);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-300</td><td>stroke: rgb(212 212 212);</td><td><svg style="stroke: rgb(212 212 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-400</td><td>stroke: rgb(163 163 163);</td><td><svg style="stroke: rgb(163 163 163);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-500</td><td>stroke: rgb(115 115 115);</td><td><svg style="stroke: rgb(115 115 115);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-600</td><td>stroke: rgb(82 82 82);</td><td><svg style="stroke: rgb(82 82 82);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-700</td><td>stroke: rgb(64 64 64);</td><td><svg style="stroke: rgb(64 64 64);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-800</td><td>stroke: rgb(38 38 38);</td><td><svg style="stroke: rgb(38 38 38);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-neutral-900</td><td>stroke: rgb(23 23 23);</td><td><svg style="stroke: rgb(23 23 23);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-50</td><td>stroke: rgb(250 250 249);</td><td><svg style="stroke: rgb(250 250 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-100</td><td>stroke: rgb(245 245 244);</td><td><svg style="stroke: rgb(245 245 244);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-200</td><td>stroke: rgb(231 229 228);</td><td><svg style="stroke: rgb(231 229 228);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-300</td><td>stroke: rgb(214 211 209);</td><td><svg style="stroke: rgb(214 211 209);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-400</td><td>stroke: rgb(168 162 158);</td><td><svg style="stroke: rgb(168 162 158);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-500</td><td>stroke: rgb(120 113 108);</td><td><svg style="stroke: rgb(120 113 108);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-600</td><td>stroke: rgb(87 83 78);</td><td><svg style="stroke: rgb(87 83 78);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-700</td><td>stroke: rgb(68 64 60);</td><td><svg style="stroke: rgb(68 64 60);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-800</td><td>stroke: rgb(41 37 36);</td><td><svg style="stroke: rgb(41 37 36);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-stone-900</td><td>stroke: rgb(28 25 23);</td><td><svg style="stroke: rgb(28 25 23);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-50</td><td>stroke: rgb(254 242 242);</td><td><svg style="stroke: rgb(254 242 242);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-100</td><td>stroke: rgb(254 226 226);</td><td><svg style="stroke: rgb(254 226 226);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-200</td><td>stroke: rgb(254 202 202);</td><td><svg style="stroke: rgb(254 202 202);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-300</td><td>stroke: rgb(252 165 165);</td><td><svg style="stroke: rgb(252 165 165);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-400</td><td>stroke: rgb(248 113 113);</td><td><svg style="stroke: rgb(248 113 113);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-500</td><td>stroke: rgb(239 68 68);</td><td><svg style="stroke: rgb(239 68 68);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-600</td><td>stroke: rgb(220 38 38);</td><td><svg style="stroke: rgb(220 38 38);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-700</td><td>stroke: rgb(185 28 28);</td><td><svg style="stroke: rgb(185 28 28);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-800</td><td>stroke: rgb(153 27 27);</td><td><svg style="stroke: rgb(153 27 27);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-red-900</td><td>stroke: rgb(127 29 29);</td><td><svg style="stroke: rgb(127 29 29);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-50</td><td>stroke: rgb(255 247 237);</td><td><svg style="stroke: rgb(255 247 237);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-100</td><td>stroke: rgb(255 237 213);</td><td><svg style="stroke: rgb(255 237 213);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-200</td><td>stroke: rgb(254 215 170);</td><td><svg style="stroke: rgb(254 215 170);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-300</td><td>stroke: rgb(253 186 116);</td><td><svg style="stroke: rgb(253 186 116);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-400</td><td>stroke: rgb(251 146 60);</td><td><svg style="stroke: rgb(251 146 60);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-500</td><td>stroke: rgb(249 115 22);</td><td><svg style="stroke: rgb(249 115 22);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-600</td><td>stroke: rgb(234 88 12);</td><td><svg style="stroke: rgb(234 88 12);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-700</td><td>stroke: rgb(194 65 12);</td><td><svg style="stroke: rgb(194 65 12);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-800</td><td>stroke: rgb(154 52 18);</td><td><svg style="stroke: rgb(154 52 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-orange-900</td><td>stroke: rgb(124 45 18);</td><td><svg style="stroke: rgb(124 45 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-50</td><td>stroke: rgb(255 251 235);</td><td><svg style="stroke: rgb(255 251 235);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-100</td><td>stroke: rgb(254 243 199);</td><td><svg style="stroke: rgb(254 243 199);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-200</td><td>stroke: rgb(253 230 138);</td><td><svg style="stroke: rgb(253 230 138);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-300</td><td>stroke: rgb(252 211 77);</td><td><svg style="stroke: rgb(252 211 77);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-400</td><td>stroke: rgb(251 191 36);</td><td><svg style="stroke: rgb(251 191 36);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-500</td><td>stroke: rgb(245 158 11);</td><td><svg style="stroke: rgb(245 158 11);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-600</td><td>stroke: rgb(217 119 6);</td><td><svg style="stroke: rgb(217 119 6);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-700</td><td>stroke: rgb(180 83 9);</td><td><svg style="stroke: rgb(180 83 9);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-800</td><td>stroke: rgb(146 64 14);</td><td><svg style="stroke: rgb(146 64 14);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-amber-900</td><td>stroke: rgb(120 53 15);</td><td><svg style="stroke: rgb(120 53 15);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-50</td><td>stroke: rgb(254 252 232);</td><td><svg style="stroke: rgb(254 252 232);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-100</td><td>stroke: rgb(254 249 195);</td><td><svg style="stroke: rgb(254 249 195);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-200</td><td>stroke: rgb(254 240 138);</td><td><svg style="stroke: rgb(254 240 138);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-300</td><td>stroke: rgb(253 224 71);</td><td><svg style="stroke: rgb(253 224 71);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-400</td><td>stroke: rgb(250 204 21);</td><td><svg style="stroke: rgb(250 204 21);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-500</td><td>stroke: rgb(234 179 8);</td><td><svg style="stroke: rgb(234 179 8);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-600</td><td>stroke: rgb(202 138 4);</td><td><svg style="stroke: rgb(202 138 4);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-700</td><td>stroke: rgb(161 98 7);</td><td><svg style="stroke: rgb(161 98 7);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-800</td><td>stroke: rgb(133 77 14);</td><td><svg style="stroke: rgb(133 77 14);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-yellow-900</td><td>stroke: rgb(113 63 18);</td><td><svg style="stroke: rgb(113 63 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-50</td><td>stroke: rgb(247 254 231);</td><td><svg style="stroke: rgb(247 254 231);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-100</td><td>stroke: rgb(236 252 203);</td><td><svg style="stroke: rgb(236 252 203);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-200</td><td>stroke: rgb(217 249 157);</td><td><svg style="stroke: rgb(217 249 157);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-300</td><td>stroke: rgb(190 242 100);</td><td><svg style="stroke: rgb(190 242 100);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-400</td><td>stroke: rgb(163 230 53);</td><td><svg style="stroke: rgb(163 230 53);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-500</td><td>stroke: rgb(132 204 22);</td><td><svg style="stroke: rgb(132 204 22);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-600</td><td>stroke: rgb(101 163 13);</td><td><svg style="stroke: rgb(101 163 13);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-700</td><td>stroke: rgb(77 124 15);</td><td><svg style="stroke: rgb(77 124 15);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-800</td><td>stroke: rgb(63 98 18);</td><td><svg style="stroke: rgb(63 98 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-lime-900</td><td>stroke: rgb(54 83 20);</td><td><svg style="stroke: rgb(54 83 20);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-50</td><td>stroke: rgb(240 253 244);</td><td><svg style="stroke: rgb(240 253 244);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-100</td><td>stroke: rgb(220 252 231);</td><td><svg style="stroke: rgb(220 252 231);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-200</td><td>stroke: rgb(187 247 208);</td><td><svg style="stroke: rgb(187 247 208);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-300</td><td>stroke: rgb(134 239 172);</td><td><svg style="stroke: rgb(134 239 172);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-400</td><td>stroke: rgb(74 222 128);</td><td><svg style="stroke: rgb(74 222 128);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-500</td><td>stroke: rgb(34 197 94);</td><td><svg style="stroke: rgb(34 197 94);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-600</td><td>stroke: rgb(22 163 74);</td><td><svg style="stroke: rgb(22 163 74);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-700</td><td>stroke: rgb(21 128 61);</td><td><svg style="stroke: rgb(21 128 61);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-800</td><td>stroke: rgb(22 101 52);</td><td><svg style="stroke: rgb(22 101 52);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-green-900</td><td>stroke: rgb(20 83 45);</td><td><svg style="stroke: rgb(20 83 45);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-50</td><td>stroke: rgb(236 253 245);</td><td><svg style="stroke: rgb(236 253 245);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-100</td><td>stroke: rgb(209 250 229);</td><td><svg style="stroke: rgb(209 250 229);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-200</td><td>stroke: rgb(167 243 208);</td><td><svg style="stroke: rgb(167 243 208);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-300</td><td>stroke: rgb(110 231 183);</td><td><svg style="stroke: rgb(110 231 183);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-400</td><td>stroke: rgb(52 211 153);</td><td><svg style="stroke: rgb(52 211 153);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-500</td><td>stroke: rgb(16 185 129);</td><td><svg style="stroke: rgb(16 185 129);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-600</td><td>stroke: rgb(5 150 105);</td><td><svg style="stroke: rgb(5 150 105);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-700</td><td>stroke: rgb(4 120 87);</td><td><svg style="stroke: rgb(4 120 87);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-800</td><td>stroke: rgb(6 95 70);</td><td><svg style="stroke: rgb(6 95 70);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-emerald-900</td><td>stroke: rgb(6 78 59);</td><td><svg style="stroke: rgb(6 78 59);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-50</td><td>stroke: rgb(240 253 250);</td><td><svg style="stroke: rgb(240 253 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-100</td><td>stroke: rgb(204 251 241);</td><td><svg style="stroke: rgb(204 251 241);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-200</td><td>stroke: rgb(153 246 228);</td><td><svg style="stroke: rgb(153 246 228);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-300</td><td>stroke: rgb(94 234 212);</td><td><svg style="stroke: rgb(94 234 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-400</td><td>stroke: rgb(45 212 191);</td><td><svg style="stroke: rgb(45 212 191);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-500</td><td>stroke: rgb(20 184 166);</td><td><svg style="stroke: rgb(20 184 166);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-600</td><td>stroke: rgb(13 148 136);</td><td><svg style="stroke: rgb(13 148 136);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-700</td><td>stroke: rgb(15 118 110);</td><td><svg style="stroke: rgb(15 118 110);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-800</td><td>stroke: rgb(17 94 89);</td><td><svg style="stroke: rgb(17 94 89);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-teal-900</td><td>stroke: rgb(19 78 74);</td><td><svg style="stroke: rgb(19 78 74);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-50</td><td>stroke: rgb(236 254 255);</td><td><svg style="stroke: rgb(236 254 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-100</td><td>stroke: rgb(207 250 254);</td><td><svg style="stroke: rgb(207 250 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-200</td><td>stroke: rgb(165 243 252);</td><td><svg style="stroke: rgb(165 243 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-300</td><td>stroke: rgb(103 232 249);</td><td><svg style="stroke: rgb(103 232 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-400</td><td>stroke: rgb(34 211 238);</td><td><svg style="stroke: rgb(34 211 238);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-500</td><td>stroke: rgb(6 182 212);</td><td><svg style="stroke: rgb(6 182 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-600</td><td>stroke: rgb(8 145 178);</td><td><svg style="stroke: rgb(8 145 178);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-700</td><td>stroke: rgb(14 116 144);</td><td><svg style="stroke: rgb(14 116 144);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-800</td><td>stroke: rgb(21 94 117);</td><td><svg style="stroke: rgb(21 94 117);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-cyan-900</td><td>stroke: rgb(22 78 99);</td><td><svg style="stroke: rgb(22 78 99);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-50</td><td>stroke: rgb(240 249 255);</td><td><svg style="stroke: rgb(240 249 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-100</td><td>stroke: rgb(224 242 254);</td><td><svg style="stroke: rgb(224 242 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-200</td><td>stroke: rgb(186 230 253);</td><td><svg style="stroke: rgb(186 230 253);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-300</td><td>stroke: rgb(125 211 252);</td><td><svg style="stroke: rgb(125 211 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-400</td><td>stroke: rgb(56 189 248);</td><td><svg style="stroke: rgb(56 189 248);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-500</td><td>stroke: rgb(14 165 233);</td><td><svg style="stroke: rgb(14 165 233);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-600</td><td>stroke: rgb(2 132 199);</td><td><svg style="stroke: rgb(2 132 199);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-700</td><td>stroke: rgb(3 105 161);</td><td><svg style="stroke: rgb(3 105 161);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-800</td><td>stroke: rgb(7 89 133);</td><td><svg style="stroke: rgb(7 89 133);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-sky-900</td><td>stroke: rgb(12 74 110);</td><td><svg style="stroke: rgb(12 74 110);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-50</td><td>stroke: rgb(239 246 255);</td><td><svg style="stroke: rgb(239 246 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-100</td><td>stroke: rgb(219 234 254);</td><td><svg style="stroke: rgb(219 234 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-200</td><td>stroke: rgb(191 219 254);</td><td><svg style="stroke: rgb(191 219 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-300</td><td>stroke: rgb(147 197 253);</td><td><svg style="stroke: rgb(147 197 253);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-400</td><td>stroke: rgb(96 165 250);</td><td><svg style="stroke: rgb(96 165 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-500</td><td>stroke: rgb(59 130 246);</td><td><svg style="stroke: rgb(59 130 246);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-600</td><td>stroke: rgb(37 99 235);</td><td><svg style="stroke: rgb(37 99 235);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-700</td><td>stroke: rgb(29 78 216);</td><td><svg style="stroke: rgb(29 78 216);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-800</td><td>stroke: rgb(30 64 175);</td><td><svg style="stroke: rgb(30 64 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-blue-900</td><td>stroke: rgb(30 58 138);</td><td><svg style="stroke: rgb(30 58 138);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-50</td><td>stroke: rgb(238 242 255);</td><td><svg style="stroke: rgb(238 242 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-100</td><td>stroke: rgb(224 231 255);</td><td><svg style="stroke: rgb(224 231 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-200</td><td>stroke: rgb(199 210 254);</td><td><svg style="stroke: rgb(199 210 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-300</td><td>stroke: rgb(165 180 252);</td><td><svg style="stroke: rgb(165 180 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-400</td><td>stroke: rgb(129 140 248);</td><td><svg style="stroke: rgb(129 140 248);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-500</td><td>stroke: rgb(99 102 241);</td><td><svg style="stroke: rgb(99 102 241);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-600</td><td>stroke: rgb(79 70 229);</td><td><svg style="stroke: rgb(79 70 229);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-700</td><td>stroke: rgb(67 56 202);</td><td><svg style="stroke: rgb(67 56 202);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-800</td><td>stroke: rgb(55 48 163);</td><td><svg style="stroke: rgb(55 48 163);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-indigo-900</td><td>stroke: rgb(49 46 129);</td><td><svg style="stroke: rgb(49 46 129);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-50</td><td>stroke: rgb(245 243 255);</td><td><svg style="stroke: rgb(245 243 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-100</td><td>stroke: rgb(237 233 254);</td><td><svg style="stroke: rgb(237 233 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-200</td><td>stroke: rgb(221 214 254);</td><td><svg style="stroke: rgb(221 214 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-300</td><td>stroke: rgb(196 181 253);</td><td><svg style="stroke: rgb(196 181 253);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-400</td><td>stroke: rgb(167 139 250);</td><td><svg style="stroke: rgb(167 139 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-500</td><td>stroke: rgb(139 92 246);</td><td><svg style="stroke: rgb(139 92 246);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-600</td><td>stroke: rgb(124 58 237);</td><td><svg style="stroke: rgb(124 58 237);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-700</td><td>stroke: rgb(109 40 217);</td><td><svg style="stroke: rgb(109 40 217);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-800</td><td>stroke: rgb(91 33 182);</td><td><svg style="stroke: rgb(91 33 182);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-violet-900</td><td>stroke: rgb(76 29 149);</td><td><svg style="stroke: rgb(76 29 149);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-50</td><td>stroke: rgb(250 245 255);</td><td><svg style="stroke: rgb(250 245 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-100</td><td>stroke: rgb(243 232 255);</td><td><svg style="stroke: rgb(243 232 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-200</td><td>stroke: rgb(233 213 255);</td><td><svg style="stroke: rgb(233 213 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-300</td><td>stroke: rgb(216 180 254);</td><td><svg style="stroke: rgb(216 180 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-400</td><td>stroke: rgb(192 132 252);</td><td><svg style="stroke: rgb(192 132 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-500</td><td>stroke: rgb(168 85 247);</td><td><svg style="stroke: rgb(168 85 247);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-600</td><td>stroke: rgb(147 51 234);</td><td><svg style="stroke: rgb(147 51 234);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-700</td><td>stroke: rgb(126 34 206);</td><td><svg style="stroke: rgb(126 34 206);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-800</td><td>stroke: rgb(107 33 168);</td><td><svg style="stroke: rgb(107 33 168);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-purple-900</td><td>stroke: rgb(88 28 135);</td><td><svg style="stroke: rgb(88 28 135);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-50</td><td>stroke: rgb(253 244 255);</td><td><svg style="stroke: rgb(253 244 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-100</td><td>stroke: rgb(250 232 255);</td><td><svg style="stroke: rgb(250 232 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-200</td><td>stroke: rgb(245 208 254);</td><td><svg style="stroke: rgb(245 208 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-300</td><td>stroke: rgb(240 171 252);</td><td><svg style="stroke: rgb(240 171 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-400</td><td>stroke: rgb(232 121 249);</td><td><svg style="stroke: rgb(232 121 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-500</td><td>stroke: rgb(217 70 239);</td><td><svg style="stroke: rgb(217 70 239);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-600</td><td>stroke: rgb(192 38 211);</td><td><svg style="stroke: rgb(192 38 211);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-700</td><td>stroke: rgb(162 28 175);</td><td><svg style="stroke: rgb(162 28 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-800</td><td>stroke: rgb(134 25 143);</td><td><svg style="stroke: rgb(134 25 143);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-fuchsia-900</td><td>stroke: rgb(112 26 117);</td><td><svg style="stroke: rgb(112 26 117);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-50</td><td>stroke: rgb(253 242 248);</td><td><svg style="stroke: rgb(253 242 248);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-100</td><td>stroke: rgb(252 231 243);</td><td><svg style="stroke: rgb(252 231 243);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-200</td><td>stroke: rgb(251 207 232);</td><td><svg style="stroke: rgb(251 207 232);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-300</td><td>stroke: rgb(249 168 212);</td><td><svg style="stroke: rgb(249 168 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-400</td><td>stroke: rgb(244 114 182);</td><td><svg style="stroke: rgb(244 114 182);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-500</td><td>stroke: rgb(236 72 153);</td><td><svg style="stroke: rgb(236 72 153);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-600</td><td>stroke: rgb(219 39 119);</td><td><svg style="stroke: rgb(219 39 119);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-700</td><td>stroke: rgb(190 24 93);</td><td><svg style="stroke: rgb(190 24 93);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-800</td><td>stroke: rgb(157 23 77);</td><td><svg style="stroke: rgb(157 23 77);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-pink-900</td><td>stroke: rgb(131 24 67);</td><td><svg style="stroke: rgb(131 24 67);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-50</td><td>stroke: rgb(255 241 242);</td><td><svg style="stroke: rgb(255 241 242);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-100</td><td>stroke: rgb(255 228 230);</td><td><svg style="stroke: rgb(255 228 230);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-200</td><td>stroke: rgb(254 205 211);</td><td><svg style="stroke: rgb(254 205 211);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-300</td><td>stroke: rgb(253 164 175);</td><td><svg style="stroke: rgb(253 164 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-400</td><td>stroke: rgb(251 113 133);</td><td><svg style="stroke: rgb(251 113 133);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-500</td><td>stroke: rgb(244 63 94);</td><td><svg style="stroke: rgb(244 63 94);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-600</td><td>stroke: rgb(225 29 72);</td><td><svg style="stroke: rgb(225 29 72);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-700</td><td>stroke: rgb(190 18 60);</td><td><svg style="stroke: rgb(190 18 60);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-800</td><td>stroke: rgb(159 18 57);</td><td><svg style="stroke: rgb(159 18 57);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>stroke-rose-900</td><td>stroke: rgb(136 19 55);</td><td><svg style="stroke: rgb(136 19 55);"><use href="#svg-logo" /></svg></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<color>`](crate::utils::value_matchers::is_matching_color) property is allowed as arbitrary value.
For example, `stroke-[rgb(33,45,42)]`.

[Tailwind reference](https://tailwindcss.com/docs/stroke)
