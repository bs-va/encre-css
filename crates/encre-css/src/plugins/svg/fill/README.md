Utilities for styling the fill of SVG elements.

<style>
#fill-color-table > tr td:nth-child(3) svg {
  width: 100%;
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
  <tbody id="fill-color-table">
    <tr><td>fill-inherit</td><td>fill: inherit;</td><td><svg style="fill: inherit;"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-current</td><td>fill: currentColor;</td><td><svg style="fill: currentColor;"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-transparent</td><td>fill: transparent;</td><td><svg style="fill: transparent;"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-black</td><td>fill: rgb(0 0 0);</td><td><svg style="fill: rgb(0 0 0);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-white</td><td>fill: rgb(255 255 255);</td><td><svg style="fill: rgb(255 255 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-50</td><td>fill: rgb(248 250 252);</td><td><svg style="fill: rgb(248 250 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-100</td><td>fill: rgb(241 245 249);</td><td><svg style="fill: rgb(241 245 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-200</td><td>fill: rgb(226 232 240);</td><td><svg style="fill: rgb(226 232 240);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-300</td><td>fill: rgb(203 213 225);</td><td><svg style="fill: rgb(203 213 225);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-400</td><td>fill: rgb(148 163 184);</td><td><svg style="fill: rgb(148 163 184);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-500</td><td>fill: rgb(100 116 139);</td><td><svg style="fill: rgb(100 116 139);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-600</td><td>fill: rgb(71 85 105);</td><td><svg style="fill: rgb(71 85 105);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-700</td><td>fill: rgb(51 65 85);</td><td><svg style="fill: rgb(51 65 85);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-800</td><td>fill: rgb(30 41 59);</td><td><svg style="fill: rgb(30 41 59);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-slate-900</td><td>fill: rgb(15 23 42);</td><td><svg style="fill: rgb(15 23 42);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-50</td><td>fill: rgb(249 250 251);</td><td><svg style="fill: rgb(249 250 251);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-100</td><td>fill: rgb(243 244 246);</td><td><svg style="fill: rgb(243 244 246);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-200</td><td>fill: rgb(229 231 235);</td><td><svg style="fill: rgb(229 231 235);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-300</td><td>fill: rgb(209 213 219);</td><td><svg style="fill: rgb(209 213 219);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-400</td><td>fill: rgb(156 163 175);</td><td><svg style="fill: rgb(156 163 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-500</td><td>fill: rgb(107 114 128);</td><td><svg style="fill: rgb(107 114 128);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-600</td><td>fill: rgb(75 85 99);</td><td><svg style="fill: rgb(75 85 99);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-700</td><td>fill: rgb(55 65 81);</td><td><svg style="fill: rgb(55 65 81);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-800</td><td>fill: rgb(31 41 55);</td><td><svg style="fill: rgb(31 41 55);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-gray-900</td><td>fill: rgb(17 24 39);</td><td><svg style="fill: rgb(17 24 39);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-50</td><td>fill: rgb(250 250 250);</td><td><svg style="fill: rgb(250 250 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-100</td><td>fill: rgb(244 244 245);</td><td><svg style="fill: rgb(244 244 245);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-200</td><td>fill: rgb(228 228 231);</td><td><svg style="fill: rgb(228 228 231);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-300</td><td>fill: rgb(212 212 216);</td><td><svg style="fill: rgb(212 212 216);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-400</td><td>fill: rgb(161 161 170);</td><td><svg style="fill: rgb(161 161 170);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-500</td><td>fill: rgb(113 113 122);</td><td><svg style="fill: rgb(113 113 122);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-600</td><td>fill: rgb(82 82 91);</td><td><svg style="fill: rgb(82 82 91);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-700</td><td>fill: rgb(63 63 70);</td><td><svg style="fill: rgb(63 63 70);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-800</td><td>fill: rgb(39 39 42);</td><td><svg style="fill: rgb(39 39 42);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-zinc-900</td><td>fill: rgb(24 24 27);</td><td><svg style="fill: rgb(24 24 27);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-50</td><td>fill: rgb(250 250 250);</td><td><svg style="fill: rgb(250 250 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-100</td><td>fill: rgb(245 245 245);</td><td><svg style="fill: rgb(245 245 245);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-200</td><td>fill: rgb(229 229 229);</td><td><svg style="fill: rgb(229 229 229);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-300</td><td>fill: rgb(212 212 212);</td><td><svg style="fill: rgb(212 212 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-400</td><td>fill: rgb(163 163 163);</td><td><svg style="fill: rgb(163 163 163);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-500</td><td>fill: rgb(115 115 115);</td><td><svg style="fill: rgb(115 115 115);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-600</td><td>fill: rgb(82 82 82);</td><td><svg style="fill: rgb(82 82 82);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-700</td><td>fill: rgb(64 64 64);</td><td><svg style="fill: rgb(64 64 64);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-800</td><td>fill: rgb(38 38 38);</td><td><svg style="fill: rgb(38 38 38);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-neutral-900</td><td>fill: rgb(23 23 23);</td><td><svg style="fill: rgb(23 23 23);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-50</td><td>fill: rgb(250 250 249);</td><td><svg style="fill: rgb(250 250 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-100</td><td>fill: rgb(245 245 244);</td><td><svg style="fill: rgb(245 245 244);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-200</td><td>fill: rgb(231 229 228);</td><td><svg style="fill: rgb(231 229 228);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-300</td><td>fill: rgb(214 211 209);</td><td><svg style="fill: rgb(214 211 209);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-400</td><td>fill: rgb(168 162 158);</td><td><svg style="fill: rgb(168 162 158);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-500</td><td>fill: rgb(120 113 108);</td><td><svg style="fill: rgb(120 113 108);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-600</td><td>fill: rgb(87 83 78);</td><td><svg style="fill: rgb(87 83 78);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-700</td><td>fill: rgb(68 64 60);</td><td><svg style="fill: rgb(68 64 60);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-800</td><td>fill: rgb(41 37 36);</td><td><svg style="fill: rgb(41 37 36);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-stone-900</td><td>fill: rgb(28 25 23);</td><td><svg style="fill: rgb(28 25 23);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-50</td><td>fill: rgb(254 242 242);</td><td><svg style="fill: rgb(254 242 242);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-100</td><td>fill: rgb(254 226 226);</td><td><svg style="fill: rgb(254 226 226);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-200</td><td>fill: rgb(254 202 202);</td><td><svg style="fill: rgb(254 202 202);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-300</td><td>fill: rgb(252 165 165);</td><td><svg style="fill: rgb(252 165 165);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-400</td><td>fill: rgb(248 113 113);</td><td><svg style="fill: rgb(248 113 113);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-500</td><td>fill: rgb(239 68 68);</td><td><svg style="fill: rgb(239 68 68);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-600</td><td>fill: rgb(220 38 38);</td><td><svg style="fill: rgb(220 38 38);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-700</td><td>fill: rgb(185 28 28);</td><td><svg style="fill: rgb(185 28 28);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-800</td><td>fill: rgb(153 27 27);</td><td><svg style="fill: rgb(153 27 27);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-red-900</td><td>fill: rgb(127 29 29);</td><td><svg style="fill: rgb(127 29 29);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-50</td><td>fill: rgb(255 247 237);</td><td><svg style="fill: rgb(255 247 237);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-100</td><td>fill: rgb(255 237 213);</td><td><svg style="fill: rgb(255 237 213);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-200</td><td>fill: rgb(254 215 170);</td><td><svg style="fill: rgb(254 215 170);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-300</td><td>fill: rgb(253 186 116);</td><td><svg style="fill: rgb(253 186 116);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-400</td><td>fill: rgb(251 146 60);</td><td><svg style="fill: rgb(251 146 60);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-500</td><td>fill: rgb(249 115 22);</td><td><svg style="fill: rgb(249 115 22);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-600</td><td>fill: rgb(234 88 12);</td><td><svg style="fill: rgb(234 88 12);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-700</td><td>fill: rgb(194 65 12);</td><td><svg style="fill: rgb(194 65 12);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-800</td><td>fill: rgb(154 52 18);</td><td><svg style="fill: rgb(154 52 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-orange-900</td><td>fill: rgb(124 45 18);</td><td><svg style="fill: rgb(124 45 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-50</td><td>fill: rgb(255 251 235);</td><td><svg style="fill: rgb(255 251 235);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-100</td><td>fill: rgb(254 243 199);</td><td><svg style="fill: rgb(254 243 199);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-200</td><td>fill: rgb(253 230 138);</td><td><svg style="fill: rgb(253 230 138);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-300</td><td>fill: rgb(252 211 77);</td><td><svg style="fill: rgb(252 211 77);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-400</td><td>fill: rgb(251 191 36);</td><td><svg style="fill: rgb(251 191 36);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-500</td><td>fill: rgb(245 158 11);</td><td><svg style="fill: rgb(245 158 11);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-600</td><td>fill: rgb(217 119 6);</td><td><svg style="fill: rgb(217 119 6);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-700</td><td>fill: rgb(180 83 9);</td><td><svg style="fill: rgb(180 83 9);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-800</td><td>fill: rgb(146 64 14);</td><td><svg style="fill: rgb(146 64 14);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-amber-900</td><td>fill: rgb(120 53 15);</td><td><svg style="fill: rgb(120 53 15);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-50</td><td>fill: rgb(254 252 232);</td><td><svg style="fill: rgb(254 252 232);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-100</td><td>fill: rgb(254 249 195);</td><td><svg style="fill: rgb(254 249 195);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-200</td><td>fill: rgb(254 240 138);</td><td><svg style="fill: rgb(254 240 138);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-300</td><td>fill: rgb(253 224 71);</td><td><svg style="fill: rgb(253 224 71);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-400</td><td>fill: rgb(250 204 21);</td><td><svg style="fill: rgb(250 204 21);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-500</td><td>fill: rgb(234 179 8);</td><td><svg style="fill: rgb(234 179 8);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-600</td><td>fill: rgb(202 138 4);</td><td><svg style="fill: rgb(202 138 4);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-700</td><td>fill: rgb(161 98 7);</td><td><svg style="fill: rgb(161 98 7);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-800</td><td>fill: rgb(133 77 14);</td><td><svg style="fill: rgb(133 77 14);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-yellow-900</td><td>fill: rgb(113 63 18);</td><td><svg style="fill: rgb(113 63 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-50</td><td>fill: rgb(247 254 231);</td><td><svg style="fill: rgb(247 254 231);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-100</td><td>fill: rgb(236 252 203);</td><td><svg style="fill: rgb(236 252 203);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-200</td><td>fill: rgb(217 249 157);</td><td><svg style="fill: rgb(217 249 157);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-300</td><td>fill: rgb(190 242 100);</td><td><svg style="fill: rgb(190 242 100);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-400</td><td>fill: rgb(163 230 53);</td><td><svg style="fill: rgb(163 230 53);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-500</td><td>fill: rgb(132 204 22);</td><td><svg style="fill: rgb(132 204 22);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-600</td><td>fill: rgb(101 163 13);</td><td><svg style="fill: rgb(101 163 13);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-700</td><td>fill: rgb(77 124 15);</td><td><svg style="fill: rgb(77 124 15);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-800</td><td>fill: rgb(63 98 18);</td><td><svg style="fill: rgb(63 98 18);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-lime-900</td><td>fill: rgb(54 83 20);</td><td><svg style="fill: rgb(54 83 20);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-50</td><td>fill: rgb(240 253 244);</td><td><svg style="fill: rgb(240 253 244);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-100</td><td>fill: rgb(220 252 231);</td><td><svg style="fill: rgb(220 252 231);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-200</td><td>fill: rgb(187 247 208);</td><td><svg style="fill: rgb(187 247 208);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-300</td><td>fill: rgb(134 239 172);</td><td><svg style="fill: rgb(134 239 172);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-400</td><td>fill: rgb(74 222 128);</td><td><svg style="fill: rgb(74 222 128);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-500</td><td>fill: rgb(34 197 94);</td><td><svg style="fill: rgb(34 197 94);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-600</td><td>fill: rgb(22 163 74);</td><td><svg style="fill: rgb(22 163 74);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-700</td><td>fill: rgb(21 128 61);</td><td><svg style="fill: rgb(21 128 61);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-800</td><td>fill: rgb(22 101 52);</td><td><svg style="fill: rgb(22 101 52);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-green-900</td><td>fill: rgb(20 83 45);</td><td><svg style="fill: rgb(20 83 45);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-50</td><td>fill: rgb(236 253 245);</td><td><svg style="fill: rgb(236 253 245);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-100</td><td>fill: rgb(209 250 229);</td><td><svg style="fill: rgb(209 250 229);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-200</td><td>fill: rgb(167 243 208);</td><td><svg style="fill: rgb(167 243 208);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-300</td><td>fill: rgb(110 231 183);</td><td><svg style="fill: rgb(110 231 183);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-400</td><td>fill: rgb(52 211 153);</td><td><svg style="fill: rgb(52 211 153);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-500</td><td>fill: rgb(16 185 129);</td><td><svg style="fill: rgb(16 185 129);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-600</td><td>fill: rgb(5 150 105);</td><td><svg style="fill: rgb(5 150 105);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-700</td><td>fill: rgb(4 120 87);</td><td><svg style="fill: rgb(4 120 87);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-800</td><td>fill: rgb(6 95 70);</td><td><svg style="fill: rgb(6 95 70);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-emerald-900</td><td>fill: rgb(6 78 59);</td><td><svg style="fill: rgb(6 78 59);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-50</td><td>fill: rgb(240 253 250);</td><td><svg style="fill: rgb(240 253 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-100</td><td>fill: rgb(204 251 241);</td><td><svg style="fill: rgb(204 251 241);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-200</td><td>fill: rgb(153 246 228);</td><td><svg style="fill: rgb(153 246 228);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-300</td><td>fill: rgb(94 234 212);</td><td><svg style="fill: rgb(94 234 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-400</td><td>fill: rgb(45 212 191);</td><td><svg style="fill: rgb(45 212 191);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-500</td><td>fill: rgb(20 184 166);</td><td><svg style="fill: rgb(20 184 166);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-600</td><td>fill: rgb(13 148 136);</td><td><svg style="fill: rgb(13 148 136);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-700</td><td>fill: rgb(15 118 110);</td><td><svg style="fill: rgb(15 118 110);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-800</td><td>fill: rgb(17 94 89);</td><td><svg style="fill: rgb(17 94 89);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-teal-900</td><td>fill: rgb(19 78 74);</td><td><svg style="fill: rgb(19 78 74);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-50</td><td>fill: rgb(236 254 255);</td><td><svg style="fill: rgb(236 254 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-100</td><td>fill: rgb(207 250 254);</td><td><svg style="fill: rgb(207 250 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-200</td><td>fill: rgb(165 243 252);</td><td><svg style="fill: rgb(165 243 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-300</td><td>fill: rgb(103 232 249);</td><td><svg style="fill: rgb(103 232 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-400</td><td>fill: rgb(34 211 238);</td><td><svg style="fill: rgb(34 211 238);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-500</td><td>fill: rgb(6 182 212);</td><td><svg style="fill: rgb(6 182 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-600</td><td>fill: rgb(8 145 178);</td><td><svg style="fill: rgb(8 145 178);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-700</td><td>fill: rgb(14 116 144);</td><td><svg style="fill: rgb(14 116 144);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-800</td><td>fill: rgb(21 94 117);</td><td><svg style="fill: rgb(21 94 117);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-cyan-900</td><td>fill: rgb(22 78 99);</td><td><svg style="fill: rgb(22 78 99);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-50</td><td>fill: rgb(240 249 255);</td><td><svg style="fill: rgb(240 249 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-100</td><td>fill: rgb(224 242 254);</td><td><svg style="fill: rgb(224 242 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-200</td><td>fill: rgb(186 230 253);</td><td><svg style="fill: rgb(186 230 253);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-300</td><td>fill: rgb(125 211 252);</td><td><svg style="fill: rgb(125 211 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-400</td><td>fill: rgb(56 189 248);</td><td><svg style="fill: rgb(56 189 248);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-500</td><td>fill: rgb(14 165 233);</td><td><svg style="fill: rgb(14 165 233);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-600</td><td>fill: rgb(2 132 199);</td><td><svg style="fill: rgb(2 132 199);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-700</td><td>fill: rgb(3 105 161);</td><td><svg style="fill: rgb(3 105 161);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-800</td><td>fill: rgb(7 89 133);</td><td><svg style="fill: rgb(7 89 133);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-sky-900</td><td>fill: rgb(12 74 110);</td><td><svg style="fill: rgb(12 74 110);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-50</td><td>fill: rgb(239 246 255);</td><td><svg style="fill: rgb(239 246 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-100</td><td>fill: rgb(219 234 254);</td><td><svg style="fill: rgb(219 234 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-200</td><td>fill: rgb(191 219 254);</td><td><svg style="fill: rgb(191 219 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-300</td><td>fill: rgb(147 197 253);</td><td><svg style="fill: rgb(147 197 253);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-400</td><td>fill: rgb(96 165 250);</td><td><svg style="fill: rgb(96 165 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-500</td><td>fill: rgb(59 130 246);</td><td><svg style="fill: rgb(59 130 246);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-600</td><td>fill: rgb(37 99 235);</td><td><svg style="fill: rgb(37 99 235);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-700</td><td>fill: rgb(29 78 216);</td><td><svg style="fill: rgb(29 78 216);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-800</td><td>fill: rgb(30 64 175);</td><td><svg style="fill: rgb(30 64 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-blue-900</td><td>fill: rgb(30 58 138);</td><td><svg style="fill: rgb(30 58 138);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-50</td><td>fill: rgb(238 242 255);</td><td><svg style="fill: rgb(238 242 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-100</td><td>fill: rgb(224 231 255);</td><td><svg style="fill: rgb(224 231 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-200</td><td>fill: rgb(199 210 254);</td><td><svg style="fill: rgb(199 210 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-300</td><td>fill: rgb(165 180 252);</td><td><svg style="fill: rgb(165 180 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-400</td><td>fill: rgb(129 140 248);</td><td><svg style="fill: rgb(129 140 248);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-500</td><td>fill: rgb(99 102 241);</td><td><svg style="fill: rgb(99 102 241);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-600</td><td>fill: rgb(79 70 229);</td><td><svg style="fill: rgb(79 70 229);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-700</td><td>fill: rgb(67 56 202);</td><td><svg style="fill: rgb(67 56 202);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-800</td><td>fill: rgb(55 48 163);</td><td><svg style="fill: rgb(55 48 163);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-indigo-900</td><td>fill: rgb(49 46 129);</td><td><svg style="fill: rgb(49 46 129);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-50</td><td>fill: rgb(245 243 255);</td><td><svg style="fill: rgb(245 243 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-100</td><td>fill: rgb(237 233 254);</td><td><svg style="fill: rgb(237 233 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-200</td><td>fill: rgb(221 214 254);</td><td><svg style="fill: rgb(221 214 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-300</td><td>fill: rgb(196 181 253);</td><td><svg style="fill: rgb(196 181 253);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-400</td><td>fill: rgb(167 139 250);</td><td><svg style="fill: rgb(167 139 250);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-500</td><td>fill: rgb(139 92 246);</td><td><svg style="fill: rgb(139 92 246);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-600</td><td>fill: rgb(124 58 237);</td><td><svg style="fill: rgb(124 58 237);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-700</td><td>fill: rgb(109 40 217);</td><td><svg style="fill: rgb(109 40 217);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-800</td><td>fill: rgb(91 33 182);</td><td><svg style="fill: rgb(91 33 182);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-violet-900</td><td>fill: rgb(76 29 149);</td><td><svg style="fill: rgb(76 29 149);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-50</td><td>fill: rgb(250 245 255);</td><td><svg style="fill: rgb(250 245 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-100</td><td>fill: rgb(243 232 255);</td><td><svg style="fill: rgb(243 232 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-200</td><td>fill: rgb(233 213 255);</td><td><svg style="fill: rgb(233 213 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-300</td><td>fill: rgb(216 180 254);</td><td><svg style="fill: rgb(216 180 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-400</td><td>fill: rgb(192 132 252);</td><td><svg style="fill: rgb(192 132 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-500</td><td>fill: rgb(168 85 247);</td><td><svg style="fill: rgb(168 85 247);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-600</td><td>fill: rgb(147 51 234);</td><td><svg style="fill: rgb(147 51 234);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-700</td><td>fill: rgb(126 34 206);</td><td><svg style="fill: rgb(126 34 206);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-800</td><td>fill: rgb(107 33 168);</td><td><svg style="fill: rgb(107 33 168);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-purple-900</td><td>fill: rgb(88 28 135);</td><td><svg style="fill: rgb(88 28 135);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-50</td><td>fill: rgb(253 244 255);</td><td><svg style="fill: rgb(253 244 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-100</td><td>fill: rgb(250 232 255);</td><td><svg style="fill: rgb(250 232 255);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-200</td><td>fill: rgb(245 208 254);</td><td><svg style="fill: rgb(245 208 254);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-300</td><td>fill: rgb(240 171 252);</td><td><svg style="fill: rgb(240 171 252);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-400</td><td>fill: rgb(232 121 249);</td><td><svg style="fill: rgb(232 121 249);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-500</td><td>fill: rgb(217 70 239);</td><td><svg style="fill: rgb(217 70 239);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-600</td><td>fill: rgb(192 38 211);</td><td><svg style="fill: rgb(192 38 211);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-700</td><td>fill: rgb(162 28 175);</td><td><svg style="fill: rgb(162 28 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-800</td><td>fill: rgb(134 25 143);</td><td><svg style="fill: rgb(134 25 143);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-fuchsia-900</td><td>fill: rgb(112 26 117);</td><td><svg style="fill: rgb(112 26 117);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-50</td><td>fill: rgb(253 242 248);</td><td><svg style="fill: rgb(253 242 248);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-100</td><td>fill: rgb(252 231 243);</td><td><svg style="fill: rgb(252 231 243);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-200</td><td>fill: rgb(251 207 232);</td><td><svg style="fill: rgb(251 207 232);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-300</td><td>fill: rgb(249 168 212);</td><td><svg style="fill: rgb(249 168 212);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-400</td><td>fill: rgb(244 114 182);</td><td><svg style="fill: rgb(244 114 182);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-500</td><td>fill: rgb(236 72 153);</td><td><svg style="fill: rgb(236 72 153);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-600</td><td>fill: rgb(219 39 119);</td><td><svg style="fill: rgb(219 39 119);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-700</td><td>fill: rgb(190 24 93);</td><td><svg style="fill: rgb(190 24 93);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-800</td><td>fill: rgb(157 23 77);</td><td><svg style="fill: rgb(157 23 77);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-pink-900</td><td>fill: rgb(131 24 67);</td><td><svg style="fill: rgb(131 24 67);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-50</td><td>fill: rgb(255 241 242);</td><td><svg style="fill: rgb(255 241 242);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-100</td><td>fill: rgb(255 228 230);</td><td><svg style="fill: rgb(255 228 230);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-200</td><td>fill: rgb(254 205 211);</td><td><svg style="fill: rgb(254 205 211);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-300</td><td>fill: rgb(253 164 175);</td><td><svg style="fill: rgb(253 164 175);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-400</td><td>fill: rgb(251 113 133);</td><td><svg style="fill: rgb(251 113 133);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-500</td><td>fill: rgb(244 63 94);</td><td><svg style="fill: rgb(244 63 94);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-600</td><td>fill: rgb(225 29 72);</td><td><svg style="fill: rgb(225 29 72);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-700</td><td>fill: rgb(190 18 60);</td><td><svg style="fill: rgb(190 18 60);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-800</td><td>fill: rgb(159 18 57);</td><td><svg style="fill: rgb(159 18 57);"><use href="#svg-logo" /></svg></td></tr>
    <tr><td>fill-rose-900</td><td>fill: rgb(136 19 55);</td><td><svg style="fill: rgb(136 19 55);"><use href="#svg-logo" /></svg></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<color>`](crate::utils::value_matchers::is_matching_color) property is allowed as arbitrary value.
For example, `fill-[#ffab12]`.

[Tailwind reference](https://tailwindcss.com/docs/fill)
