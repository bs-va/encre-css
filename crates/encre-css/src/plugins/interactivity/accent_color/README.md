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
    <tr><td>accent-red-50</td><td>accent-color: oklch(97.1% .013 17.38);</td><td style="accent-color: oklch(97.1% .013 17.38);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-100</td><td>accent-color: oklch(93.6% .032 17.717);</td><td style="accent-color: oklch(93.6% .032 17.717);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-200</td><td>accent-color: oklch(88.5% .062 18.334);</td><td style="accent-color: oklch(88.5% .062 18.334);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-300</td><td>accent-color: oklch(80.8% .114 19.571);</td><td style="accent-color: oklch(80.8% .114 19.571);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-400</td><td>accent-color: oklch(70.4% .191 22.216);</td><td style="accent-color: oklch(70.4% .191 22.216);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-500</td><td>accent-color: oklch(63.7% .237 25.331);</td><td style="accent-color: oklch(63.7% .237 25.331);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-600</td><td>accent-color: oklch(57.7% .245 27.325);</td><td style="accent-color: oklch(57.7% .245 27.325);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-700</td><td>accent-color: oklch(50.5% .213 27.518);</td><td style="accent-color: oklch(50.5% .213 27.518);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-800</td><td>accent-color: oklch(44.4% .177 26.899);</td><td style="accent-color: oklch(44.4% .177 26.899);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-900</td><td>accent-color: oklch(39.6% .141 25.723);</td><td style="accent-color: oklch(39.6% .141 25.723);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-red-950</td><td>accent-color: oklch(25.8% .092 26.042);</td><td style="accent-color: oklch(25.8% .092 26.042);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-50</td><td>accent-color: oklch(98% .016 73.684);</td><td style="accent-color: oklch(98% .016 73.684);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-100</td><td>accent-color: oklch(95.4% .038 75.164);</td><td style="accent-color: oklch(95.4% .038 75.164);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-200</td><td>accent-color: oklch(90.1% .076 70.697);</td><td style="accent-color: oklch(90.1% .076 70.697);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-300</td><td>accent-color: oklch(83.7% .128 66.29);</td><td style="accent-color: oklch(83.7% .128 66.29);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-400</td><td>accent-color: oklch(75% .183 55.934);</td><td style="accent-color: oklch(75% .183 55.934);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-500</td><td>accent-color: oklch(70.5% .213 47.604);</td><td style="accent-color: oklch(70.5% .213 47.604);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-600</td><td>accent-color: oklch(64.6% .222 41.116);</td><td style="accent-color: oklch(64.6% .222 41.116);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-700</td><td>accent-color: oklch(55.3% .195 38.402);</td><td style="accent-color: oklch(55.3% .195 38.402);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-800</td><td>accent-color: oklch(47% .157 37.304);</td><td style="accent-color: oklch(47% .157 37.304);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-900</td><td>accent-color: oklch(40.8% .123 38.172);</td><td style="accent-color: oklch(40.8% .123 38.172);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-orange-950</td><td>accent-color: oklch(26.6% .079 36.259);</td><td style="accent-color: oklch(26.6% .079 36.259);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-50</td><td>accent-color: oklch(98.7% .022 95.277);</td><td style="accent-color: oklch(98.7% .022 95.277);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-100</td><td>accent-color: oklch(96.2% .059 95.617);</td><td style="accent-color: oklch(96.2% .059 95.617);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-200</td><td>accent-color: oklch(92.4% .12 95.746);</td><td style="accent-color: oklch(92.4% .12 95.746);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-300</td><td>accent-color: oklch(87.9% .169 91.605);</td><td style="accent-color: oklch(87.9% .169 91.605);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-400</td><td>accent-color: oklch(82.8% .189 84.429);</td><td style="accent-color: oklch(82.8% .189 84.429);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-500</td><td>accent-color: oklch(76.9% .188 70.08);</td><td style="accent-color: oklch(76.9% .188 70.08);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-600</td><td>accent-color: oklch(66.6% .179 58.318);</td><td style="accent-color: oklch(66.6% .179 58.318);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-700</td><td>accent-color: oklch(55.5% .163 48.998);</td><td style="accent-color: oklch(55.5% .163 48.998);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-800</td><td>accent-color: oklch(47.3% .137 46.201);</td><td style="accent-color: oklch(47.3% .137 46.201);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-900</td><td>accent-color: oklch(41.4% .112 45.904);</td><td style="accent-color: oklch(41.4% .112 45.904);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-amber-950</td><td>accent-color: oklch(27.9% .077 45.635);</td><td style="accent-color: oklch(27.9% .077 45.635);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-50</td><td>accent-color: oklch(98.7% .026 102.212);</td><td style="accent-color: oklch(98.7% .026 102.212);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-100</td><td>accent-color: oklch(97.3% .071 103.193);</td><td style="accent-color: oklch(97.3% .071 103.193);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-200</td><td>accent-color: oklch(94.5% .129 101.54);</td><td style="accent-color: oklch(94.5% .129 101.54);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-300</td><td>accent-color: oklch(90.5% .182 98.111);</td><td style="accent-color: oklch(90.5% .182 98.111);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-400</td><td>accent-color: oklch(85.2% .199 91.936);</td><td style="accent-color: oklch(85.2% .199 91.936);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-500</td><td>accent-color: oklch(79.5% .184 86.047);</td><td style="accent-color: oklch(79.5% .184 86.047);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-600</td><td>accent-color: oklch(68.1% .162 75.834);</td><td style="accent-color: oklch(68.1% .162 75.834);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-700</td><td>accent-color: oklch(55.4% .135 66.442);</td><td style="accent-color: oklch(55.4% .135 66.442);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-800</td><td>accent-color: oklch(47.6% .114 61.907);</td><td style="accent-color: oklch(47.6% .114 61.907);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-900</td><td>accent-color: oklch(42.1% .095 57.708);</td><td style="accent-color: oklch(42.1% .095 57.708);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-yellow-950</td><td>accent-color: oklch(28.6% .066 53.813);</td><td style="accent-color: oklch(28.6% .066 53.813);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-50</td><td>accent-color: oklch(98.6% .031 120.757);</td><td style="accent-color: oklch(98.6% .031 120.757);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-100</td><td>accent-color: oklch(96.7% .067 122.328);</td><td style="accent-color: oklch(96.7% .067 122.328);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-200</td><td>accent-color: oklch(93.8% .127 124.321);</td><td style="accent-color: oklch(93.8% .127 124.321);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-300</td><td>accent-color: oklch(89.7% .196 126.665);</td><td style="accent-color: oklch(89.7% .196 126.665);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-400</td><td>accent-color: oklch(84.1% .238 128.85);</td><td style="accent-color: oklch(84.1% .238 128.85);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-500</td><td>accent-color: oklch(76.8% .233 130.85);</td><td style="accent-color: oklch(76.8% .233 130.85);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-600</td><td>accent-color: oklch(64.8% .2 131.684);</td><td style="accent-color: oklch(64.8% .2 131.684);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-700</td><td>accent-color: oklch(53.2% .157 131.589);</td><td style="accent-color: oklch(53.2% .157 131.589);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-800</td><td>accent-color: oklch(45.3% .124 130.933);</td><td style="accent-color: oklch(45.3% .124 130.933);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-900</td><td>accent-color: oklch(40.5% .101 131.063);</td><td style="accent-color: oklch(40.5% .101 131.063);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-lime-950</td><td>accent-color: oklch(27.4% .072 132.109);</td><td style="accent-color: oklch(27.4% .072 132.109);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-50</td><td>accent-color: oklch(98.2% .018 155.826);</td><td style="accent-color: oklch(98.2% .018 155.826);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-100</td><td>accent-color: oklch(96.2% .044 156.743);</td><td style="accent-color: oklch(96.2% .044 156.743);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-200</td><td>accent-color: oklch(92.5% .084 155.995);</td><td style="accent-color: oklch(92.5% .084 155.995);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-300</td><td>accent-color: oklch(87.1% .15 154.449);</td><td style="accent-color: oklch(87.1% .15 154.449);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-400</td><td>accent-color: oklch(79.2% .209 151.711);</td><td style="accent-color: oklch(79.2% .209 151.711);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-500</td><td>accent-color: oklch(72.3% .219 149.579);</td><td style="accent-color: oklch(72.3% .219 149.579);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-600</td><td>accent-color: oklch(62.7% .194 149.214);</td><td style="accent-color: oklch(62.7% .194 149.214);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-700</td><td>accent-color: oklch(52.7% .154 150.069);</td><td style="accent-color: oklch(52.7% .154 150.069);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-800</td><td>accent-color: oklch(44.8% .119 151.328);</td><td style="accent-color: oklch(44.8% .119 151.328);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-900</td><td>accent-color: oklch(39.3% .095 152.535);</td><td style="accent-color: oklch(39.3% .095 152.535);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-green-950</td><td>accent-color: oklch(26.6% .065 152.934);</td><td style="accent-color: oklch(26.6% .065 152.934);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-50</td><td>accent-color: oklch(97.9% .021 166.113);</td><td style="accent-color: oklch(97.9% .021 166.113);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-100</td><td>accent-color: oklch(95% .052 163.051);</td><td style="accent-color: oklch(95% .052 163.051);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-200</td><td>accent-color: oklch(90.5% .093 164.15);</td><td style="accent-color: oklch(90.5% .093 164.15);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-300</td><td>accent-color: oklch(84.5% .143 164.978);</td><td style="accent-color: oklch(84.5% .143 164.978);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-400</td><td>accent-color: oklch(76.5% .177 163.223);</td><td style="accent-color: oklch(76.5% .177 163.223);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-500</td><td>accent-color: oklch(69.6% .17 162.48);</td><td style="accent-color: oklch(69.6% .17 162.48);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-600</td><td>accent-color: oklch(59.6% .145 163.225);</td><td style="accent-color: oklch(59.6% .145 163.225);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-700</td><td>accent-color: oklch(50.8% .118 165.612);</td><td style="accent-color: oklch(50.8% .118 165.612);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-800</td><td>accent-color: oklch(43.2% .095 166.913);</td><td style="accent-color: oklch(43.2% .095 166.913);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-900</td><td>accent-color: oklch(37.8% .077 168.94);</td><td style="accent-color: oklch(37.8% .077 168.94);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-emerald-950</td><td>accent-color: oklch(26.2% .051 172.552);</td><td style="accent-color: oklch(26.2% .051 172.552);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-50</td><td>accent-color: oklch(98.4% .014 180.72);</td><td style="accent-color: oklch(98.4% .014 180.72);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-100</td><td>accent-color: oklch(95.3% .051 180.801);</td><td style="accent-color: oklch(95.3% .051 180.801);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-200</td><td>accent-color: oklch(91% .096 180.426);</td><td style="accent-color: oklch(91% .096 180.426);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-300</td><td>accent-color: oklch(85.5% .138 181.071);</td><td style="accent-color: oklch(85.5% .138 181.071);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-400</td><td>accent-color: oklch(77.7% .152 181.912);</td><td style="accent-color: oklch(77.7% .152 181.912);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-500</td><td>accent-color: oklch(70.4% .14 182.503);</td><td style="accent-color: oklch(70.4% .14 182.503);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-600</td><td>accent-color: oklch(60% .118 184.704);</td><td style="accent-color: oklch(60% .118 184.704);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-700</td><td>accent-color: oklch(51.1% .096 186.391);</td><td style="accent-color: oklch(51.1% .096 186.391);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-800</td><td>accent-color: oklch(43.7% .078 188.216);</td><td style="accent-color: oklch(43.7% .078 188.216);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-900</td><td>accent-color: oklch(38.6% .063 188.416);</td><td style="accent-color: oklch(38.6% .063 188.416);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-teal-950</td><td>accent-color: oklch(27.7% .046 192.524);</td><td style="accent-color: oklch(27.7% .046 192.524);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-50</td><td>accent-color: oklch(98.4% .019 200.873);</td><td style="accent-color: oklch(98.4% .019 200.873);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-100</td><td>accent-color: oklch(95.6% .045 203.388);</td><td style="accent-color: oklch(95.6% .045 203.388);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-200</td><td>accent-color: oklch(91.7% .08 205.041);</td><td style="accent-color: oklch(91.7% .08 205.041);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-300</td><td>accent-color: oklch(86.5% .127 207.078);</td><td style="accent-color: oklch(86.5% .127 207.078);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-400</td><td>accent-color: oklch(78.9% .154 211.53);</td><td style="accent-color: oklch(78.9% .154 211.53);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-500</td><td>accent-color: oklch(71.5% .143 215.221);</td><td style="accent-color: oklch(71.5% .143 215.221);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-600</td><td>accent-color: oklch(60.9% .126 221.723);</td><td style="accent-color: oklch(60.9% .126 221.723);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-700</td><td>accent-color: oklch(52% .105 223.128);</td><td style="accent-color: oklch(52% .105 223.128);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-800</td><td>accent-color: oklch(45% .085 224.283);</td><td style="accent-color: oklch(45% .085 224.283);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-900</td><td>accent-color: oklch(39.8% .07 227.392);</td><td style="accent-color: oklch(39.8% .07 227.392);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-cyan-950</td><td>accent-color: oklch(30.2% .056 229.695);</td><td style="accent-color: oklch(30.2% .056 229.695);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-50</td><td>accent-color: oklch(97.7% .013 236.62);</td><td style="accent-color: oklch(97.7% .013 236.62);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-100</td><td>accent-color: oklch(95.1% .026 236.824);</td><td style="accent-color: oklch(95.1% .026 236.824);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-200</td><td>accent-color: oklch(90.1% .058 230.902);</td><td style="accent-color: oklch(90.1% .058 230.902);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-300</td><td>accent-color: oklch(82.8% .111 230.318);</td><td style="accent-color: oklch(82.8% .111 230.318);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-400</td><td>accent-color: oklch(74.6% .16 232.661);</td><td style="accent-color: oklch(74.6% .16 232.661);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-500</td><td>accent-color: oklch(68.5% .169 237.323);</td><td style="accent-color: oklch(68.5% .169 237.323);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-600</td><td>accent-color: oklch(58.8% .158 241.966);</td><td style="accent-color: oklch(58.8% .158 241.966);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-700</td><td>accent-color: oklch(50% .134 242.749);</td><td style="accent-color: oklch(50% .134 242.749);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-800</td><td>accent-color: oklch(44.3% .11 240.79);</td><td style="accent-color: oklch(44.3% .11 240.79);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-900</td><td>accent-color: oklch(39.1% .09 240.876);</td><td style="accent-color: oklch(39.1% .09 240.876);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-sky-950</td><td>accent-color: oklch(29.3% .066 243.157);</td><td style="accent-color: oklch(29.3% .066 243.157);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-50</td><td>accent-color: oklch(97% .014 254.604);</td><td style="accent-color: oklch(97% .014 254.604);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-100</td><td>accent-color: oklch(93.2% .032 255.585);</td><td style="accent-color: oklch(93.2% .032 255.585);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-200</td><td>accent-color: oklch(88.2% .059 254.128);</td><td style="accent-color: oklch(88.2% .059 254.128);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-300</td><td>accent-color: oklch(80.9% .105 251.813);</td><td style="accent-color: oklch(80.9% .105 251.813);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-400</td><td>accent-color: oklch(70.7% .165 254.624);</td><td style="accent-color: oklch(70.7% .165 254.624);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-500</td><td>accent-color: oklch(62.3% .214 259.815);</td><td style="accent-color: oklch(62.3% .214 259.815);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-600</td><td>accent-color: oklch(54.6% .245 262.881);</td><td style="accent-color: oklch(54.6% .245 262.881);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-700</td><td>accent-color: oklch(48.8% .243 264.376);</td><td style="accent-color: oklch(48.8% .243 264.376);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-800</td><td>accent-color: oklch(42.4% .199 265.638);</td><td style="accent-color: oklch(42.4% .199 265.638);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-900</td><td>accent-color: oklch(37.9% .146 265.522);</td><td style="accent-color: oklch(37.9% .146 265.522);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-blue-950</td><td>accent-color: oklch(28.2% .091 267.935);</td><td style="accent-color: oklch(28.2% .091 267.935);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-50</td><td>accent-color: oklch(96.2% .018 272.314);</td><td style="accent-color: oklch(96.2% .018 272.314);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-100</td><td>accent-color: oklch(93% .034 272.788);</td><td style="accent-color: oklch(93% .034 272.788);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-200</td><td>accent-color: oklch(87% .065 274.039);</td><td style="accent-color: oklch(87% .065 274.039);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-300</td><td>accent-color: oklch(78.5% .115 274.713);</td><td style="accent-color: oklch(78.5% .115 274.713);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-400</td><td>accent-color: oklch(67.3% .182 276.935);</td><td style="accent-color: oklch(67.3% .182 276.935);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-500</td><td>accent-color: oklch(58.5% .233 277.117);</td><td style="accent-color: oklch(58.5% .233 277.117);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-600</td><td>accent-color: oklch(51.1% .262 276.966);</td><td style="accent-color: oklch(51.1% .262 276.966);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-700</td><td>accent-color: oklch(45.7% .24 277.023);</td><td style="accent-color: oklch(45.7% .24 277.023);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-800</td><td>accent-color: oklch(39.8% .195 277.366);</td><td style="accent-color: oklch(39.8% .195 277.366);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-900</td><td>accent-color: oklch(35.9% .144 278.697);</td><td style="accent-color: oklch(35.9% .144 278.697);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-indigo-950</td><td>accent-color: oklch(25.7% .09 281.288);</td><td style="accent-color: oklch(25.7% .09 281.288);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-50</td><td>accent-color: oklch(96.9% .016 293.756);</td><td style="accent-color: oklch(96.9% .016 293.756);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-100</td><td>accent-color: oklch(94.3% .029 294.588);</td><td style="accent-color: oklch(94.3% .029 294.588);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-200</td><td>accent-color: oklch(89.4% .057 293.283);</td><td style="accent-color: oklch(89.4% .057 293.283);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-300</td><td>accent-color: oklch(81.1% .111 293.571);</td><td style="accent-color: oklch(81.1% .111 293.571);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-400</td><td>accent-color: oklch(70.2% .183 293.541);</td><td style="accent-color: oklch(70.2% .183 293.541);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-500</td><td>accent-color: oklch(60.6% .25 292.717);</td><td style="accent-color: oklch(60.6% .25 292.717);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-600</td><td>accent-color: oklch(54.1% .281 293.009);</td><td style="accent-color: oklch(54.1% .281 293.009);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-700</td><td>accent-color: oklch(49.1% .27 292.581);</td><td style="accent-color: oklch(49.1% .27 292.581);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-800</td><td>accent-color: oklch(43.2% .232 292.759);</td><td style="accent-color: oklch(43.2% .232 292.759);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-900</td><td>accent-color: oklch(38% .189 293.745);</td><td style="accent-color: oklch(38% .189 293.745);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-violet-950</td><td>accent-color: oklch(28.3% .141 291.089);</td><td style="accent-color: oklch(28.3% .141 291.089);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-50</td><td>accent-color: oklch(97.7% .014 308.299);</td><td style="accent-color: oklch(97.7% .014 308.299);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-100</td><td>accent-color: oklch(94.6% .033 307.174);</td><td style="accent-color: oklch(94.6% .033 307.174);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-200</td><td>accent-color: oklch(90.2% .063 306.703);</td><td style="accent-color: oklch(90.2% .063 306.703);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-300</td><td>accent-color: oklch(82.7% .119 306.383);</td><td style="accent-color: oklch(82.7% .119 306.383);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-400</td><td>accent-color: oklch(71.4% .203 305.504);</td><td style="accent-color: oklch(71.4% .203 305.504);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-500</td><td>accent-color: oklch(62.7% .265 303.9);</td><td style="accent-color: oklch(62.7% .265 303.9);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-600</td><td>accent-color: oklch(55.8% .288 302.321);</td><td style="accent-color: oklch(55.8% .288 302.321);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-700</td><td>accent-color: oklch(49.6% .265 301.924);</td><td style="accent-color: oklch(49.6% .265 301.924);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-800</td><td>accent-color: oklch(43.8% .218 303.724);</td><td style="accent-color: oklch(43.8% .218 303.724);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-900</td><td>accent-color: oklch(38.1% .176 304.987);</td><td style="accent-color: oklch(38.1% .176 304.987);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-purple-950</td><td>accent-color: oklch(29.1% .149 302.717);</td><td style="accent-color: oklch(29.1% .149 302.717);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-50</td><td>accent-color: oklch(97.7% .017 320.058);</td><td style="accent-color: oklch(97.7% .017 320.058);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-100</td><td>accent-color: oklch(95.2% .037 318.852);</td><td style="accent-color: oklch(95.2% .037 318.852);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-200</td><td>accent-color: oklch(90.3% .076 319.62);</td><td style="accent-color: oklch(90.3% .076 319.62);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-300</td><td>accent-color: oklch(83.3% .145 321.434);</td><td style="accent-color: oklch(83.3% .145 321.434);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-400</td><td>accent-color: oklch(74% .238 322.16);</td><td style="accent-color: oklch(74% .238 322.16);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-500</td><td>accent-color: oklch(66.7% .295 322.15);</td><td style="accent-color: oklch(66.7% .295 322.15);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-600</td><td>accent-color: oklch(59.1% .293 322.896);</td><td style="accent-color: oklch(59.1% .293 322.896);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-700</td><td>accent-color: oklch(51.8% .253 323.949);</td><td style="accent-color: oklch(51.8% .253 323.949);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-800</td><td>accent-color: oklch(45.2% .211 324.591);</td><td style="accent-color: oklch(45.2% .211 324.591);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-900</td><td>accent-color: oklch(40.1% .17 325.612);</td><td style="accent-color: oklch(40.1% .17 325.612);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-fuchsia-950</td><td>accent-color: oklch(29.3% .136 325.661);</td><td style="accent-color: oklch(29.3% .136 325.661);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-50</td><td>accent-color: oklch(97.1% .014 343.198);</td><td style="accent-color: oklch(97.1% .014 343.198);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-100</td><td>accent-color: oklch(94.8% .028 342.258);</td><td style="accent-color: oklch(94.8% .028 342.258);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-200</td><td>accent-color: oklch(89.9% .061 343.231);</td><td style="accent-color: oklch(89.9% .061 343.231);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-300</td><td>accent-color: oklch(82.3% .12 346.018);</td><td style="accent-color: oklch(82.3% .12 346.018);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-400</td><td>accent-color: oklch(71.8% .202 349.761);</td><td style="accent-color: oklch(71.8% .202 349.761);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-500</td><td>accent-color: oklch(65.6% .241 354.308);</td><td style="accent-color: oklch(65.6% .241 354.308);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-600</td><td>accent-color: oklch(59.2% .249 .584);</td><td style="accent-color: oklch(59.2% .249 .584);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-700</td><td>accent-color: oklch(52.5% .223 3.958);</td><td style="accent-color: oklch(52.5% .223 3.958);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-800</td><td>accent-color: oklch(45.9% .187 3.815);</td><td style="accent-color: oklch(45.9% .187 3.815);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-900</td><td>accent-color: oklch(40.8% .153 2.432);</td><td style="accent-color: oklch(40.8% .153 2.432);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-pink-950</td><td>accent-color: oklch(28.4% .109 3.907);</td><td style="accent-color: oklch(28.4% .109 3.907);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-50</td><td>accent-color: oklch(96.9% .015 12.422);</td><td style="accent-color: oklch(96.9% .015 12.422);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-100</td><td>accent-color: oklch(94.1% .03 12.58);</td><td style="accent-color: oklch(94.1% .03 12.58);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-200</td><td>accent-color: oklch(89.2% .058 10.001);</td><td style="accent-color: oklch(89.2% .058 10.001);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-300</td><td>accent-color: oklch(81% .117 11.638);</td><td style="accent-color: oklch(81% .117 11.638);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-400</td><td>accent-color: oklch(71.2% .194 13.428);</td><td style="accent-color: oklch(71.2% .194 13.428);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-500</td><td>accent-color: oklch(64.5% .246 16.439);</td><td style="accent-color: oklch(64.5% .246 16.439);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-600</td><td>accent-color: oklch(58.6% .253 17.585);</td><td style="accent-color: oklch(58.6% .253 17.585);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-700</td><td>accent-color: oklch(51.4% .222 16.935);</td><td style="accent-color: oklch(51.4% .222 16.935);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-800</td><td>accent-color: oklch(45.5% .188 13.697);</td><td style="accent-color: oklch(45.5% .188 13.697);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-900</td><td>accent-color: oklch(41% .159 10.272);</td><td style="accent-color: oklch(41% .159 10.272);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-rose-950</td><td>accent-color: oklch(27.1% .105 12.094);</td><td style="accent-color: oklch(27.1% .105 12.094);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-50</td><td>accent-color: oklch(98.4% .003 247.858);</td><td style="accent-color: oklch(98.4% .003 247.858);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-100</td><td>accent-color: oklch(96.8% .007 247.896);</td><td style="accent-color: oklch(96.8% .007 247.896);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-200</td><td>accent-color: oklch(92.9% .013 255.508);</td><td style="accent-color: oklch(92.9% .013 255.508);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-300</td><td>accent-color: oklch(86.9% .022 252.894);</td><td style="accent-color: oklch(86.9% .022 252.894);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-400</td><td>accent-color: oklch(70.4% .04 256.788);</td><td style="accent-color: oklch(70.4% .04 256.788);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-500</td><td>accent-color: oklch(55.4% .046 257.417);</td><td style="accent-color: oklch(55.4% .046 257.417);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-600</td><td>accent-color: oklch(44.6% .043 257.281);</td><td style="accent-color: oklch(44.6% .043 257.281);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-700</td><td>accent-color: oklch(37.2% .044 257.287);</td><td style="accent-color: oklch(37.2% .044 257.287);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-800</td><td>accent-color: oklch(27.9% .041 260.031);</td><td style="accent-color: oklch(27.9% .041 260.031);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-900</td><td>accent-color: oklch(20.8% .042 265.755);</td><td style="accent-color: oklch(20.8% .042 265.755);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-slate-950</td><td>accent-color: oklch(12.9% .042 264.695);</td><td style="accent-color: oklch(12.9% .042 264.695);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-50</td><td>accent-color: oklch(98.5% .002 247.839);</td><td style="accent-color: oklch(98.5% .002 247.839);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-100</td><td>accent-color: oklch(96.7% .003 264.542);</td><td style="accent-color: oklch(96.7% .003 264.542);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-200</td><td>accent-color: oklch(92.8% .006 264.531);</td><td style="accent-color: oklch(92.8% .006 264.531);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-300</td><td>accent-color: oklch(87.2% .01 258.338);</td><td style="accent-color: oklch(87.2% .01 258.338);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-400</td><td>accent-color: oklch(70.7% .022 261.325);</td><td style="accent-color: oklch(70.7% .022 261.325);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-500</td><td>accent-color: oklch(55.1% .027 264.364);</td><td style="accent-color: oklch(55.1% .027 264.364);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-600</td><td>accent-color: oklch(44.6% .03 256.802);</td><td style="accent-color: oklch(44.6% .03 256.802);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-700</td><td>accent-color: oklch(37.3% .034 259.733);</td><td style="accent-color: oklch(37.3% .034 259.733);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-800</td><td>accent-color: oklch(27.8% .033 256.848);</td><td style="accent-color: oklch(27.8% .033 256.848);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-900</td><td>accent-color: oklch(21% .034 264.665);</td><td style="accent-color: oklch(21% .034 264.665);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-gray-950</td><td>accent-color: oklch(13% .028 261.692);</td><td style="accent-color: oklch(13% .028 261.692);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-50</td><td>accent-color: oklch(98.5% 0 0);</td><td style="accent-color: oklch(98.5% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-100</td><td>accent-color: oklch(96.7% .001 286.375);</td><td style="accent-color: oklch(96.7% .001 286.375);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-200</td><td>accent-color: oklch(92% .004 286.32);</td><td style="accent-color: oklch(92% .004 286.32);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-300</td><td>accent-color: oklch(87.1% .006 286.286);</td><td style="accent-color: oklch(87.1% .006 286.286);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-400</td><td>accent-color: oklch(70.5% .015 286.067);</td><td style="accent-color: oklch(70.5% .015 286.067);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-500</td><td>accent-color: oklch(55.2% .016 285.938);</td><td style="accent-color: oklch(55.2% .016 285.938);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-600</td><td>accent-color: oklch(44.2% .017 285.786);</td><td style="accent-color: oklch(44.2% .017 285.786);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-700</td><td>accent-color: oklch(37% .013 285.805);</td><td style="accent-color: oklch(37% .013 285.805);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-800</td><td>accent-color: oklch(27.4% .006 286.033);</td><td style="accent-color: oklch(27.4% .006 286.033);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-900</td><td>accent-color: oklch(21% .006 285.885);</td><td style="accent-color: oklch(21% .006 285.885);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-zinc-950</td><td>accent-color: oklch(14.1% .005 285.823);</td><td style="accent-color: oklch(14.1% .005 285.823);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-50</td><td>accent-color: oklch(98.5% 0 0);</td><td style="accent-color: oklch(98.5% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-100</td><td>accent-color: oklch(97% 0 0);</td><td style="accent-color: oklch(97% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-200</td><td>accent-color: oklch(92.2% 0 0);</td><td style="accent-color: oklch(92.2% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-300</td><td>accent-color: oklch(87% 0 0);</td><td style="accent-color: oklch(87% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-400</td><td>accent-color: oklch(70.8% 0 0);</td><td style="accent-color: oklch(70.8% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-500</td><td>accent-color: oklch(55.6% 0 0);</td><td style="accent-color: oklch(55.6% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-600</td><td>accent-color: oklch(43.9% 0 0);</td><td style="accent-color: oklch(43.9% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-700</td><td>accent-color: oklch(37.1% 0 0);</td><td style="accent-color: oklch(37.1% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-800</td><td>accent-color: oklch(26.9% 0 0);</td><td style="accent-color: oklch(26.9% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-900</td><td>accent-color: oklch(20.5% 0 0);</td><td style="accent-color: oklch(20.5% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-neutral-950</td><td>accent-color: oklch(14.5% 0 0);</td><td style="accent-color: oklch(14.5% 0 0);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-50</td><td>accent-color: oklch(98.5% .001 106.423);</td><td style="accent-color: oklch(98.5% .001 106.423);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-100</td><td>accent-color: oklch(97% .001 106.424);</td><td style="accent-color: oklch(97% .001 106.424);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-200</td><td>accent-color: oklch(92.3% .003 48.717);</td><td style="accent-color: oklch(92.3% .003 48.717);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-300</td><td>accent-color: oklch(86.9% .005 56.366);</td><td style="accent-color: oklch(86.9% .005 56.366);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-400</td><td>accent-color: oklch(70.9% .01 56.259);</td><td style="accent-color: oklch(70.9% .01 56.259);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-500</td><td>accent-color: oklch(55.3% .013 58.071);</td><td style="accent-color: oklch(55.3% .013 58.071);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-600</td><td>accent-color: oklch(44.4% .011 73.639);</td><td style="accent-color: oklch(44.4% .011 73.639);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-700</td><td>accent-color: oklch(37.4% .01 67.558);</td><td style="accent-color: oklch(37.4% .01 67.558);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-800</td><td>accent-color: oklch(26.8% .007 34.298);</td><td style="accent-color: oklch(26.8% .007 34.298);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-900</td><td>accent-color: oklch(21.6% .006 56.043);</td><td style="accent-color: oklch(21.6% .006 56.043);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-stone-950</td><td>accent-color: oklch(14.7% .004 49.25);</td><td style="accent-color: oklch(14.7% .004 49.25);"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-black</td><td>accent-color: #000;</td><td style="accent-color: #000;"><input type="checkbox" checked /></td></tr>
    <tr><td>accent-white</td><td>accent-color: #fff;</td><td style="accent-color: #fff;"><input type="checkbox" checked /></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<color>`](crate::utils::value_matchers::is_matching_color) property is allowed as arbitrary value.
For example, `accent-[hsl(135,100%,50%)]`.

[Tailwind reference](https://tailwindcss.com/docs/accent-color)
