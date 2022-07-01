Utilities for controlling the border color between elements.

<style>
#divide-color-table > tr td:nth-child(3) {
  position: relative;
}

#divide-color-table > tr td:nth-child(3) div {
  position: absolute;
  top: 50%;
  bottom: 0;
  left: 50%;
  right: 0;
  border-top-style: solid;
  border-top-width: 4px;
  width: 80%;
  height: 0px;
  transform: translate(-50%, -50%);
}
</style>

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center;">Color</th>
    </tr>
  </thead>
  <tbody id="divide-color-table">
    <tr><td>divide-inherit</td><td>border-color: inherit;</td><td><div style="border-color: inherit;"></div></td></tr>
    <tr><td>divide-current</td><td>border-color: currentColor;</td><td><div style="border-color: currentColor;"></div></td></tr>
    <tr><td>divide-transparent</td><td>border-color: transparent;</td><td><div style="border-color: transparent;"></div></td></tr>
    <tr><td>divide-black</td><td>border-color: rgb(0 0 0);</td><td><div style="border-color: rgb(0 0 0);"></div></td></tr>
    <tr><td>divide-white</td><td>border-color: rgb(255 255 255);</td><td><div style="border-color: rgb(255 255 255);"></div></td></tr>
    <tr><td>divide-slate-50</td><td>border-color: rgb(248 250 252);</td><td><div style="border-color: rgb(248 250 252);"></div></td></tr>
    <tr><td>divide-slate-100</td><td>border-color: rgb(241 245 249);</td><td><div style="border-color: rgb(241 245 249);"></div></td></tr>
    <tr><td>divide-slate-200</td><td>border-color: rgb(226 232 240);</td><td><div style="border-color: rgb(226 232 240);"></div></td></tr>
    <tr><td>divide-slate-300</td><td>border-color: rgb(203 213 225);</td><td><div style="border-color: rgb(203 213 225);"></div></td></tr>
    <tr><td>divide-slate-400</td><td>border-color: rgb(148 163 184);</td><td><div style="border-color: rgb(148 163 184);"></div></td></tr>
    <tr><td>divide-slate-500</td><td>border-color: rgb(100 116 139);</td><td><div style="border-color: rgb(100 116 139);"></div></td></tr>
    <tr><td>divide-slate-600</td><td>border-color: rgb(71 85 105);</td><td><div style="border-color: rgb(71 85 105);"></div></td></tr>
    <tr><td>divide-slate-700</td><td>border-color: rgb(51 65 85);</td><td><div style="border-color: rgb(51 65 85);"></div></td></tr>
    <tr><td>divide-slate-800</td><td>border-color: rgb(30 41 59);</td><td><div style="border-color: rgb(30 41 59);"></div></td></tr>
    <tr><td>divide-slate-900</td><td>border-color: rgb(15 23 42);</td><td><div style="border-color: rgb(15 23 42);"></div></td></tr>
    <tr><td>divide-gray-50</td><td>border-color: rgb(249 250 251);</td><td><div style="border-color: rgb(249 250 251);"></div></td></tr>
    <tr><td>divide-gray-100</td><td>border-color: rgb(243 244 246);</td><td><div style="border-color: rgb(243 244 246);"></div></td></tr>
    <tr><td>divide-gray-200</td><td>border-color: rgb(229 231 235);</td><td><div style="border-color: rgb(229 231 235);"></div></td></tr>
    <tr><td>divide-gray-300</td><td>border-color: rgb(209 213 219);</td><td><div style="border-color: rgb(209 213 219);"></div></td></tr>
    <tr><td>divide-gray-400</td><td>border-color: rgb(156 163 175);</td><td><div style="border-color: rgb(156 163 175);"></div></td></tr>
    <tr><td>divide-gray-500</td><td>border-color: rgb(107 114 128);</td><td><div style="border-color: rgb(107 114 128);"></div></td></tr>
    <tr><td>divide-gray-600</td><td>border-color: rgb(75 85 99);</td><td><div style="border-color: rgb(75 85 99);"></div></td></tr>
    <tr><td>divide-gray-700</td><td>border-color: rgb(55 65 81);</td><td><div style="border-color: rgb(55 65 81);"></div></td></tr>
    <tr><td>divide-gray-800</td><td>border-color: rgb(31 41 55);</td><td><div style="border-color: rgb(31 41 55);"></div></td></tr>
    <tr><td>divide-gray-900</td><td>border-color: rgb(17 24 39);</td><td><div style="border-color: rgb(17 24 39);"></div></td></tr>
    <tr><td>divide-zinc-50</td><td>border-color: rgb(250 250 250);</td><td><div style="border-color: rgb(250 250 250);"></div></td></tr>
    <tr><td>divide-zinc-100</td><td>border-color: rgb(244 244 245);</td><td><div style="border-color: rgb(244 244 245);"></div></td></tr>
    <tr><td>divide-zinc-200</td><td>border-color: rgb(228 228 231);</td><td><div style="border-color: rgb(228 228 231);"></div></td></tr>
    <tr><td>divide-zinc-300</td><td>border-color: rgb(212 212 216);</td><td><div style="border-color: rgb(212 212 216);"></div></td></tr>
    <tr><td>divide-zinc-400</td><td>border-color: rgb(161 161 170);</td><td><div style="border-color: rgb(161 161 170);"></div></td></tr>
    <tr><td>divide-zinc-500</td><td>border-color: rgb(113 113 122);</td><td><div style="border-color: rgb(113 113 122);"></div></td></tr>
    <tr><td>divide-zinc-600</td><td>border-color: rgb(82 82 91);</td><td><div style="border-color: rgb(82 82 91);"></div></td></tr>
    <tr><td>divide-zinc-700</td><td>border-color: rgb(63 63 70);</td><td><div style="border-color: rgb(63 63 70);"></div></td></tr>
    <tr><td>divide-zinc-800</td><td>border-color: rgb(39 39 42);</td><td><div style="border-color: rgb(39 39 42);"></div></td></tr>
    <tr><td>divide-zinc-900</td><td>border-color: rgb(24 24 27);</td><td><div style="border-color: rgb(24 24 27);"></div></td></tr>
    <tr><td>divide-neutral-50</td><td>border-color: rgb(250 250 250);</td><td><div style="border-color: rgb(250 250 250);"></div></td></tr>
    <tr><td>divide-neutral-100</td><td>border-color: rgb(245 245 245);</td><td><div style="border-color: rgb(245 245 245);"></div></td></tr>
    <tr><td>divide-neutral-200</td><td>border-color: rgb(229 229 229);</td><td><div style="border-color: rgb(229 229 229);"></div></td></tr>
    <tr><td>divide-neutral-300</td><td>border-color: rgb(212 212 212);</td><td><div style="border-color: rgb(212 212 212);"></div></td></tr>
    <tr><td>divide-neutral-400</td><td>border-color: rgb(163 163 163);</td><td><div style="border-color: rgb(163 163 163);"></div></td></tr>
    <tr><td>divide-neutral-500</td><td>border-color: rgb(115 115 115);</td><td><div style="border-color: rgb(115 115 115);"></div></td></tr>
    <tr><td>divide-neutral-600</td><td>border-color: rgb(82 82 82);</td><td><div style="border-color: rgb(82 82 82);"></div></td></tr>
    <tr><td>divide-neutral-700</td><td>border-color: rgb(64 64 64);</td><td><div style="border-color: rgb(64 64 64);"></div></td></tr>
    <tr><td>divide-neutral-800</td><td>border-color: rgb(38 38 38);</td><td><div style="border-color: rgb(38 38 38);"></div></td></tr>
    <tr><td>divide-neutral-900</td><td>border-color: rgb(23 23 23);</td><td><div style="border-color: rgb(23 23 23);"></div></td></tr>
    <tr><td>divide-stone-50</td><td>border-color: rgb(250 250 249);</td><td><div style="border-color: rgb(250 250 249);"></div></td></tr>
    <tr><td>divide-stone-100</td><td>border-color: rgb(245 245 244);</td><td><div style="border-color: rgb(245 245 244);"></div></td></tr>
    <tr><td>divide-stone-200</td><td>border-color: rgb(231 229 228);</td><td><div style="border-color: rgb(231 229 228);"></div></td></tr>
    <tr><td>divide-stone-300</td><td>border-color: rgb(214 211 209);</td><td><div style="border-color: rgb(214 211 209);"></div></td></tr>
    <tr><td>divide-stone-400</td><td>border-color: rgb(168 162 158);</td><td><div style="border-color: rgb(168 162 158);"></div></td></tr>
    <tr><td>divide-stone-500</td><td>border-color: rgb(120 113 108);</td><td><div style="border-color: rgb(120 113 108);"></div></td></tr>
    <tr><td>divide-stone-600</td><td>border-color: rgb(87 83 78);</td><td><div style="border-color: rgb(87 83 78);"></div></td></tr>
    <tr><td>divide-stone-700</td><td>border-color: rgb(68 64 60);</td><td><div style="border-color: rgb(68 64 60);"></div></td></tr>
    <tr><td>divide-stone-800</td><td>border-color: rgb(41 37 36);</td><td><div style="border-color: rgb(41 37 36);"></div></td></tr>
    <tr><td>divide-stone-900</td><td>border-color: rgb(28 25 23);</td><td><div style="border-color: rgb(28 25 23);"></div></td></tr>
    <tr><td>divide-red-50</td><td>border-color: rgb(254 242 242);</td><td><div style="border-color: rgb(254 242 242);"></div></td></tr>
    <tr><td>divide-red-100</td><td>border-color: rgb(254 226 226);</td><td><div style="border-color: rgb(254 226 226);"></div></td></tr>
    <tr><td>divide-red-200</td><td>border-color: rgb(254 202 202);</td><td><div style="border-color: rgb(254 202 202);"></div></td></tr>
    <tr><td>divide-red-300</td><td>border-color: rgb(252 165 165);</td><td><div style="border-color: rgb(252 165 165);"></div></td></tr>
    <tr><td>divide-red-400</td><td>border-color: rgb(248 113 113);</td><td><div style="border-color: rgb(248 113 113);"></div></td></tr>
    <tr><td>divide-red-500</td><td>border-color: rgb(239 68 68);</td><td><div style="border-color: rgb(239 68 68);"></div></td></tr>
    <tr><td>divide-red-600</td><td>border-color: rgb(220 38 38);</td><td><div style="border-color: rgb(220 38 38);"></div></td></tr>
    <tr><td>divide-red-700</td><td>border-color: rgb(185 28 28);</td><td><div style="border-color: rgb(185 28 28);"></div></td></tr>
    <tr><td>divide-red-800</td><td>border-color: rgb(153 27 27);</td><td><div style="border-color: rgb(153 27 27);"></div></td></tr>
    <tr><td>divide-red-900</td><td>border-color: rgb(127 29 29);</td><td><div style="border-color: rgb(127 29 29);"></div></td></tr>
    <tr><td>divide-orange-50</td><td>border-color: rgb(255 247 237);</td><td><div style="border-color: rgb(255 247 237);"></div></td></tr>
    <tr><td>divide-orange-100</td><td>border-color: rgb(255 237 213);</td><td><div style="border-color: rgb(255 237 213);"></div></td></tr>
    <tr><td>divide-orange-200</td><td>border-color: rgb(254 215 170);</td><td><div style="border-color: rgb(254 215 170);"></div></td></tr>
    <tr><td>divide-orange-300</td><td>border-color: rgb(253 186 116);</td><td><div style="border-color: rgb(253 186 116);"></div></td></tr>
    <tr><td>divide-orange-400</td><td>border-color: rgb(251 146 60);</td><td><div style="border-color: rgb(251 146 60);"></div></td></tr>
    <tr><td>divide-orange-500</td><td>border-color: rgb(249 115 22);</td><td><div style="border-color: rgb(249 115 22);"></div></td></tr>
    <tr><td>divide-orange-600</td><td>border-color: rgb(234 88 12);</td><td><div style="border-color: rgb(234 88 12);"></div></td></tr>
    <tr><td>divide-orange-700</td><td>border-color: rgb(194 65 12);</td><td><div style="border-color: rgb(194 65 12);"></div></td></tr>
    <tr><td>divide-orange-800</td><td>border-color: rgb(154 52 18);</td><td><div style="border-color: rgb(154 52 18);"></div></td></tr>
    <tr><td>divide-orange-900</td><td>border-color: rgb(124 45 18);</td><td><div style="border-color: rgb(124 45 18);"></div></td></tr>
    <tr><td>divide-amber-50</td><td>border-color: rgb(255 251 235);</td><td><div style="border-color: rgb(255 251 235);"></div></td></tr>
    <tr><td>divide-amber-100</td><td>border-color: rgb(254 243 199);</td><td><div style="border-color: rgb(254 243 199);"></div></td></tr>
    <tr><td>divide-amber-200</td><td>border-color: rgb(253 230 138);</td><td><div style="border-color: rgb(253 230 138);"></div></td></tr>
    <tr><td>divide-amber-300</td><td>border-color: rgb(252 211 77);</td><td><div style="border-color: rgb(252 211 77);"></div></td></tr>
    <tr><td>divide-amber-400</td><td>border-color: rgb(251 191 36);</td><td><div style="border-color: rgb(251 191 36);"></div></td></tr>
    <tr><td>divide-amber-500</td><td>border-color: rgb(245 158 11);</td><td><div style="border-color: rgb(245 158 11);"></div></td></tr>
    <tr><td>divide-amber-600</td><td>border-color: rgb(217 119 6);</td><td><div style="border-color: rgb(217 119 6);"></div></td></tr>
    <tr><td>divide-amber-700</td><td>border-color: rgb(180 83 9);</td><td><div style="border-color: rgb(180 83 9);"></div></td></tr>
    <tr><td>divide-amber-800</td><td>border-color: rgb(146 64 14);</td><td><div style="border-color: rgb(146 64 14);"></div></td></tr>
    <tr><td>divide-amber-900</td><td>border-color: rgb(120 53 15);</td><td><div style="border-color: rgb(120 53 15);"></div></td></tr>
    <tr><td>divide-yellow-50</td><td>border-color: rgb(254 252 232);</td><td><div style="border-color: rgb(254 252 232);"></div></td></tr>
    <tr><td>divide-yellow-100</td><td>border-color: rgb(254 249 195);</td><td><div style="border-color: rgb(254 249 195);"></div></td></tr>
    <tr><td>divide-yellow-200</td><td>border-color: rgb(254 240 138);</td><td><div style="border-color: rgb(254 240 138);"></div></td></tr>
    <tr><td>divide-yellow-300</td><td>border-color: rgb(253 224 71);</td><td><div style="border-color: rgb(253 224 71);"></div></td></tr>
    <tr><td>divide-yellow-400</td><td>border-color: rgb(250 204 21);</td><td><div style="border-color: rgb(250 204 21);"></div></td></tr>
    <tr><td>divide-yellow-500</td><td>border-color: rgb(234 179 8);</td><td><div style="border-color: rgb(234 179 8);"></div></td></tr>
    <tr><td>divide-yellow-600</td><td>border-color: rgb(202 138 4);</td><td><div style="border-color: rgb(202 138 4);"></div></td></tr>
    <tr><td>divide-yellow-700</td><td>border-color: rgb(161 98 7);</td><td><div style="border-color: rgb(161 98 7);"></div></td></tr>
    <tr><td>divide-yellow-800</td><td>border-color: rgb(133 77 14);</td><td><div style="border-color: rgb(133 77 14);"></div></td></tr>
    <tr><td>divide-yellow-900</td><td>border-color: rgb(113 63 18);</td><td><div style="border-color: rgb(113 63 18);"></div></td></tr>
    <tr><td>divide-lime-50</td><td>border-color: rgb(247 254 231);</td><td><div style="border-color: rgb(247 254 231);"></div></td></tr>
    <tr><td>divide-lime-100</td><td>border-color: rgb(236 252 203);</td><td><div style="border-color: rgb(236 252 203);"></div></td></tr>
    <tr><td>divide-lime-200</td><td>border-color: rgb(217 249 157);</td><td><div style="border-color: rgb(217 249 157);"></div></td></tr>
    <tr><td>divide-lime-300</td><td>border-color: rgb(190 242 100);</td><td><div style="border-color: rgb(190 242 100);"></div></td></tr>
    <tr><td>divide-lime-400</td><td>border-color: rgb(163 230 53);</td><td><div style="border-color: rgb(163 230 53);"></div></td></tr>
    <tr><td>divide-lime-500</td><td>border-color: rgb(132 204 22);</td><td><div style="border-color: rgb(132 204 22);"></div></td></tr>
    <tr><td>divide-lime-600</td><td>border-color: rgb(101 163 13);</td><td><div style="border-color: rgb(101 163 13);"></div></td></tr>
    <tr><td>divide-lime-700</td><td>border-color: rgb(77 124 15);</td><td><div style="border-color: rgb(77 124 15);"></div></td></tr>
    <tr><td>divide-lime-800</td><td>border-color: rgb(63 98 18);</td><td><div style="border-color: rgb(63 98 18);"></div></td></tr>
    <tr><td>divide-lime-900</td><td>border-color: rgb(54 83 20);</td><td><div style="border-color: rgb(54 83 20);"></div></td></tr>
    <tr><td>divide-green-50</td><td>border-color: rgb(240 253 244);</td><td><div style="border-color: rgb(240 253 244);"></div></td></tr>
    <tr><td>divide-green-100</td><td>border-color: rgb(220 252 231);</td><td><div style="border-color: rgb(220 252 231);"></div></td></tr>
    <tr><td>divide-green-200</td><td>border-color: rgb(187 247 208);</td><td><div style="border-color: rgb(187 247 208);"></div></td></tr>
    <tr><td>divide-green-300</td><td>border-color: rgb(134 239 172);</td><td><div style="border-color: rgb(134 239 172);"></div></td></tr>
    <tr><td>divide-green-400</td><td>border-color: rgb(74 222 128);</td><td><div style="border-color: rgb(74 222 128);"></div></td></tr>
    <tr><td>divide-green-500</td><td>border-color: rgb(34 197 94);</td><td><div style="border-color: rgb(34 197 94);"></div></td></tr>
    <tr><td>divide-green-600</td><td>border-color: rgb(22 163 74);</td><td><div style="border-color: rgb(22 163 74);"></div></td></tr>
    <tr><td>divide-green-700</td><td>border-color: rgb(21 128 61);</td><td><div style="border-color: rgb(21 128 61);"></div></td></tr>
    <tr><td>divide-green-800</td><td>border-color: rgb(22 101 52);</td><td><div style="border-color: rgb(22 101 52);"></div></td></tr>
    <tr><td>divide-green-900</td><td>border-color: rgb(20 83 45);</td><td><div style="border-color: rgb(20 83 45);"></div></td></tr>
    <tr><td>divide-emerald-50</td><td>border-color: rgb(236 253 245);</td><td><div style="border-color: rgb(236 253 245);"></div></td></tr>
    <tr><td>divide-emerald-100</td><td>border-color: rgb(209 250 229);</td><td><div style="border-color: rgb(209 250 229);"></div></td></tr>
    <tr><td>divide-emerald-200</td><td>border-color: rgb(167 243 208);</td><td><div style="border-color: rgb(167 243 208);"></div></td></tr>
    <tr><td>divide-emerald-300</td><td>border-color: rgb(110 231 183);</td><td><div style="border-color: rgb(110 231 183);"></div></td></tr>
    <tr><td>divide-emerald-400</td><td>border-color: rgb(52 211 153);</td><td><div style="border-color: rgb(52 211 153);"></div></td></tr>
    <tr><td>divide-emerald-500</td><td>border-color: rgb(16 185 129);</td><td><div style="border-color: rgb(16 185 129);"></div></td></tr>
    <tr><td>divide-emerald-600</td><td>border-color: rgb(5 150 105);</td><td><div style="border-color: rgb(5 150 105);"></div></td></tr>
    <tr><td>divide-emerald-700</td><td>border-color: rgb(4 120 87);</td><td><div style="border-color: rgb(4 120 87);"></div></td></tr>
    <tr><td>divide-emerald-800</td><td>border-color: rgb(6 95 70);</td><td><div style="border-color: rgb(6 95 70);"></div></td></tr>
    <tr><td>divide-emerald-900</td><td>border-color: rgb(6 78 59);</td><td><div style="border-color: rgb(6 78 59);"></div></td></tr>
    <tr><td>divide-teal-50</td><td>border-color: rgb(240 253 250);</td><td><div style="border-color: rgb(240 253 250);"></div></td></tr>
    <tr><td>divide-teal-100</td><td>border-color: rgb(204 251 241);</td><td><div style="border-color: rgb(204 251 241);"></div></td></tr>
    <tr><td>divide-teal-200</td><td>border-color: rgb(153 246 228);</td><td><div style="border-color: rgb(153 246 228);"></div></td></tr>
    <tr><td>divide-teal-300</td><td>border-color: rgb(94 234 212);</td><td><div style="border-color: rgb(94 234 212);"></div></td></tr>
    <tr><td>divide-teal-400</td><td>border-color: rgb(45 212 191);</td><td><div style="border-color: rgb(45 212 191);"></div></td></tr>
    <tr><td>divide-teal-500</td><td>border-color: rgb(20 184 166);</td><td><div style="border-color: rgb(20 184 166);"></div></td></tr>
    <tr><td>divide-teal-600</td><td>border-color: rgb(13 148 136);</td><td><div style="border-color: rgb(13 148 136);"></div></td></tr>
    <tr><td>divide-teal-700</td><td>border-color: rgb(15 118 110);</td><td><div style="border-color: rgb(15 118 110);"></div></td></tr>
    <tr><td>divide-teal-800</td><td>border-color: rgb(17 94 89);</td><td><div style="border-color: rgb(17 94 89);"></div></td></tr>
    <tr><td>divide-teal-900</td><td>border-color: rgb(19 78 74);</td><td><div style="border-color: rgb(19 78 74);"></div></td></tr>
    <tr><td>divide-cyan-50</td><td>border-color: rgb(236 254 255);</td><td><div style="border-color: rgb(236 254 255);"></div></td></tr>
    <tr><td>divide-cyan-100</td><td>border-color: rgb(207 250 254);</td><td><div style="border-color: rgb(207 250 254);"></div></td></tr>
    <tr><td>divide-cyan-200</td><td>border-color: rgb(165 243 252);</td><td><div style="border-color: rgb(165 243 252);"></div></td></tr>
    <tr><td>divide-cyan-300</td><td>border-color: rgb(103 232 249);</td><td><div style="border-color: rgb(103 232 249);"></div></td></tr>
    <tr><td>divide-cyan-400</td><td>border-color: rgb(34 211 238);</td><td><div style="border-color: rgb(34 211 238);"></div></td></tr>
    <tr><td>divide-cyan-500</td><td>border-color: rgb(6 182 212);</td><td><div style="border-color: rgb(6 182 212);"></div></td></tr>
    <tr><td>divide-cyan-600</td><td>border-color: rgb(8 145 178);</td><td><div style="border-color: rgb(8 145 178);"></div></td></tr>
    <tr><td>divide-cyan-700</td><td>border-color: rgb(14 116 144);</td><td><div style="border-color: rgb(14 116 144);"></div></td></tr>
    <tr><td>divide-cyan-800</td><td>border-color: rgb(21 94 117);</td><td><div style="border-color: rgb(21 94 117);"></div></td></tr>
    <tr><td>divide-cyan-900</td><td>border-color: rgb(22 78 99);</td><td><div style="border-color: rgb(22 78 99);"></div></td></tr>
    <tr><td>divide-sky-50</td><td>border-color: rgb(240 249 255);</td><td><div style="border-color: rgb(240 249 255);"></div></td></tr>
    <tr><td>divide-sky-100</td><td>border-color: rgb(224 242 254);</td><td><div style="border-color: rgb(224 242 254);"></div></td></tr>
    <tr><td>divide-sky-200</td><td>border-color: rgb(186 230 253);</td><td><div style="border-color: rgb(186 230 253);"></div></td></tr>
    <tr><td>divide-sky-300</td><td>border-color: rgb(125 211 252);</td><td><div style="border-color: rgb(125 211 252);"></div></td></tr>
    <tr><td>divide-sky-400</td><td>border-color: rgb(56 189 248);</td><td><div style="border-color: rgb(56 189 248);"></div></td></tr>
    <tr><td>divide-sky-500</td><td>border-color: rgb(14 165 233);</td><td><div style="border-color: rgb(14 165 233);"></div></td></tr>
    <tr><td>divide-sky-600</td><td>border-color: rgb(2 132 199);</td><td><div style="border-color: rgb(2 132 199);"></div></td></tr>
    <tr><td>divide-sky-700</td><td>border-color: rgb(3 105 161);</td><td><div style="border-color: rgb(3 105 161);"></div></td></tr>
    <tr><td>divide-sky-800</td><td>border-color: rgb(7 89 133);</td><td><div style="border-color: rgb(7 89 133);"></div></td></tr>
    <tr><td>divide-sky-900</td><td>border-color: rgb(12 74 110);</td><td><div style="border-color: rgb(12 74 110);"></div></td></tr>
    <tr><td>divide-blue-50</td><td>border-color: rgb(239 246 255);</td><td><div style="border-color: rgb(239 246 255);"></div></td></tr>
    <tr><td>divide-blue-100</td><td>border-color: rgb(219 234 254);</td><td><div style="border-color: rgb(219 234 254);"></div></td></tr>
    <tr><td>divide-blue-200</td><td>border-color: rgb(191 219 254);</td><td><div style="border-color: rgb(191 219 254);"></div></td></tr>
    <tr><td>divide-blue-300</td><td>border-color: rgb(147 197 253);</td><td><div style="border-color: rgb(147 197 253);"></div></td></tr>
    <tr><td>divide-blue-400</td><td>border-color: rgb(96 165 250);</td><td><div style="border-color: rgb(96 165 250);"></div></td></tr>
    <tr><td>divide-blue-500</td><td>border-color: rgb(59 130 246);</td><td><div style="border-color: rgb(59 130 246);"></div></td></tr>
    <tr><td>divide-blue-600</td><td>border-color: rgb(37 99 235);</td><td><div style="border-color: rgb(37 99 235);"></div></td></tr>
    <tr><td>divide-blue-700</td><td>border-color: rgb(29 78 216);</td><td><div style="border-color: rgb(29 78 216);"></div></td></tr>
    <tr><td>divide-blue-800</td><td>border-color: rgb(30 64 175);</td><td><div style="border-color: rgb(30 64 175);"></div></td></tr>
    <tr><td>divide-blue-900</td><td>border-color: rgb(30 58 138);</td><td><div style="border-color: rgb(30 58 138);"></div></td></tr>
    <tr><td>divide-indigo-50</td><td>border-color: rgb(238 242 255);</td><td><div style="border-color: rgb(238 242 255);"></div></td></tr>
    <tr><td>divide-indigo-100</td><td>border-color: rgb(224 231 255);</td><td><div style="border-color: rgb(224 231 255);"></div></td></tr>
    <tr><td>divide-indigo-200</td><td>border-color: rgb(199 210 254);</td><td><div style="border-color: rgb(199 210 254);"></div></td></tr>
    <tr><td>divide-indigo-300</td><td>border-color: rgb(165 180 252);</td><td><div style="border-color: rgb(165 180 252);"></div></td></tr>
    <tr><td>divide-indigo-400</td><td>border-color: rgb(129 140 248);</td><td><div style="border-color: rgb(129 140 248);"></div></td></tr>
    <tr><td>divide-indigo-500</td><td>border-color: rgb(99 102 241);</td><td><div style="border-color: rgb(99 102 241);"></div></td></tr>
    <tr><td>divide-indigo-600</td><td>border-color: rgb(79 70 229);</td><td><div style="border-color: rgb(79 70 229);"></div></td></tr>
    <tr><td>divide-indigo-700</td><td>border-color: rgb(67 56 202);</td><td><div style="border-color: rgb(67 56 202);"></div></td></tr>
    <tr><td>divide-indigo-800</td><td>border-color: rgb(55 48 163);</td><td><div style="border-color: rgb(55 48 163);"></div></td></tr>
    <tr><td>divide-indigo-900</td><td>border-color: rgb(49 46 129);</td><td><div style="border-color: rgb(49 46 129);"></div></td></tr>
    <tr><td>divide-violet-50</td><td>border-color: rgb(245 243 255);</td><td><div style="border-color: rgb(245 243 255);"></div></td></tr>
    <tr><td>divide-violet-100</td><td>border-color: rgb(237 233 254);</td><td><div style="border-color: rgb(237 233 254);"></div></td></tr>
    <tr><td>divide-violet-200</td><td>border-color: rgb(221 214 254);</td><td><div style="border-color: rgb(221 214 254);"></div></td></tr>
    <tr><td>divide-violet-300</td><td>border-color: rgb(196 181 253);</td><td><div style="border-color: rgb(196 181 253);"></div></td></tr>
    <tr><td>divide-violet-400</td><td>border-color: rgb(167 139 250);</td><td><div style="border-color: rgb(167 139 250);"></div></td></tr>
    <tr><td>divide-violet-500</td><td>border-color: rgb(139 92 246);</td><td><div style="border-color: rgb(139 92 246);"></div></td></tr>
    <tr><td>divide-violet-600</td><td>border-color: rgb(124 58 237);</td><td><div style="border-color: rgb(124 58 237);"></div></td></tr>
    <tr><td>divide-violet-700</td><td>border-color: rgb(109 40 217);</td><td><div style="border-color: rgb(109 40 217);"></div></td></tr>
    <tr><td>divide-violet-800</td><td>border-color: rgb(91 33 182);</td><td><div style="border-color: rgb(91 33 182);"></div></td></tr>
    <tr><td>divide-violet-900</td><td>border-color: rgb(76 29 149);</td><td><div style="border-color: rgb(76 29 149);"></div></td></tr>
    <tr><td>divide-purple-50</td><td>border-color: rgb(250 245 255);</td><td><div style="border-color: rgb(250 245 255);"></div></td></tr>
    <tr><td>divide-purple-100</td><td>border-color: rgb(243 232 255);</td><td><div style="border-color: rgb(243 232 255);"></div></td></tr>
    <tr><td>divide-purple-200</td><td>border-color: rgb(233 213 255);</td><td><div style="border-color: rgb(233 213 255);"></div></td></tr>
    <tr><td>divide-purple-300</td><td>border-color: rgb(216 180 254);</td><td><div style="border-color: rgb(216 180 254);"></div></td></tr>
    <tr><td>divide-purple-400</td><td>border-color: rgb(192 132 252);</td><td><div style="border-color: rgb(192 132 252);"></div></td></tr>
    <tr><td>divide-purple-500</td><td>border-color: rgb(168 85 247);</td><td><div style="border-color: rgb(168 85 247);"></div></td></tr>
    <tr><td>divide-purple-600</td><td>border-color: rgb(147 51 234);</td><td><div style="border-color: rgb(147 51 234);"></div></td></tr>
    <tr><td>divide-purple-700</td><td>border-color: rgb(126 34 206);</td><td><div style="border-color: rgb(126 34 206);"></div></td></tr>
    <tr><td>divide-purple-800</td><td>border-color: rgb(107 33 168);</td><td><div style="border-color: rgb(107 33 168);"></div></td></tr>
    <tr><td>divide-purple-900</td><td>border-color: rgb(88 28 135);</td><td><div style="border-color: rgb(88 28 135);"></div></td></tr>
    <tr><td>divide-fuchsia-50</td><td>border-color: rgb(253 244 255);</td><td><div style="border-color: rgb(253 244 255);"></div></td></tr>
    <tr><td>divide-fuchsia-100</td><td>border-color: rgb(250 232 255);</td><td><div style="border-color: rgb(250 232 255);"></div></td></tr>
    <tr><td>divide-fuchsia-200</td><td>border-color: rgb(245 208 254);</td><td><div style="border-color: rgb(245 208 254);"></div></td></tr>
    <tr><td>divide-fuchsia-300</td><td>border-color: rgb(240 171 252);</td><td><div style="border-color: rgb(240 171 252);"></div></td></tr>
    <tr><td>divide-fuchsia-400</td><td>border-color: rgb(232 121 249);</td><td><div style="border-color: rgb(232 121 249);"></div></td></tr>
    <tr><td>divide-fuchsia-500</td><td>border-color: rgb(217 70 239);</td><td><div style="border-color: rgb(217 70 239);"></div></td></tr>
    <tr><td>divide-fuchsia-600</td><td>border-color: rgb(192 38 211);</td><td><div style="border-color: rgb(192 38 211);"></div></td></tr>
    <tr><td>divide-fuchsia-700</td><td>border-color: rgb(162 28 175);</td><td><div style="border-color: rgb(162 28 175);"></div></td></tr>
    <tr><td>divide-fuchsia-800</td><td>border-color: rgb(134 25 143);</td><td><div style="border-color: rgb(134 25 143);"></div></td></tr>
    <tr><td>divide-fuchsia-900</td><td>border-color: rgb(112 26 117);</td><td><div style="border-color: rgb(112 26 117);"></div></td></tr>
    <tr><td>divide-pink-50</td><td>border-color: rgb(253 242 248);</td><td><div style="border-color: rgb(253 242 248);"></div></td></tr>
    <tr><td>divide-pink-100</td><td>border-color: rgb(252 231 243);</td><td><div style="border-color: rgb(252 231 243);"></div></td></tr>
    <tr><td>divide-pink-200</td><td>border-color: rgb(251 207 232);</td><td><div style="border-color: rgb(251 207 232);"></div></td></tr>
    <tr><td>divide-pink-300</td><td>border-color: rgb(249 168 212);</td><td><div style="border-color: rgb(249 168 212);"></div></td></tr>
    <tr><td>divide-pink-400</td><td>border-color: rgb(244 114 182);</td><td><div style="border-color: rgb(244 114 182);"></div></td></tr>
    <tr><td>divide-pink-500</td><td>border-color: rgb(236 72 153);</td><td><div style="border-color: rgb(236 72 153);"></div></td></tr>
    <tr><td>divide-pink-600</td><td>border-color: rgb(219 39 119);</td><td><div style="border-color: rgb(219 39 119);"></div></td></tr>
    <tr><td>divide-pink-700</td><td>border-color: rgb(190 24 93);</td><td><div style="border-color: rgb(190 24 93);"></div></td></tr>
    <tr><td>divide-pink-800</td><td>border-color: rgb(157 23 77);</td><td><div style="border-color: rgb(157 23 77);"></div></td></tr>
    <tr><td>divide-pink-900</td><td>border-color: rgb(131 24 67);</td><td><div style="border-color: rgb(131 24 67);"></div></td></tr>
    <tr><td>divide-rose-50</td><td>border-color: rgb(255 241 242);</td><td><div style="border-color: rgb(255 241 242);"></div></td></tr>
    <tr><td>divide-rose-100</td><td>border-color: rgb(255 228 230);</td><td><div style="border-color: rgb(255 228 230);"></div></td></tr>
    <tr><td>divide-rose-200</td><td>border-color: rgb(254 205 211);</td><td><div style="border-color: rgb(254 205 211);"></div></td></tr>
    <tr><td>divide-rose-300</td><td>border-color: rgb(253 164 175);</td><td><div style="border-color: rgb(253 164 175);"></div></td></tr>
    <tr><td>divide-rose-400</td><td>border-color: rgb(251 113 133);</td><td><div style="border-color: rgb(251 113 133);"></div></td></tr>
    <tr><td>divide-rose-500</td><td>border-color: rgb(244 63 94);</td><td><div style="border-color: rgb(244 63 94);"></div></td></tr>
    <tr><td>divide-rose-600</td><td>border-color: rgb(225 29 72);</td><td><div style="border-color: rgb(225 29 72);"></div></td></tr>
    <tr><td>divide-rose-700</td><td>border-color: rgb(190 18 60);</td><td><div style="border-color: rgb(190 18 60);"></div></td></tr>
    <tr><td>divide-rose-800</td><td>border-color: rgb(159 18 57);</td><td><div style="border-color: rgb(159 18 57);"></div></td></tr>
    <tr><td>divide-rose-900</td><td>border-color: rgb(136 19 55);</td><td><div style="border-color: rgb(136 19 55);"></div></td></tr>
  </tbody>
</table>

### Note about the opacity syntax

The color opacity can be changed using the new syntax: e.g. `divide-red-500/25` will generate
`border-color: rgb(239 68 68 / 0.25);`.

### Arbitrary values

Any [`<color>`](crate::utils::value_matchers::is_matching_color) property is allowed as arbitrary value.
For example, `divide-[#f0f]`.

[Tailwind reference](https://tailwindcss.com/docs/divide-color)
