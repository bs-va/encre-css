Utilities for controlling the color of an element's borders.

<style>
#border-color-table > tr td:nth-child(3) {
  position: relative;
}

#border-color-table > tr {
  height: 4rem;
}

#border-color-table tr td:nth-child(3) div {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  border-style: solid;
  border-width: 4px;
  margin: 1rem;
  border-color: transparent;
}
</style>

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center; width: 10rem;">Color</th>
    </tr>
  </thead>
  <tbody id="border-color-table">
    <tr><td>border-inherit</td><td>border-color: inherit;</td><td><div style="border-color: inherit;"></div></td></tr>
    <tr><td>border-current</td><td>border-color: currentColor;</td><td><div style="border-color: currentColor;"></div></td></tr>
    <tr><td>border-transparent</td><td>border-color: transparent;</td><td><div style="border-color: transparent;"></div></td></tr>
    <tr><td>border-red-50</td><td>border-color: oklch(97.1% .013 17.38);</td><td><div style="border-color: oklch(97.1% .013 17.38);"></div></td></tr>
    <tr><td>border-red-100</td><td>border-color: oklch(93.6% .032 17.717);</td><td><div style="border-color: oklch(93.6% .032 17.717);"></div></td></tr>
    <tr><td>border-red-200</td><td>border-color: oklch(88.5% .062 18.334);</td><td><div style="border-color: oklch(88.5% .062 18.334);"></div></td></tr>
    <tr><td>border-red-300</td><td>border-color: oklch(80.8% .114 19.571);</td><td><div style="border-color: oklch(80.8% .114 19.571);"></div></td></tr>
    <tr><td>border-red-400</td><td>border-color: oklch(70.4% .191 22.216);</td><td><div style="border-color: oklch(70.4% .191 22.216);"></div></td></tr>
    <tr><td>border-red-500</td><td>border-color: oklch(63.7% .237 25.331);</td><td><div style="border-color: oklch(63.7% .237 25.331);"></div></td></tr>
    <tr><td>border-red-600</td><td>border-color: oklch(57.7% .245 27.325);</td><td><div style="border-color: oklch(57.7% .245 27.325);"></div></td></tr>
    <tr><td>border-red-700</td><td>border-color: oklch(50.5% .213 27.518);</td><td><div style="border-color: oklch(50.5% .213 27.518);"></div></td></tr>
    <tr><td>border-red-800</td><td>border-color: oklch(44.4% .177 26.899);</td><td><div style="border-color: oklch(44.4% .177 26.899);"></div></td></tr>
    <tr><td>border-red-900</td><td>border-color: oklch(39.6% .141 25.723);</td><td><div style="border-color: oklch(39.6% .141 25.723);"></div></td></tr>
    <tr><td>border-red-950</td><td>border-color: oklch(25.8% .092 26.042);</td><td><div style="border-color: oklch(25.8% .092 26.042);"></div></td></tr>
    <tr><td>border-orange-50</td><td>border-color: oklch(98% .016 73.684);</td><td><div style="border-color: oklch(98% .016 73.684);"></div></td></tr>
    <tr><td>border-orange-100</td><td>border-color: oklch(95.4% .038 75.164);</td><td><div style="border-color: oklch(95.4% .038 75.164);"></div></td></tr>
    <tr><td>border-orange-200</td><td>border-color: oklch(90.1% .076 70.697);</td><td><div style="border-color: oklch(90.1% .076 70.697);"></div></td></tr>
    <tr><td>border-orange-300</td><td>border-color: oklch(83.7% .128 66.29);</td><td><div style="border-color: oklch(83.7% .128 66.29);"></div></td></tr>
    <tr><td>border-orange-400</td><td>border-color: oklch(75% .183 55.934);</td><td><div style="border-color: oklch(75% .183 55.934);"></div></td></tr>
    <tr><td>border-orange-500</td><td>border-color: oklch(70.5% .213 47.604);</td><td><div style="border-color: oklch(70.5% .213 47.604);"></div></td></tr>
    <tr><td>border-orange-600</td><td>border-color: oklch(64.6% .222 41.116);</td><td><div style="border-color: oklch(64.6% .222 41.116);"></div></td></tr>
    <tr><td>border-orange-700</td><td>border-color: oklch(55.3% .195 38.402);</td><td><div style="border-color: oklch(55.3% .195 38.402);"></div></td></tr>
    <tr><td>border-orange-800</td><td>border-color: oklch(47% .157 37.304);</td><td><div style="border-color: oklch(47% .157 37.304);"></div></td></tr>
    <tr><td>border-orange-900</td><td>border-color: oklch(40.8% .123 38.172);</td><td><div style="border-color: oklch(40.8% .123 38.172);"></div></td></tr>
    <tr><td>border-orange-950</td><td>border-color: oklch(26.6% .079 36.259);</td><td><div style="border-color: oklch(26.6% .079 36.259);"></div></td></tr>
    <tr><td>border-amber-50</td><td>border-color: oklch(98.7% .022 95.277);</td><td><div style="border-color: oklch(98.7% .022 95.277);"></div></td></tr>
    <tr><td>border-amber-100</td><td>border-color: oklch(96.2% .059 95.617);</td><td><div style="border-color: oklch(96.2% .059 95.617);"></div></td></tr>
    <tr><td>border-amber-200</td><td>border-color: oklch(92.4% .12 95.746);</td><td><div style="border-color: oklch(92.4% .12 95.746);"></div></td></tr>
    <tr><td>border-amber-300</td><td>border-color: oklch(87.9% .169 91.605);</td><td><div style="border-color: oklch(87.9% .169 91.605);"></div></td></tr>
    <tr><td>border-amber-400</td><td>border-color: oklch(82.8% .189 84.429);</td><td><div style="border-color: oklch(82.8% .189 84.429);"></div></td></tr>
    <tr><td>border-amber-500</td><td>border-color: oklch(76.9% .188 70.08);</td><td><div style="border-color: oklch(76.9% .188 70.08);"></div></td></tr>
    <tr><td>border-amber-600</td><td>border-color: oklch(66.6% .179 58.318);</td><td><div style="border-color: oklch(66.6% .179 58.318);"></div></td></tr>
    <tr><td>border-amber-700</td><td>border-color: oklch(55.5% .163 48.998);</td><td><div style="border-color: oklch(55.5% .163 48.998);"></div></td></tr>
    <tr><td>border-amber-800</td><td>border-color: oklch(47.3% .137 46.201);</td><td><div style="border-color: oklch(47.3% .137 46.201);"></div></td></tr>
    <tr><td>border-amber-900</td><td>border-color: oklch(41.4% .112 45.904);</td><td><div style="border-color: oklch(41.4% .112 45.904);"></div></td></tr>
    <tr><td>border-amber-950</td><td>border-color: oklch(27.9% .077 45.635);</td><td><div style="border-color: oklch(27.9% .077 45.635);"></div></td></tr>
    <tr><td>border-yellow-50</td><td>border-color: oklch(98.7% .026 102.212);</td><td><div style="border-color: oklch(98.7% .026 102.212);"></div></td></tr>
    <tr><td>border-yellow-100</td><td>border-color: oklch(97.3% .071 103.193);</td><td><div style="border-color: oklch(97.3% .071 103.193);"></div></td></tr>
    <tr><td>border-yellow-200</td><td>border-color: oklch(94.5% .129 101.54);</td><td><div style="border-color: oklch(94.5% .129 101.54);"></div></td></tr>
    <tr><td>border-yellow-300</td><td>border-color: oklch(90.5% .182 98.111);</td><td><div style="border-color: oklch(90.5% .182 98.111);"></div></td></tr>
    <tr><td>border-yellow-400</td><td>border-color: oklch(85.2% .199 91.936);</td><td><div style="border-color: oklch(85.2% .199 91.936);"></div></td></tr>
    <tr><td>border-yellow-500</td><td>border-color: oklch(79.5% .184 86.047);</td><td><div style="border-color: oklch(79.5% .184 86.047);"></div></td></tr>
    <tr><td>border-yellow-600</td><td>border-color: oklch(68.1% .162 75.834);</td><td><div style="border-color: oklch(68.1% .162 75.834);"></div></td></tr>
    <tr><td>border-yellow-700</td><td>border-color: oklch(55.4% .135 66.442);</td><td><div style="border-color: oklch(55.4% .135 66.442);"></div></td></tr>
    <tr><td>border-yellow-800</td><td>border-color: oklch(47.6% .114 61.907);</td><td><div style="border-color: oklch(47.6% .114 61.907);"></div></td></tr>
    <tr><td>border-yellow-900</td><td>border-color: oklch(42.1% .095 57.708);</td><td><div style="border-color: oklch(42.1% .095 57.708);"></div></td></tr>
    <tr><td>border-yellow-950</td><td>border-color: oklch(28.6% .066 53.813);</td><td><div style="border-color: oklch(28.6% .066 53.813);"></div></td></tr>
    <tr><td>border-lime-50</td><td>border-color: oklch(98.6% .031 120.757);</td><td><div style="border-color: oklch(98.6% .031 120.757);"></div></td></tr>
    <tr><td>border-lime-100</td><td>border-color: oklch(96.7% .067 122.328);</td><td><div style="border-color: oklch(96.7% .067 122.328);"></div></td></tr>
    <tr><td>border-lime-200</td><td>border-color: oklch(93.8% .127 124.321);</td><td><div style="border-color: oklch(93.8% .127 124.321);"></div></td></tr>
    <tr><td>border-lime-300</td><td>border-color: oklch(89.7% .196 126.665);</td><td><div style="border-color: oklch(89.7% .196 126.665);"></div></td></tr>
    <tr><td>border-lime-400</td><td>border-color: oklch(84.1% .238 128.85);</td><td><div style="border-color: oklch(84.1% .238 128.85);"></div></td></tr>
    <tr><td>border-lime-500</td><td>border-color: oklch(76.8% .233 130.85);</td><td><div style="border-color: oklch(76.8% .233 130.85);"></div></td></tr>
    <tr><td>border-lime-600</td><td>border-color: oklch(64.8% .2 131.684);</td><td><div style="border-color: oklch(64.8% .2 131.684);"></div></td></tr>
    <tr><td>border-lime-700</td><td>border-color: oklch(53.2% .157 131.589);</td><td><div style="border-color: oklch(53.2% .157 131.589);"></div></td></tr>
    <tr><td>border-lime-800</td><td>border-color: oklch(45.3% .124 130.933);</td><td><div style="border-color: oklch(45.3% .124 130.933);"></div></td></tr>
    <tr><td>border-lime-900</td><td>border-color: oklch(40.5% .101 131.063);</td><td><div style="border-color: oklch(40.5% .101 131.063);"></div></td></tr>
    <tr><td>border-lime-950</td><td>border-color: oklch(27.4% .072 132.109);</td><td><div style="border-color: oklch(27.4% .072 132.109);"></div></td></tr>
    <tr><td>border-green-50</td><td>border-color: oklch(98.2% .018 155.826);</td><td><div style="border-color: oklch(98.2% .018 155.826);"></div></td></tr>
    <tr><td>border-green-100</td><td>border-color: oklch(96.2% .044 156.743);</td><td><div style="border-color: oklch(96.2% .044 156.743);"></div></td></tr>
    <tr><td>border-green-200</td><td>border-color: oklch(92.5% .084 155.995);</td><td><div style="border-color: oklch(92.5% .084 155.995);"></div></td></tr>
    <tr><td>border-green-300</td><td>border-color: oklch(87.1% .15 154.449);</td><td><div style="border-color: oklch(87.1% .15 154.449);"></div></td></tr>
    <tr><td>border-green-400</td><td>border-color: oklch(79.2% .209 151.711);</td><td><div style="border-color: oklch(79.2% .209 151.711);"></div></td></tr>
    <tr><td>border-green-500</td><td>border-color: oklch(72.3% .219 149.579);</td><td><div style="border-color: oklch(72.3% .219 149.579);"></div></td></tr>
    <tr><td>border-green-600</td><td>border-color: oklch(62.7% .194 149.214);</td><td><div style="border-color: oklch(62.7% .194 149.214);"></div></td></tr>
    <tr><td>border-green-700</td><td>border-color: oklch(52.7% .154 150.069);</td><td><div style="border-color: oklch(52.7% .154 150.069);"></div></td></tr>
    <tr><td>border-green-800</td><td>border-color: oklch(44.8% .119 151.328);</td><td><div style="border-color: oklch(44.8% .119 151.328);"></div></td></tr>
    <tr><td>border-green-900</td><td>border-color: oklch(39.3% .095 152.535);</td><td><div style="border-color: oklch(39.3% .095 152.535);"></div></td></tr>
    <tr><td>border-green-950</td><td>border-color: oklch(26.6% .065 152.934);</td><td><div style="border-color: oklch(26.6% .065 152.934);"></div></td></tr>
    <tr><td>border-emerald-50</td><td>border-color: oklch(97.9% .021 166.113);</td><td><div style="border-color: oklch(97.9% .021 166.113);"></div></td></tr>
    <tr><td>border-emerald-100</td><td>border-color: oklch(95% .052 163.051);</td><td><div style="border-color: oklch(95% .052 163.051);"></div></td></tr>
    <tr><td>border-emerald-200</td><td>border-color: oklch(90.5% .093 164.15);</td><td><div style="border-color: oklch(90.5% .093 164.15);"></div></td></tr>
    <tr><td>border-emerald-300</td><td>border-color: oklch(84.5% .143 164.978);</td><td><div style="border-color: oklch(84.5% .143 164.978);"></div></td></tr>
    <tr><td>border-emerald-400</td><td>border-color: oklch(76.5% .177 163.223);</td><td><div style="border-color: oklch(76.5% .177 163.223);"></div></td></tr>
    <tr><td>border-emerald-500</td><td>border-color: oklch(69.6% .17 162.48);</td><td><div style="border-color: oklch(69.6% .17 162.48);"></div></td></tr>
    <tr><td>border-emerald-600</td><td>border-color: oklch(59.6% .145 163.225);</td><td><div style="border-color: oklch(59.6% .145 163.225);"></div></td></tr>
    <tr><td>border-emerald-700</td><td>border-color: oklch(50.8% .118 165.612);</td><td><div style="border-color: oklch(50.8% .118 165.612);"></div></td></tr>
    <tr><td>border-emerald-800</td><td>border-color: oklch(43.2% .095 166.913);</td><td><div style="border-color: oklch(43.2% .095 166.913);"></div></td></tr>
    <tr><td>border-emerald-900</td><td>border-color: oklch(37.8% .077 168.94);</td><td><div style="border-color: oklch(37.8% .077 168.94);"></div></td></tr>
    <tr><td>border-emerald-950</td><td>border-color: oklch(26.2% .051 172.552);</td><td><div style="border-color: oklch(26.2% .051 172.552);"></div></td></tr>
    <tr><td>border-teal-50</td><td>border-color: oklch(98.4% .014 180.72);</td><td><div style="border-color: oklch(98.4% .014 180.72);"></div></td></tr>
    <tr><td>border-teal-100</td><td>border-color: oklch(95.3% .051 180.801);</td><td><div style="border-color: oklch(95.3% .051 180.801);"></div></td></tr>
    <tr><td>border-teal-200</td><td>border-color: oklch(91% .096 180.426);</td><td><div style="border-color: oklch(91% .096 180.426);"></div></td></tr>
    <tr><td>border-teal-300</td><td>border-color: oklch(85.5% .138 181.071);</td><td><div style="border-color: oklch(85.5% .138 181.071);"></div></td></tr>
    <tr><td>border-teal-400</td><td>border-color: oklch(77.7% .152 181.912);</td><td><div style="border-color: oklch(77.7% .152 181.912);"></div></td></tr>
    <tr><td>border-teal-500</td><td>border-color: oklch(70.4% .14 182.503);</td><td><div style="border-color: oklch(70.4% .14 182.503);"></div></td></tr>
    <tr><td>border-teal-600</td><td>border-color: oklch(60% .118 184.704);</td><td><div style="border-color: oklch(60% .118 184.704);"></div></td></tr>
    <tr><td>border-teal-700</td><td>border-color: oklch(51.1% .096 186.391);</td><td><div style="border-color: oklch(51.1% .096 186.391);"></div></td></tr>
    <tr><td>border-teal-800</td><td>border-color: oklch(43.7% .078 188.216);</td><td><div style="border-color: oklch(43.7% .078 188.216);"></div></td></tr>
    <tr><td>border-teal-900</td><td>border-color: oklch(38.6% .063 188.416);</td><td><div style="border-color: oklch(38.6% .063 188.416);"></div></td></tr>
    <tr><td>border-teal-950</td><td>border-color: oklch(27.7% .046 192.524);</td><td><div style="border-color: oklch(27.7% .046 192.524);"></div></td></tr>
    <tr><td>border-cyan-50</td><td>border-color: oklch(98.4% .019 200.873);</td><td><div style="border-color: oklch(98.4% .019 200.873);"></div></td></tr>
    <tr><td>border-cyan-100</td><td>border-color: oklch(95.6% .045 203.388);</td><td><div style="border-color: oklch(95.6% .045 203.388);"></div></td></tr>
    <tr><td>border-cyan-200</td><td>border-color: oklch(91.7% .08 205.041);</td><td><div style="border-color: oklch(91.7% .08 205.041);"></div></td></tr>
    <tr><td>border-cyan-300</td><td>border-color: oklch(86.5% .127 207.078);</td><td><div style="border-color: oklch(86.5% .127 207.078);"></div></td></tr>
    <tr><td>border-cyan-400</td><td>border-color: oklch(78.9% .154 211.53);</td><td><div style="border-color: oklch(78.9% .154 211.53);"></div></td></tr>
    <tr><td>border-cyan-500</td><td>border-color: oklch(71.5% .143 215.221);</td><td><div style="border-color: oklch(71.5% .143 215.221);"></div></td></tr>
    <tr><td>border-cyan-600</td><td>border-color: oklch(60.9% .126 221.723);</td><td><div style="border-color: oklch(60.9% .126 221.723);"></div></td></tr>
    <tr><td>border-cyan-700</td><td>border-color: oklch(52% .105 223.128);</td><td><div style="border-color: oklch(52% .105 223.128);"></div></td></tr>
    <tr><td>border-cyan-800</td><td>border-color: oklch(45% .085 224.283);</td><td><div style="border-color: oklch(45% .085 224.283);"></div></td></tr>
    <tr><td>border-cyan-900</td><td>border-color: oklch(39.8% .07 227.392);</td><td><div style="border-color: oklch(39.8% .07 227.392);"></div></td></tr>
    <tr><td>border-cyan-950</td><td>border-color: oklch(30.2% .056 229.695);</td><td><div style="border-color: oklch(30.2% .056 229.695);"></div></td></tr>
    <tr><td>border-sky-50</td><td>border-color: oklch(97.7% .013 236.62);</td><td><div style="border-color: oklch(97.7% .013 236.62);"></div></td></tr>
    <tr><td>border-sky-100</td><td>border-color: oklch(95.1% .026 236.824);</td><td><div style="border-color: oklch(95.1% .026 236.824);"></div></td></tr>
    <tr><td>border-sky-200</td><td>border-color: oklch(90.1% .058 230.902);</td><td><div style="border-color: oklch(90.1% .058 230.902);"></div></td></tr>
    <tr><td>border-sky-300</td><td>border-color: oklch(82.8% .111 230.318);</td><td><div style="border-color: oklch(82.8% .111 230.318);"></div></td></tr>
    <tr><td>border-sky-400</td><td>border-color: oklch(74.6% .16 232.661);</td><td><div style="border-color: oklch(74.6% .16 232.661);"></div></td></tr>
    <tr><td>border-sky-500</td><td>border-color: oklch(68.5% .169 237.323);</td><td><div style="border-color: oklch(68.5% .169 237.323);"></div></td></tr>
    <tr><td>border-sky-600</td><td>border-color: oklch(58.8% .158 241.966);</td><td><div style="border-color: oklch(58.8% .158 241.966);"></div></td></tr>
    <tr><td>border-sky-700</td><td>border-color: oklch(50% .134 242.749);</td><td><div style="border-color: oklch(50% .134 242.749);"></div></td></tr>
    <tr><td>border-sky-800</td><td>border-color: oklch(44.3% .11 240.79);</td><td><div style="border-color: oklch(44.3% .11 240.79);"></div></td></tr>
    <tr><td>border-sky-900</td><td>border-color: oklch(39.1% .09 240.876);</td><td><div style="border-color: oklch(39.1% .09 240.876);"></div></td></tr>
    <tr><td>border-sky-950</td><td>border-color: oklch(29.3% .066 243.157);</td><td><div style="border-color: oklch(29.3% .066 243.157);"></div></td></tr>
    <tr><td>border-blue-50</td><td>border-color: oklch(97% .014 254.604);</td><td><div style="border-color: oklch(97% .014 254.604);"></div></td></tr>
    <tr><td>border-blue-100</td><td>border-color: oklch(93.2% .032 255.585);</td><td><div style="border-color: oklch(93.2% .032 255.585);"></div></td></tr>
    <tr><td>border-blue-200</td><td>border-color: oklch(88.2% .059 254.128);</td><td><div style="border-color: oklch(88.2% .059 254.128);"></div></td></tr>
    <tr><td>border-blue-300</td><td>border-color: oklch(80.9% .105 251.813);</td><td><div style="border-color: oklch(80.9% .105 251.813);"></div></td></tr>
    <tr><td>border-blue-400</td><td>border-color: oklch(70.7% .165 254.624);</td><td><div style="border-color: oklch(70.7% .165 254.624);"></div></td></tr>
    <tr><td>border-blue-500</td><td>border-color: oklch(62.3% .214 259.815);</td><td><div style="border-color: oklch(62.3% .214 259.815);"></div></td></tr>
    <tr><td>border-blue-600</td><td>border-color: oklch(54.6% .245 262.881);</td><td><div style="border-color: oklch(54.6% .245 262.881);"></div></td></tr>
    <tr><td>border-blue-700</td><td>border-color: oklch(48.8% .243 264.376);</td><td><div style="border-color: oklch(48.8% .243 264.376);"></div></td></tr>
    <tr><td>border-blue-800</td><td>border-color: oklch(42.4% .199 265.638);</td><td><div style="border-color: oklch(42.4% .199 265.638);"></div></td></tr>
    <tr><td>border-blue-900</td><td>border-color: oklch(37.9% .146 265.522);</td><td><div style="border-color: oklch(37.9% .146 265.522);"></div></td></tr>
    <tr><td>border-blue-950</td><td>border-color: oklch(28.2% .091 267.935);</td><td><div style="border-color: oklch(28.2% .091 267.935);"></div></td></tr>
    <tr><td>border-indigo-50</td><td>border-color: oklch(96.2% .018 272.314);</td><td><div style="border-color: oklch(96.2% .018 272.314);"></div></td></tr>
    <tr><td>border-indigo-100</td><td>border-color: oklch(93% .034 272.788);</td><td><div style="border-color: oklch(93% .034 272.788);"></div></td></tr>
    <tr><td>border-indigo-200</td><td>border-color: oklch(87% .065 274.039);</td><td><div style="border-color: oklch(87% .065 274.039);"></div></td></tr>
    <tr><td>border-indigo-300</td><td>border-color: oklch(78.5% .115 274.713);</td><td><div style="border-color: oklch(78.5% .115 274.713);"></div></td></tr>
    <tr><td>border-indigo-400</td><td>border-color: oklch(67.3% .182 276.935);</td><td><div style="border-color: oklch(67.3% .182 276.935);"></div></td></tr>
    <tr><td>border-indigo-500</td><td>border-color: oklch(58.5% .233 277.117);</td><td><div style="border-color: oklch(58.5% .233 277.117);"></div></td></tr>
    <tr><td>border-indigo-600</td><td>border-color: oklch(51.1% .262 276.966);</td><td><div style="border-color: oklch(51.1% .262 276.966);"></div></td></tr>
    <tr><td>border-indigo-700</td><td>border-color: oklch(45.7% .24 277.023);</td><td><div style="border-color: oklch(45.7% .24 277.023);"></div></td></tr>
    <tr><td>border-indigo-800</td><td>border-color: oklch(39.8% .195 277.366);</td><td><div style="border-color: oklch(39.8% .195 277.366);"></div></td></tr>
    <tr><td>border-indigo-900</td><td>border-color: oklch(35.9% .144 278.697);</td><td><div style="border-color: oklch(35.9% .144 278.697);"></div></td></tr>
    <tr><td>border-indigo-950</td><td>border-color: oklch(25.7% .09 281.288);</td><td><div style="border-color: oklch(25.7% .09 281.288);"></div></td></tr>
    <tr><td>border-violet-50</td><td>border-color: oklch(96.9% .016 293.756);</td><td><div style="border-color: oklch(96.9% .016 293.756);"></div></td></tr>
    <tr><td>border-violet-100</td><td>border-color: oklch(94.3% .029 294.588);</td><td><div style="border-color: oklch(94.3% .029 294.588);"></div></td></tr>
    <tr><td>border-violet-200</td><td>border-color: oklch(89.4% .057 293.283);</td><td><div style="border-color: oklch(89.4% .057 293.283);"></div></td></tr>
    <tr><td>border-violet-300</td><td>border-color: oklch(81.1% .111 293.571);</td><td><div style="border-color: oklch(81.1% .111 293.571);"></div></td></tr>
    <tr><td>border-violet-400</td><td>border-color: oklch(70.2% .183 293.541);</td><td><div style="border-color: oklch(70.2% .183 293.541);"></div></td></tr>
    <tr><td>border-violet-500</td><td>border-color: oklch(60.6% .25 292.717);</td><td><div style="border-color: oklch(60.6% .25 292.717);"></div></td></tr>
    <tr><td>border-violet-600</td><td>border-color: oklch(54.1% .281 293.009);</td><td><div style="border-color: oklch(54.1% .281 293.009);"></div></td></tr>
    <tr><td>border-violet-700</td><td>border-color: oklch(49.1% .27 292.581);</td><td><div style="border-color: oklch(49.1% .27 292.581);"></div></td></tr>
    <tr><td>border-violet-800</td><td>border-color: oklch(43.2% .232 292.759);</td><td><div style="border-color: oklch(43.2% .232 292.759);"></div></td></tr>
    <tr><td>border-violet-900</td><td>border-color: oklch(38% .189 293.745);</td><td><div style="border-color: oklch(38% .189 293.745);"></div></td></tr>
    <tr><td>border-violet-950</td><td>border-color: oklch(28.3% .141 291.089);</td><td><div style="border-color: oklch(28.3% .141 291.089);"></div></td></tr>
    <tr><td>border-purple-50</td><td>border-color: oklch(97.7% .014 308.299);</td><td><div style="border-color: oklch(97.7% .014 308.299);"></div></td></tr>
    <tr><td>border-purple-100</td><td>border-color: oklch(94.6% .033 307.174);</td><td><div style="border-color: oklch(94.6% .033 307.174);"></div></td></tr>
    <tr><td>border-purple-200</td><td>border-color: oklch(90.2% .063 306.703);</td><td><div style="border-color: oklch(90.2% .063 306.703);"></div></td></tr>
    <tr><td>border-purple-300</td><td>border-color: oklch(82.7% .119 306.383);</td><td><div style="border-color: oklch(82.7% .119 306.383);"></div></td></tr>
    <tr><td>border-purple-400</td><td>border-color: oklch(71.4% .203 305.504);</td><td><div style="border-color: oklch(71.4% .203 305.504);"></div></td></tr>
    <tr><td>border-purple-500</td><td>border-color: oklch(62.7% .265 303.9);</td><td><div style="border-color: oklch(62.7% .265 303.9);"></div></td></tr>
    <tr><td>border-purple-600</td><td>border-color: oklch(55.8% .288 302.321);</td><td><div style="border-color: oklch(55.8% .288 302.321);"></div></td></tr>
    <tr><td>border-purple-700</td><td>border-color: oklch(49.6% .265 301.924);</td><td><div style="border-color: oklch(49.6% .265 301.924);"></div></td></tr>
    <tr><td>border-purple-800</td><td>border-color: oklch(43.8% .218 303.724);</td><td><div style="border-color: oklch(43.8% .218 303.724);"></div></td></tr>
    <tr><td>border-purple-900</td><td>border-color: oklch(38.1% .176 304.987);</td><td><div style="border-color: oklch(38.1% .176 304.987);"></div></td></tr>
    <tr><td>border-purple-950</td><td>border-color: oklch(29.1% .149 302.717);</td><td><div style="border-color: oklch(29.1% .149 302.717);"></div></td></tr>
    <tr><td>border-fuchsia-50</td><td>border-color: oklch(97.7% .017 320.058);</td><td><div style="border-color: oklch(97.7% .017 320.058);"></div></td></tr>
    <tr><td>border-fuchsia-100</td><td>border-color: oklch(95.2% .037 318.852);</td><td><div style="border-color: oklch(95.2% .037 318.852);"></div></td></tr>
    <tr><td>border-fuchsia-200</td><td>border-color: oklch(90.3% .076 319.62);</td><td><div style="border-color: oklch(90.3% .076 319.62);"></div></td></tr>
    <tr><td>border-fuchsia-300</td><td>border-color: oklch(83.3% .145 321.434);</td><td><div style="border-color: oklch(83.3% .145 321.434);"></div></td></tr>
    <tr><td>border-fuchsia-400</td><td>border-color: oklch(74% .238 322.16);</td><td><div style="border-color: oklch(74% .238 322.16);"></div></td></tr>
    <tr><td>border-fuchsia-500</td><td>border-color: oklch(66.7% .295 322.15);</td><td><div style="border-color: oklch(66.7% .295 322.15);"></div></td></tr>
    <tr><td>border-fuchsia-600</td><td>border-color: oklch(59.1% .293 322.896);</td><td><div style="border-color: oklch(59.1% .293 322.896);"></div></td></tr>
    <tr><td>border-fuchsia-700</td><td>border-color: oklch(51.8% .253 323.949);</td><td><div style="border-color: oklch(51.8% .253 323.949);"></div></td></tr>
    <tr><td>border-fuchsia-800</td><td>border-color: oklch(45.2% .211 324.591);</td><td><div style="border-color: oklch(45.2% .211 324.591);"></div></td></tr>
    <tr><td>border-fuchsia-900</td><td>border-color: oklch(40.1% .17 325.612);</td><td><div style="border-color: oklch(40.1% .17 325.612);"></div></td></tr>
    <tr><td>border-fuchsia-950</td><td>border-color: oklch(29.3% .136 325.661);</td><td><div style="border-color: oklch(29.3% .136 325.661);"></div></td></tr>
    <tr><td>border-pink-50</td><td>border-color: oklch(97.1% .014 343.198);</td><td><div style="border-color: oklch(97.1% .014 343.198);"></div></td></tr>
    <tr><td>border-pink-100</td><td>border-color: oklch(94.8% .028 342.258);</td><td><div style="border-color: oklch(94.8% .028 342.258);"></div></td></tr>
    <tr><td>border-pink-200</td><td>border-color: oklch(89.9% .061 343.231);</td><td><div style="border-color: oklch(89.9% .061 343.231);"></div></td></tr>
    <tr><td>border-pink-300</td><td>border-color: oklch(82.3% .12 346.018);</td><td><div style="border-color: oklch(82.3% .12 346.018);"></div></td></tr>
    <tr><td>border-pink-400</td><td>border-color: oklch(71.8% .202 349.761);</td><td><div style="border-color: oklch(71.8% .202 349.761);"></div></td></tr>
    <tr><td>border-pink-500</td><td>border-color: oklch(65.6% .241 354.308);</td><td><div style="border-color: oklch(65.6% .241 354.308);"></div></td></tr>
    <tr><td>border-pink-600</td><td>border-color: oklch(59.2% .249 .584);</td><td><div style="border-color: oklch(59.2% .249 .584);"></div></td></tr>
    <tr><td>border-pink-700</td><td>border-color: oklch(52.5% .223 3.958);</td><td><div style="border-color: oklch(52.5% .223 3.958);"></div></td></tr>
    <tr><td>border-pink-800</td><td>border-color: oklch(45.9% .187 3.815);</td><td><div style="border-color: oklch(45.9% .187 3.815);"></div></td></tr>
    <tr><td>border-pink-900</td><td>border-color: oklch(40.8% .153 2.432);</td><td><div style="border-color: oklch(40.8% .153 2.432);"></div></td></tr>
    <tr><td>border-pink-950</td><td>border-color: oklch(28.4% .109 3.907);</td><td><div style="border-color: oklch(28.4% .109 3.907);"></div></td></tr>
    <tr><td>border-rose-50</td><td>border-color: oklch(96.9% .015 12.422);</td><td><div style="border-color: oklch(96.9% .015 12.422);"></div></td></tr>
    <tr><td>border-rose-100</td><td>border-color: oklch(94.1% .03 12.58);</td><td><div style="border-color: oklch(94.1% .03 12.58);"></div></td></tr>
    <tr><td>border-rose-200</td><td>border-color: oklch(89.2% .058 10.001);</td><td><div style="border-color: oklch(89.2% .058 10.001);"></div></td></tr>
    <tr><td>border-rose-300</td><td>border-color: oklch(81% .117 11.638);</td><td><div style="border-color: oklch(81% .117 11.638);"></div></td></tr>
    <tr><td>border-rose-400</td><td>border-color: oklch(71.2% .194 13.428);</td><td><div style="border-color: oklch(71.2% .194 13.428);"></div></td></tr>
    <tr><td>border-rose-500</td><td>border-color: oklch(64.5% .246 16.439);</td><td><div style="border-color: oklch(64.5% .246 16.439);"></div></td></tr>
    <tr><td>border-rose-600</td><td>border-color: oklch(58.6% .253 17.585);</td><td><div style="border-color: oklch(58.6% .253 17.585);"></div></td></tr>
    <tr><td>border-rose-700</td><td>border-color: oklch(51.4% .222 16.935);</td><td><div style="border-color: oklch(51.4% .222 16.935);"></div></td></tr>
    <tr><td>border-rose-800</td><td>border-color: oklch(45.5% .188 13.697);</td><td><div style="border-color: oklch(45.5% .188 13.697);"></div></td></tr>
    <tr><td>border-rose-900</td><td>border-color: oklch(41% .159 10.272);</td><td><div style="border-color: oklch(41% .159 10.272);"></div></td></tr>
    <tr><td>border-rose-950</td><td>border-color: oklch(27.1% .105 12.094);</td><td><div style="border-color: oklch(27.1% .105 12.094);"></div></td></tr>
    <tr><td>border-slate-50</td><td>border-color: oklch(98.4% .003 247.858);</td><td><div style="border-color: oklch(98.4% .003 247.858);"></div></td></tr>
    <tr><td>border-slate-100</td><td>border-color: oklch(96.8% .007 247.896);</td><td><div style="border-color: oklch(96.8% .007 247.896);"></div></td></tr>
    <tr><td>border-slate-200</td><td>border-color: oklch(92.9% .013 255.508);</td><td><div style="border-color: oklch(92.9% .013 255.508);"></div></td></tr>
    <tr><td>border-slate-300</td><td>border-color: oklch(86.9% .022 252.894);</td><td><div style="border-color: oklch(86.9% .022 252.894);"></div></td></tr>
    <tr><td>border-slate-400</td><td>border-color: oklch(70.4% .04 256.788);</td><td><div style="border-color: oklch(70.4% .04 256.788);"></div></td></tr>
    <tr><td>border-slate-500</td><td>border-color: oklch(55.4% .046 257.417);</td><td><div style="border-color: oklch(55.4% .046 257.417);"></div></td></tr>
    <tr><td>border-slate-600</td><td>border-color: oklch(44.6% .043 257.281);</td><td><div style="border-color: oklch(44.6% .043 257.281);"></div></td></tr>
    <tr><td>border-slate-700</td><td>border-color: oklch(37.2% .044 257.287);</td><td><div style="border-color: oklch(37.2% .044 257.287);"></div></td></tr>
    <tr><td>border-slate-800</td><td>border-color: oklch(27.9% .041 260.031);</td><td><div style="border-color: oklch(27.9% .041 260.031);"></div></td></tr>
    <tr><td>border-slate-900</td><td>border-color: oklch(20.8% .042 265.755);</td><td><div style="border-color: oklch(20.8% .042 265.755);"></div></td></tr>
    <tr><td>border-slate-950</td><td>border-color: oklch(12.9% .042 264.695);</td><td><div style="border-color: oklch(12.9% .042 264.695);"></div></td></tr>
    <tr><td>border-gray-50</td><td>border-color: oklch(98.5% .002 247.839);</td><td><div style="border-color: oklch(98.5% .002 247.839);"></div></td></tr>
    <tr><td>border-gray-100</td><td>border-color: oklch(96.7% .003 264.542);</td><td><div style="border-color: oklch(96.7% .003 264.542);"></div></td></tr>
    <tr><td>border-gray-200</td><td>border-color: oklch(92.8% .006 264.531);</td><td><div style="border-color: oklch(92.8% .006 264.531);"></div></td></tr>
    <tr><td>border-gray-300</td><td>border-color: oklch(87.2% .01 258.338);</td><td><div style="border-color: oklch(87.2% .01 258.338);"></div></td></tr>
    <tr><td>border-gray-400</td><td>border-color: oklch(70.7% .022 261.325);</td><td><div style="border-color: oklch(70.7% .022 261.325);"></div></td></tr>
    <tr><td>border-gray-500</td><td>border-color: oklch(55.1% .027 264.364);</td><td><div style="border-color: oklch(55.1% .027 264.364);"></div></td></tr>
    <tr><td>border-gray-600</td><td>border-color: oklch(44.6% .03 256.802);</td><td><div style="border-color: oklch(44.6% .03 256.802);"></div></td></tr>
    <tr><td>border-gray-700</td><td>border-color: oklch(37.3% .034 259.733);</td><td><div style="border-color: oklch(37.3% .034 259.733);"></div></td></tr>
    <tr><td>border-gray-800</td><td>border-color: oklch(27.8% .033 256.848);</td><td><div style="border-color: oklch(27.8% .033 256.848);"></div></td></tr>
    <tr><td>border-gray-900</td><td>border-color: oklch(21% .034 264.665);</td><td><div style="border-color: oklch(21% .034 264.665);"></div></td></tr>
    <tr><td>border-gray-950</td><td>border-color: oklch(13% .028 261.692);</td><td><div style="border-color: oklch(13% .028 261.692);"></div></td></tr>
    <tr><td>border-zinc-50</td><td>border-color: oklch(98.5% 0 0);</td><td><div style="border-color: oklch(98.5% 0 0);"></div></td></tr>
    <tr><td>border-zinc-100</td><td>border-color: oklch(96.7% .001 286.375);</td><td><div style="border-color: oklch(96.7% .001 286.375);"></div></td></tr>
    <tr><td>border-zinc-200</td><td>border-color: oklch(92% .004 286.32);</td><td><div style="border-color: oklch(92% .004 286.32);"></div></td></tr>
    <tr><td>border-zinc-300</td><td>border-color: oklch(87.1% .006 286.286);</td><td><div style="border-color: oklch(87.1% .006 286.286);"></div></td></tr>
    <tr><td>border-zinc-400</td><td>border-color: oklch(70.5% .015 286.067);</td><td><div style="border-color: oklch(70.5% .015 286.067);"></div></td></tr>
    <tr><td>border-zinc-500</td><td>border-color: oklch(55.2% .016 285.938);</td><td><div style="border-color: oklch(55.2% .016 285.938);"></div></td></tr>
    <tr><td>border-zinc-600</td><td>border-color: oklch(44.2% .017 285.786);</td><td><div style="border-color: oklch(44.2% .017 285.786);"></div></td></tr>
    <tr><td>border-zinc-700</td><td>border-color: oklch(37% .013 285.805);</td><td><div style="border-color: oklch(37% .013 285.805);"></div></td></tr>
    <tr><td>border-zinc-800</td><td>border-color: oklch(27.4% .006 286.033);</td><td><div style="border-color: oklch(27.4% .006 286.033);"></div></td></tr>
    <tr><td>border-zinc-900</td><td>border-color: oklch(21% .006 285.885);</td><td><div style="border-color: oklch(21% .006 285.885);"></div></td></tr>
    <tr><td>border-zinc-950</td><td>border-color: oklch(14.1% .005 285.823);</td><td><div style="border-color: oklch(14.1% .005 285.823);"></div></td></tr>
    <tr><td>border-neutral-50</td><td>border-color: oklch(98.5% 0 0);</td><td><div style="border-color: oklch(98.5% 0 0);"></div></td></tr>
    <tr><td>border-neutral-100</td><td>border-color: oklch(97% 0 0);</td><td><div style="border-color: oklch(97% 0 0);"></div></td></tr>
    <tr><td>border-neutral-200</td><td>border-color: oklch(92.2% 0 0);</td><td><div style="border-color: oklch(92.2% 0 0);"></div></td></tr>
    <tr><td>border-neutral-300</td><td>border-color: oklch(87% 0 0);</td><td><div style="border-color: oklch(87% 0 0);"></div></td></tr>
    <tr><td>border-neutral-400</td><td>border-color: oklch(70.8% 0 0);</td><td><div style="border-color: oklch(70.8% 0 0);"></div></td></tr>
    <tr><td>border-neutral-500</td><td>border-color: oklch(55.6% 0 0);</td><td><div style="border-color: oklch(55.6% 0 0);"></div></td></tr>
    <tr><td>border-neutral-600</td><td>border-color: oklch(43.9% 0 0);</td><td><div style="border-color: oklch(43.9% 0 0);"></div></td></tr>
    <tr><td>border-neutral-700</td><td>border-color: oklch(37.1% 0 0);</td><td><div style="border-color: oklch(37.1% 0 0);"></div></td></tr>
    <tr><td>border-neutral-800</td><td>border-color: oklch(26.9% 0 0);</td><td><div style="border-color: oklch(26.9% 0 0);"></div></td></tr>
    <tr><td>border-neutral-900</td><td>border-color: oklch(20.5% 0 0);</td><td><div style="border-color: oklch(20.5% 0 0);"></div></td></tr>
    <tr><td>border-neutral-950</td><td>border-color: oklch(14.5% 0 0);</td><td><div style="border-color: oklch(14.5% 0 0);"></div></td></tr>
    <tr><td>border-stone-50</td><td>border-color: oklch(98.5% .001 106.423);</td><td><div style="border-color: oklch(98.5% .001 106.423);"></div></td></tr>
    <tr><td>border-stone-100</td><td>border-color: oklch(97% .001 106.424);</td><td><div style="border-color: oklch(97% .001 106.424);"></div></td></tr>
    <tr><td>border-stone-200</td><td>border-color: oklch(92.3% .003 48.717);</td><td><div style="border-color: oklch(92.3% .003 48.717);"></div></td></tr>
    <tr><td>border-stone-300</td><td>border-color: oklch(86.9% .005 56.366);</td><td><div style="border-color: oklch(86.9% .005 56.366);"></div></td></tr>
    <tr><td>border-stone-400</td><td>border-color: oklch(70.9% .01 56.259);</td><td><div style="border-color: oklch(70.9% .01 56.259);"></div></td></tr>
    <tr><td>border-stone-500</td><td>border-color: oklch(55.3% .013 58.071);</td><td><div style="border-color: oklch(55.3% .013 58.071);"></div></td></tr>
    <tr><td>border-stone-600</td><td>border-color: oklch(44.4% .011 73.639);</td><td><div style="border-color: oklch(44.4% .011 73.639);"></div></td></tr>
    <tr><td>border-stone-700</td><td>border-color: oklch(37.4% .01 67.558);</td><td><div style="border-color: oklch(37.4% .01 67.558);"></div></td></tr>
    <tr><td>border-stone-800</td><td>border-color: oklch(26.8% .007 34.298);</td><td><div style="border-color: oklch(26.8% .007 34.298);"></div></td></tr>
    <tr><td>border-stone-900</td><td>border-color: oklch(21.6% .006 56.043);</td><td><div style="border-color: oklch(21.6% .006 56.043);"></div></td></tr>
    <tr><td>border-stone-950</td><td>border-color: oklch(14.7% .004 49.25);</td><td><div style="border-color: oklch(14.7% .004 49.25);"></div></td></tr>
    <tr><td>border-black</td><td>border-color: #000;</td><td><div style="border-color: #000;"></div></td></tr>
    <tr><td>border-white</td><td>border-color: #fff;</td><td><div style="border-color: #fff;"></div></td></tr>
  </tbody>
</table>

### Note about the opacity syntax

The color opacity can be changed using the new syntax: e.g. `border-red-500/25` will generate
`border-color: rgb(239 68 68 / 0.25);`.

### Arbitrary values

Any [`<color>`](crate::utils::value_matchers::is_matching_color) property is allowed as arbitrary value.
For example, `border-[hsla(100,100%,50%,1)]`.

[Tailwind reference](https://tailwindcss.com/docs/border-color)
