Utilities for controlling the accented color of a form control.

<style>
#accent-color-table > tr td:nth-child(3) {
  position: relative;
}

#accent-color-table > tr td:nth-child(3) input {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 2rem;
  height: 2rem;
  cursor: pointer;
}
</style>

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center; width: 10rem;">Properties</th>
      <th style="text-align: center;">Color</th>
    </tr>
  </thead>
  <tbody id="accent-color-table">
    <tr><td>accent-inherit</td><td>accent-color: inherit;</td><td style="accent-color: inherit;"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-current</td><td>accent-color: currentColor;</td><td style="accent-color: currentColor;"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-transparent</td><td>accent-color: transparent;</td><td style="accent-color: transparent;"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-black</td><td>accent-color: rgb(0 0 0);</td><td style="accent-color: rgb(0 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-white</td><td>accent-color: rgb(255 255 255);</td><td style="accent-color: rgb(255 255 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-50</td><td>accent-color: rgb(248 250 252);</td><td style="accent-color: rgb(248 250 252);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-100</td><td>accent-color: rgb(241 245 249);</td><td style="accent-color: rgb(241 245 249);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-200</td><td>accent-color: rgb(226 232 240);</td><td style="accent-color: rgb(226 232 240);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-300</td><td>accent-color: rgb(203 213 225);</td><td style="accent-color: rgb(203 213 225);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-400</td><td>accent-color: rgb(148 163 184);</td><td style="accent-color: rgb(148 163 184);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-500</td><td>accent-color: rgb(100 116 139);</td><td style="accent-color: rgb(100 116 139);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-600</td><td>accent-color: rgb(71 85 105);</td><td style="accent-color: rgb(71 85 105);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-700</td><td>accent-color: rgb(51 65 85);</td><td style="accent-color: rgb(51 65 85);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-800</td><td>accent-color: rgb(30 41 59);</td><td style="accent-color: rgb(30 41 59);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-900</td><td>accent-color: rgb(15 23 42);</td><td style="accent-color: rgb(15 23 42);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-50</td><td>accent-color: rgb(249 250 251);</td><td style="accent-color: rgb(249 250 251);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-100</td><td>accent-color: rgb(243 244 246);</td><td style="accent-color: rgb(243 244 246);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-200</td><td>accent-color: rgb(229 231 235);</td><td style="accent-color: rgb(229 231 235);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-300</td><td>accent-color: rgb(209 213 219);</td><td style="accent-color: rgb(209 213 219);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-400</td><td>accent-color: rgb(156 163 175);</td><td style="accent-color: rgb(156 163 175);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-500</td><td>accent-color: rgb(107 114 128);</td><td style="accent-color: rgb(107 114 128);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-600</td><td>accent-color: rgb(75 85 99);</td><td style="accent-color: rgb(75 85 99);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-700</td><td>accent-color: rgb(55 65 81);</td><td style="accent-color: rgb(55 65 81);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-800</td><td>accent-color: rgb(31 41 55);</td><td style="accent-color: rgb(31 41 55);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-900</td><td>accent-color: rgb(17 24 39);</td><td style="accent-color: rgb(17 24 39);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-50</td><td>accent-color: rgb(250 250 250);</td><td style="accent-color: rgb(250 250 250);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-100</td><td>accent-color: rgb(244 244 245);</td><td style="accent-color: rgb(244 244 245);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-200</td><td>accent-color: rgb(228 228 231);</td><td style="accent-color: rgb(228 228 231);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-300</td><td>accent-color: rgb(212 212 216);</td><td style="accent-color: rgb(212 212 216);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-400</td><td>accent-color: rgb(161 161 170);</td><td style="accent-color: rgb(161 161 170);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-500</td><td>accent-color: rgb(113 113 122);</td><td style="accent-color: rgb(113 113 122);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-600</td><td>accent-color: rgb(82 82 91);</td><td style="accent-color: rgb(82 82 91);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-700</td><td>accent-color: rgb(63 63 70);</td><td style="accent-color: rgb(63 63 70);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-800</td><td>accent-color: rgb(39 39 42);</td><td style="accent-color: rgb(39 39 42);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-900</td><td>accent-color: rgb(24 24 27);</td><td style="accent-color: rgb(24 24 27);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-50</td><td>accent-color: rgb(250 250 250);</td><td style="accent-color: rgb(250 250 250);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-100</td><td>accent-color: rgb(245 245 245);</td><td style="accent-color: rgb(245 245 245);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-200</td><td>accent-color: rgb(229 229 229);</td><td style="accent-color: rgb(229 229 229);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-300</td><td>accent-color: rgb(212 212 212);</td><td style="accent-color: rgb(212 212 212);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-400</td><td>accent-color: rgb(163 163 163);</td><td style="accent-color: rgb(163 163 163);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-500</td><td>accent-color: rgb(115 115 115);</td><td style="accent-color: rgb(115 115 115);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-600</td><td>accent-color: rgb(82 82 82);</td><td style="accent-color: rgb(82 82 82);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-700</td><td>accent-color: rgb(64 64 64);</td><td style="accent-color: rgb(64 64 64);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-800</td><td>accent-color: rgb(38 38 38);</td><td style="accent-color: rgb(38 38 38);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-900</td><td>accent-color: rgb(23 23 23);</td><td style="accent-color: rgb(23 23 23);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-50</td><td>accent-color: rgb(250 250 249);</td><td style="accent-color: rgb(250 250 249);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-100</td><td>accent-color: rgb(245 245 244);</td><td style="accent-color: rgb(245 245 244);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-200</td><td>accent-color: rgb(231 229 228);</td><td style="accent-color: rgb(231 229 228);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-300</td><td>accent-color: rgb(214 211 209);</td><td style="accent-color: rgb(214 211 209);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-400</td><td>accent-color: rgb(168 162 158);</td><td style="accent-color: rgb(168 162 158);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-500</td><td>accent-color: rgb(120 113 108);</td><td style="accent-color: rgb(120 113 108);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-600</td><td>accent-color: rgb(87 83 78);</td><td style="accent-color: rgb(87 83 78);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-700</td><td>accent-color: rgb(68 64 60);</td><td style="accent-color: rgb(68 64 60);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-800</td><td>accent-color: rgb(41 37 36);</td><td style="accent-color: rgb(41 37 36);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-900</td><td>accent-color: rgb(28 25 23);</td><td style="accent-color: rgb(28 25 23);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-50</td><td>accent-color: rgb(254 242 242);</td><td style="accent-color: rgb(254 242 242);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-100</td><td>accent-color: rgb(254 226 226);</td><td style="accent-color: rgb(254 226 226);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-200</td><td>accent-color: rgb(254 202 202);</td><td style="accent-color: rgb(254 202 202);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-300</td><td>accent-color: rgb(252 165 165);</td><td style="accent-color: rgb(252 165 165);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-400</td><td>accent-color: rgb(248 113 113);</td><td style="accent-color: rgb(248 113 113);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-500</td><td>accent-color: rgb(239 68 68);</td><td style="accent-color: rgb(239 68 68);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-600</td><td>accent-color: rgb(220 38 38);</td><td style="accent-color: rgb(220 38 38);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-700</td><td>accent-color: rgb(185 28 28);</td><td style="accent-color: rgb(185 28 28);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-800</td><td>accent-color: rgb(153 27 27);</td><td style="accent-color: rgb(153 27 27);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-900</td><td>accent-color: rgb(127 29 29);</td><td style="accent-color: rgb(127 29 29);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-50</td><td>accent-color: rgb(255 247 237);</td><td style="accent-color: rgb(255 247 237);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-100</td><td>accent-color: rgb(255 237 213);</td><td style="accent-color: rgb(255 237 213);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-200</td><td>accent-color: rgb(254 215 170);</td><td style="accent-color: rgb(254 215 170);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-300</td><td>accent-color: rgb(253 186 116);</td><td style="accent-color: rgb(253 186 116);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-400</td><td>accent-color: rgb(251 146 60);</td><td style="accent-color: rgb(251 146 60);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-500</td><td>accent-color: rgb(249 115 22);</td><td style="accent-color: rgb(249 115 22);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-600</td><td>accent-color: rgb(234 88 12);</td><td style="accent-color: rgb(234 88 12);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-700</td><td>accent-color: rgb(194 65 12);</td><td style="accent-color: rgb(194 65 12);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-800</td><td>accent-color: rgb(154 52 18);</td><td style="accent-color: rgb(154 52 18);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-900</td><td>accent-color: rgb(124 45 18);</td><td style="accent-color: rgb(124 45 18);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-50</td><td>accent-color: rgb(255 251 235);</td><td style="accent-color: rgb(255 251 235);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-100</td><td>accent-color: rgb(254 243 199);</td><td style="accent-color: rgb(254 243 199);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-200</td><td>accent-color: rgb(253 230 138);</td><td style="accent-color: rgb(253 230 138);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-300</td><td>accent-color: rgb(252 211 77);</td><td style="accent-color: rgb(252 211 77);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-400</td><td>accent-color: rgb(251 191 36);</td><td style="accent-color: rgb(251 191 36);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-500</td><td>accent-color: rgb(245 158 11);</td><td style="accent-color: rgb(245 158 11);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-600</td><td>accent-color: rgb(217 119 6);</td><td style="accent-color: rgb(217 119 6);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-700</td><td>accent-color: rgb(180 83 9);</td><td style="accent-color: rgb(180 83 9);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-800</td><td>accent-color: rgb(146 64 14);</td><td style="accent-color: rgb(146 64 14);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-900</td><td>accent-color: rgb(120 53 15);</td><td style="accent-color: rgb(120 53 15);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-50</td><td>accent-color: rgb(254 252 232);</td><td style="accent-color: rgb(254 252 232);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-100</td><td>accent-color: rgb(254 249 195);</td><td style="accent-color: rgb(254 249 195);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-200</td><td>accent-color: rgb(254 240 138);</td><td style="accent-color: rgb(254 240 138);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-300</td><td>accent-color: rgb(253 224 71);</td><td style="accent-color: rgb(253 224 71);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-400</td><td>accent-color: rgb(250 204 21);</td><td style="accent-color: rgb(250 204 21);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-500</td><td>accent-color: rgb(234 179 8);</td><td style="accent-color: rgb(234 179 8);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-600</td><td>accent-color: rgb(202 138 4);</td><td style="accent-color: rgb(202 138 4);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-700</td><td>accent-color: rgb(161 98 7);</td><td style="accent-color: rgb(161 98 7);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-800</td><td>accent-color: rgb(133 77 14);</td><td style="accent-color: rgb(133 77 14);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-900</td><td>accent-color: rgb(113 63 18);</td><td style="accent-color: rgb(113 63 18);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-50</td><td>accent-color: rgb(247 254 231);</td><td style="accent-color: rgb(247 254 231);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-100</td><td>accent-color: rgb(236 252 203);</td><td style="accent-color: rgb(236 252 203);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-200</td><td>accent-color: rgb(217 249 157);</td><td style="accent-color: rgb(217 249 157);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-300</td><td>accent-color: rgb(190 242 100);</td><td style="accent-color: rgb(190 242 100);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-400</td><td>accent-color: rgb(163 230 53);</td><td style="accent-color: rgb(163 230 53);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-500</td><td>accent-color: rgb(132 204 22);</td><td style="accent-color: rgb(132 204 22);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-600</td><td>accent-color: rgb(101 163 13);</td><td style="accent-color: rgb(101 163 13);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-700</td><td>accent-color: rgb(77 124 15);</td><td style="accent-color: rgb(77 124 15);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-800</td><td>accent-color: rgb(63 98 18);</td><td style="accent-color: rgb(63 98 18);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-900</td><td>accent-color: rgb(54 83 20);</td><td style="accent-color: rgb(54 83 20);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-50</td><td>accent-color: rgb(240 253 244);</td><td style="accent-color: rgb(240 253 244);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-100</td><td>accent-color: rgb(220 252 231);</td><td style="accent-color: rgb(220 252 231);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-200</td><td>accent-color: rgb(187 247 208);</td><td style="accent-color: rgb(187 247 208);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-300</td><td>accent-color: rgb(134 239 172);</td><td style="accent-color: rgb(134 239 172);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-400</td><td>accent-color: rgb(74 222 128);</td><td style="accent-color: rgb(74 222 128);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-500</td><td>accent-color: rgb(34 197 94);</td><td style="accent-color: rgb(34 197 94);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-600</td><td>accent-color: rgb(22 163 74);</td><td style="accent-color: rgb(22 163 74);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-700</td><td>accent-color: rgb(21 128 61);</td><td style="accent-color: rgb(21 128 61);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-800</td><td>accent-color: rgb(22 101 52);</td><td style="accent-color: rgb(22 101 52);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-900</td><td>accent-color: rgb(20 83 45);</td><td style="accent-color: rgb(20 83 45);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-50</td><td>accent-color: rgb(236 253 245);</td><td style="accent-color: rgb(236 253 245);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-100</td><td>accent-color: rgb(209 250 229);</td><td style="accent-color: rgb(209 250 229);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-200</td><td>accent-color: rgb(167 243 208);</td><td style="accent-color: rgb(167 243 208);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-300</td><td>accent-color: rgb(110 231 183);</td><td style="accent-color: rgb(110 231 183);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-400</td><td>accent-color: rgb(52 211 153);</td><td style="accent-color: rgb(52 211 153);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-500</td><td>accent-color: rgb(16 185 129);</td><td style="accent-color: rgb(16 185 129);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-600</td><td>accent-color: rgb(5 150 105);</td><td style="accent-color: rgb(5 150 105);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-700</td><td>accent-color: rgb(4 120 87);</td><td style="accent-color: rgb(4 120 87);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-800</td><td>accent-color: rgb(6 95 70);</td><td style="accent-color: rgb(6 95 70);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-900</td><td>accent-color: rgb(6 78 59);</td><td style="accent-color: rgb(6 78 59);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-50</td><td>accent-color: rgb(240 253 250);</td><td style="accent-color: rgb(240 253 250);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-100</td><td>accent-color: rgb(204 251 241);</td><td style="accent-color: rgb(204 251 241);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-200</td><td>accent-color: rgb(153 246 228);</td><td style="accent-color: rgb(153 246 228);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-300</td><td>accent-color: rgb(94 234 212);</td><td style="accent-color: rgb(94 234 212);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-400</td><td>accent-color: rgb(45 212 191);</td><td style="accent-color: rgb(45 212 191);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-500</td><td>accent-color: rgb(20 184 166);</td><td style="accent-color: rgb(20 184 166);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-600</td><td>accent-color: rgb(13 148 136);</td><td style="accent-color: rgb(13 148 136);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-700</td><td>accent-color: rgb(15 118 110);</td><td style="accent-color: rgb(15 118 110);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-800</td><td>accent-color: rgb(17 94 89);</td><td style="accent-color: rgb(17 94 89);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-900</td><td>accent-color: rgb(19 78 74);</td><td style="accent-color: rgb(19 78 74);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-50</td><td>accent-color: rgb(236 254 255);</td><td style="accent-color: rgb(236 254 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-100</td><td>accent-color: rgb(207 250 254);</td><td style="accent-color: rgb(207 250 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-200</td><td>accent-color: rgb(165 243 252);</td><td style="accent-color: rgb(165 243 252);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-300</td><td>accent-color: rgb(103 232 249);</td><td style="accent-color: rgb(103 232 249);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-400</td><td>accent-color: rgb(34 211 238);</td><td style="accent-color: rgb(34 211 238);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-500</td><td>accent-color: rgb(6 182 212);</td><td style="accent-color: rgb(6 182 212);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-600</td><td>accent-color: rgb(8 145 178);</td><td style="accent-color: rgb(8 145 178);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-700</td><td>accent-color: rgb(14 116 144);</td><td style="accent-color: rgb(14 116 144);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-800</td><td>accent-color: rgb(21 94 117);</td><td style="accent-color: rgb(21 94 117);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-900</td><td>accent-color: rgb(22 78 99);</td><td style="accent-color: rgb(22 78 99);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-50</td><td>accent-color: rgb(240 249 255);</td><td style="accent-color: rgb(240 249 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-100</td><td>accent-color: rgb(224 242 254);</td><td style="accent-color: rgb(224 242 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-200</td><td>accent-color: rgb(186 230 253);</td><td style="accent-color: rgb(186 230 253);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-300</td><td>accent-color: rgb(125 211 252);</td><td style="accent-color: rgb(125 211 252);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-400</td><td>accent-color: rgb(56 189 248);</td><td style="accent-color: rgb(56 189 248);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-500</td><td>accent-color: rgb(14 165 233);</td><td style="accent-color: rgb(14 165 233);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-600</td><td>accent-color: rgb(2 132 199);</td><td style="accent-color: rgb(2 132 199);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-700</td><td>accent-color: rgb(3 105 161);</td><td style="accent-color: rgb(3 105 161);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-800</td><td>accent-color: rgb(7 89 133);</td><td style="accent-color: rgb(7 89 133);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-900</td><td>accent-color: rgb(12 74 110);</td><td style="accent-color: rgb(12 74 110);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-50</td><td>accent-color: rgb(239 246 255);</td><td style="accent-color: rgb(239 246 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-100</td><td>accent-color: rgb(219 234 254);</td><td style="accent-color: rgb(219 234 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-200</td><td>accent-color: rgb(191 219 254);</td><td style="accent-color: rgb(191 219 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-300</td><td>accent-color: rgb(147 197 253);</td><td style="accent-color: rgb(147 197 253);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-400</td><td>accent-color: rgb(96 165 250);</td><td style="accent-color: rgb(96 165 250);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-500</td><td>accent-color: rgb(59 130 246);</td><td style="accent-color: rgb(59 130 246);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-600</td><td>accent-color: rgb(37 99 235);</td><td style="accent-color: rgb(37 99 235);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-700</td><td>accent-color: rgb(29 78 216);</td><td style="accent-color: rgb(29 78 216);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-800</td><td>accent-color: rgb(30 64 175);</td><td style="accent-color: rgb(30 64 175);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-900</td><td>accent-color: rgb(30 58 138);</td><td style="accent-color: rgb(30 58 138);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-50</td><td>accent-color: rgb(238 242 255);</td><td style="accent-color: rgb(238 242 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-100</td><td>accent-color: rgb(224 231 255);</td><td style="accent-color: rgb(224 231 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-200</td><td>accent-color: rgb(199 210 254);</td><td style="accent-color: rgb(199 210 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-300</td><td>accent-color: rgb(165 180 252);</td><td style="accent-color: rgb(165 180 252);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-400</td><td>accent-color: rgb(129 140 248);</td><td style="accent-color: rgb(129 140 248);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-500</td><td>accent-color: rgb(99 102 241);</td><td style="accent-color: rgb(99 102 241);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-600</td><td>accent-color: rgb(79 70 229);</td><td style="accent-color: rgb(79 70 229);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-700</td><td>accent-color: rgb(67 56 202);</td><td style="accent-color: rgb(67 56 202);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-800</td><td>accent-color: rgb(55 48 163);</td><td style="accent-color: rgb(55 48 163);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-900</td><td>accent-color: rgb(49 46 129);</td><td style="accent-color: rgb(49 46 129);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-50</td><td>accent-color: rgb(245 243 255);</td><td style="accent-color: rgb(245 243 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-100</td><td>accent-color: rgb(237 233 254);</td><td style="accent-color: rgb(237 233 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-200</td><td>accent-color: rgb(221 214 254);</td><td style="accent-color: rgb(221 214 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-300</td><td>accent-color: rgb(196 181 253);</td><td style="accent-color: rgb(196 181 253);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-400</td><td>accent-color: rgb(167 139 250);</td><td style="accent-color: rgb(167 139 250);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-500</td><td>accent-color: rgb(139 92 246);</td><td style="accent-color: rgb(139 92 246);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-600</td><td>accent-color: rgb(124 58 237);</td><td style="accent-color: rgb(124 58 237);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-700</td><td>accent-color: rgb(109 40 217);</td><td style="accent-color: rgb(109 40 217);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-800</td><td>accent-color: rgb(91 33 182);</td><td style="accent-color: rgb(91 33 182);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-900</td><td>accent-color: rgb(76 29 149);</td><td style="accent-color: rgb(76 29 149);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-50</td><td>accent-color: rgb(250 245 255);</td><td style="accent-color: rgb(250 245 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-100</td><td>accent-color: rgb(243 232 255);</td><td style="accent-color: rgb(243 232 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-200</td><td>accent-color: rgb(233 213 255);</td><td style="accent-color: rgb(233 213 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-300</td><td>accent-color: rgb(216 180 254);</td><td style="accent-color: rgb(216 180 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-400</td><td>accent-color: rgb(192 132 252);</td><td style="accent-color: rgb(192 132 252);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-500</td><td>accent-color: rgb(168 85 247);</td><td style="accent-color: rgb(168 85 247);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-600</td><td>accent-color: rgb(147 51 234);</td><td style="accent-color: rgb(147 51 234);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-700</td><td>accent-color: rgb(126 34 206);</td><td style="accent-color: rgb(126 34 206);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-800</td><td>accent-color: rgb(107 33 168);</td><td style="accent-color: rgb(107 33 168);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-900</td><td>accent-color: rgb(88 28 135);</td><td style="accent-color: rgb(88 28 135);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-50</td><td>accent-color: rgb(253 244 255);</td><td style="accent-color: rgb(253 244 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-100</td><td>accent-color: rgb(250 232 255);</td><td style="accent-color: rgb(250 232 255);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-200</td><td>accent-color: rgb(245 208 254);</td><td style="accent-color: rgb(245 208 254);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-300</td><td>accent-color: rgb(240 171 252);</td><td style="accent-color: rgb(240 171 252);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-400</td><td>accent-color: rgb(232 121 249);</td><td style="accent-color: rgb(232 121 249);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-500</td><td>accent-color: rgb(217 70 239);</td><td style="accent-color: rgb(217 70 239);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-600</td><td>accent-color: rgb(192 38 211);</td><td style="accent-color: rgb(192 38 211);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-700</td><td>accent-color: rgb(162 28 175);</td><td style="accent-color: rgb(162 28 175);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-800</td><td>accent-color: rgb(134 25 143);</td><td style="accent-color: rgb(134 25 143);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-900</td><td>accent-color: rgb(112 26 117);</td><td style="accent-color: rgb(112 26 117);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-50</td><td>accent-color: rgb(253 242 248);</td><td style="accent-color: rgb(253 242 248);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-100</td><td>accent-color: rgb(252 231 243);</td><td style="accent-color: rgb(252 231 243);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-200</td><td>accent-color: rgb(251 207 232);</td><td style="accent-color: rgb(251 207 232);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-300</td><td>accent-color: rgb(249 168 212);</td><td style="accent-color: rgb(249 168 212);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-400</td><td>accent-color: rgb(244 114 182);</td><td style="accent-color: rgb(244 114 182);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-500</td><td>accent-color: rgb(236 72 153);</td><td style="accent-color: rgb(236 72 153);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-600</td><td>accent-color: rgb(219 39 119);</td><td style="accent-color: rgb(219 39 119);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-700</td><td>accent-color: rgb(190 24 93);</td><td style="accent-color: rgb(190 24 93);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-800</td><td>accent-color: rgb(157 23 77);</td><td style="accent-color: rgb(157 23 77);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-900</td><td>accent-color: rgb(131 24 67);</td><td style="accent-color: rgb(131 24 67);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-50</td><td>accent-color: rgb(255 241 242);</td><td style="accent-color: rgb(255 241 242);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-100</td><td>accent-color: rgb(255 228 230);</td><td style="accent-color: rgb(255 228 230);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-200</td><td>accent-color: rgb(254 205 211);</td><td style="accent-color: rgb(254 205 211);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-300</td><td>accent-color: rgb(253 164 175);</td><td style="accent-color: rgb(253 164 175);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-400</td><td>accent-color: rgb(251 113 133);</td><td style="accent-color: rgb(251 113 133);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-500</td><td>accent-color: rgb(244 63 94);</td><td style="accent-color: rgb(244 63 94);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-600</td><td>accent-color: rgb(225 29 72);</td><td style="accent-color: rgb(225 29 72);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-700</td><td>accent-color: rgb(190 18 60);</td><td style="accent-color: rgb(190 18 60);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-800</td><td>accent-color: rgb(159 18 57);</td><td style="accent-color: rgb(159 18 57);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-900</td><td>accent-color: rgb(136 19 55);</td><td style="accent-color: rgb(136 19 55);"><input type="checkbox" checked /></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<color>`](crate::utils::value_matchers::is_matching_color) property is allowed as arbitrary value.
For example, `accent-[hsl(135,100%,50%)]`.

[Tailwind reference](https://tailwindcss.com/docs/accent-color)
