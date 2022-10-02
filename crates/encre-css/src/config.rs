//! Define the [`Config`] structure used to configure an [`EncreGenerator`] using a
//! [Tailwind-like configuration](https://tailwindcss.com/docs/configuration).
//!
//! # Example
//!
//! ```rust
//! use encre_css::{EncreGenerator, Config, config::DarkMode};
//!
//! let mut config = Config::default();
//!
//! // Toggles the dark mode using the class `.dark`
//! config.theme.dark_mode = DarkMode::new_class(".dark");
//!
//! // Defines some custom colors, they will be usable in the `text`, `bg`,
//! // `border`, etc utilities.
//! config.theme.colors.add("primary", "#d3198c");
//! config.theme.colors.add("secondary", "#fff");
//!
//! // Defines some custom screen breakpoints
//! config.theme.screens.add("tablet", "640px");
//! config.theme.screens.add("laptop", "1024px");
//! config.theme.screens.add("desktop", "1280px");
//!
//! let mut generator = EncreGenerator::from_config(config);
//! generator.add_selector("tablet:dark:bg-primary");
//!
//! assert!(generator.generate().ends_with(r#"@media (min-width: 640px) {
//!   .dark .tablet\:dark\:bg-primary {
//!     --en-bg-opacity: 1;
//!     background-color: rgb(211 25 140 / var(--en-bg-opacity));
//!   }
//! }"#));
//! ```
//!
//! The previous example is equivalent to the following TOML configuration file:
//!
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[theme]</span>
//! dark_mode = { class = <span class="string">".dark"</span> }
//! colors = { primary = <span class="string">"#d3198c"</span>, secondary = <span class="string">"#fff"</span> }
//! screens = { tablet = <span class="string">"640px"</span>, laptop = <span class="string">"1024px"</span>, desktop = <span class="string">"1280px"</span> }
//! </code></pre></div>
//!
//! [`EncreGenerator`]: crate::EncreGenerator
use crate::{
    error::{Error, Result},
    preflight::Preflight,
    scanner::Scanner,
    selector::VariantType,
};

#[allow(clippy::wildcard_imports)]
use crate::plugins::*;

use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt, fs, iter,
    path::Path,
    sync::{Mutex, MutexGuard},
};

/// The list of all default colors.
///
/// <table style="display: table;">
///     <thead>
///         <tr>
///             <th style="text-align: center;">Name</th>
///             <th style="text-align: center;">RGB Value</th>
///             <th style="text-align: center;">Color</th>
///         </tr>
///     </thead>
///     <tbody>
///         <tr><td>slate-50</td><td>rgb(248 250 252)</td><td style="background-color: rgb(248 250 252);"></td></tr>
///         <tr><td>slate-100</td><td>rgb(241 245 249)</td><td style="background-color: rgb(241 245 249);"></td></tr>
///         <tr><td>slate-200</td><td>rgb(226 232 240)</td><td style="background-color: rgb(226 232 240);"></td></tr>
///         <tr><td>slate-300</td><td>rgb(203 213 225)</td><td style="background-color: rgb(203 213 225);"></td></tr>
///         <tr><td>slate-400</td><td>rgb(148 163 184)</td><td style="background-color: rgb(148 163 184);"></td></tr>
///         <tr><td>slate-500</td><td>rgb(100 116 139)</td><td style="background-color: rgb(100 116 139);"></td></tr>
///         <tr><td>slate-600</td><td>rgb(71 85 105)</td><td style="background-color: rgb(71 85 105);"></td></tr>
///         <tr><td>slate-700</td><td>rgb(51 65 85)</td><td style="background-color: rgb(51 65 85);"></td></tr>
///         <tr><td>slate-800</td><td>rgb(30 41 59)</td><td style="background-color: rgb(30 41 59);"></td></tr>
///         <tr><td>slate-900</td><td>rgb(15 23 42)</td><td style="background-color: rgb(15 23 42);"></td></tr>
///         <tr><td>gray-50</td><td>rgb(249 250 251)</td><td style="background-color: rgb(249 250 251);"></td></tr>
///         <tr><td>gray-100</td><td>rgb(243 244 246)</td><td style="background-color: rgb(243 244 246);"></td></tr>
///         <tr><td>gray-200</td><td>rgb(229 231 235)</td><td style="background-color: rgb(229 231 235);"></td></tr>
///         <tr><td>gray-300</td><td>rgb(209 213 219)</td><td style="background-color: rgb(209 213 219);"></td></tr>
///         <tr><td>gray-400</td><td>rgb(156 163 175)</td><td style="background-color: rgb(156 163 175);"></td></tr>
///         <tr><td>gray-500</td><td>rgb(107 114 128)</td><td style="background-color: rgb(107 114 128);"></td></tr>
///         <tr><td>gray-600</td><td>rgb(75 85 99)</td><td style="background-color: rgb(75 85 99);"></td></tr>
///         <tr><td>gray-700</td><td>rgb(55 65 81)</td><td style="background-color: rgb(55 65 81);"></td></tr>
///         <tr><td>gray-800</td><td>rgb(31 41 55)</td><td style="background-color: rgb(31 41 55);"></td></tr>
///         <tr><td>gray-900</td><td>rgb(17 24 39)</td><td style="background-color: rgb(17 24 39);"></td></tr>
///         <tr><td>zinc-50</td><td>rgb(250 250 250)</td><td style="background-color: rgb(250 250 250);"></td></tr>
///         <tr><td>zinc-100</td><td>rgb(244 244 245)</td><td style="background-color: rgb(244 244 245);"></td></tr>
///         <tr><td>zinc-200</td><td>rgb(228 228 231)</td><td style="background-color: rgb(228 228 231);"></td></tr>
///         <tr><td>zinc-300</td><td>rgb(212 212 216)</td><td style="background-color: rgb(212 212 216);"></td></tr>
///         <tr><td>zinc-400</td><td>rgb(161 161 170)</td><td style="background-color: rgb(161 161 170);"></td></tr>
///         <tr><td>zinc-500</td><td>rgb(113 113 122)</td><td style="background-color: rgb(113 113 122);"></td></tr>
///         <tr><td>zinc-600</td><td>rgb(82 82 91)</td><td style="background-color: rgb(82 82 91);"></td></tr>
///         <tr><td>zinc-700</td><td>rgb(63 63 70)</td><td style="background-color: rgb(63 63 70);"></td></tr>
///         <tr><td>zinc-800</td><td>rgb(39 39 42)</td><td style="background-color: rgb(39 39 42);"></td></tr>
///         <tr><td>zinc-900</td><td>rgb(24 24 27)</td><td style="background-color: rgb(24 24 27);"></td></tr>
///         <tr><td>neutral-50</td><td>rgb(250 250 250)</td><td style="background-color: rgb(250 250 250);"></td></tr>
///         <tr><td>neutral-100</td><td>rgb(245 245 245)</td><td style="background-color: rgb(245 245 245);"></td></tr>
///         <tr><td>neutral-200</td><td>rgb(229 229 229)</td><td style="background-color: rgb(229 229 229);"></td></tr>
///         <tr><td>neutral-300</td><td>rgb(212 212 212)</td><td style="background-color: rgb(212 212 212);"></td></tr>
///         <tr><td>neutral-400</td><td>rgb(163 163 163)</td><td style="background-color: rgb(163 163 163);"></td></tr>
///         <tr><td>neutral-500</td><td>rgb(115 115 115)</td><td style="background-color: rgb(115 115 115);"></td></tr>
///         <tr><td>neutral-600</td><td>rgb(82 82 82)</td><td style="background-color: rgb(82 82 82);"></td></tr>
///         <tr><td>neutral-700</td><td>rgb(64 64 64)</td><td style="background-color: rgb(64 64 64);"></td></tr>
///         <tr><td>neutral-800</td><td>rgb(38 38 38)</td><td style="background-color: rgb(38 38 38);"></td></tr>
///         <tr><td>neutral-900</td><td>rgb(23 23 23)</td><td style="background-color: rgb(23 23 23);"></td></tr>
///         <tr><td>stone-50</td><td>rgb(250 250 249)</td><td style="background-color: rgb(250 250 249);"></td></tr>
///         <tr><td>stone-100</td><td>rgb(245 245 244)</td><td style="background-color: rgb(245 245 244);"></td></tr>
///         <tr><td>stone-200</td><td>rgb(231 229 228)</td><td style="background-color: rgb(231 229 228);"></td></tr>
///         <tr><td>stone-300</td><td>rgb(214 211 209)</td><td style="background-color: rgb(214 211 209);"></td></tr>
///         <tr><td>stone-400</td><td>rgb(168 162 158)</td><td style="background-color: rgb(168 162 158);"></td></tr>
///         <tr><td>stone-500</td><td>rgb(120 113 108)</td><td style="background-color: rgb(120 113 108);"></td></tr>
///         <tr><td>stone-600</td><td>rgb(87 83 78)</td><td style="background-color: rgb(87 83 78);"></td></tr>
///         <tr><td>stone-700</td><td>rgb(68 64 60)</td><td style="background-color: rgb(68 64 60);"></td></tr>
///         <tr><td>stone-800</td><td>rgb(41 37 36)</td><td style="background-color: rgb(41 37 36);"></td></tr>
///         <tr><td>stone-900</td><td>rgb(28 25 23)</td><td style="background-color: rgb(28 25 23);"></td></tr>
///         <tr><td>red-50</td><td>rgb(254 242 242)</td><td style="background-color: rgb(254 242 242);"></td></tr>
///         <tr><td>red-100</td><td>rgb(254 226 226)</td><td style="background-color: rgb(254 226 226);"></td></tr>
///         <tr><td>red-200</td><td>rgb(254 202 202)</td><td style="background-color: rgb(254 202 202);"></td></tr>
///         <tr><td>red-300</td><td>rgb(252 165 165)</td><td style="background-color: rgb(252 165 165);"></td></tr>
///         <tr><td>red-400</td><td>rgb(248 113 113)</td><td style="background-color: rgb(248 113 113);"></td></tr>
///         <tr><td>red-500</td><td>rgb(239 68 68)</td><td style="background-color: rgb(239 68 68);"></td></tr>
///         <tr><td>red-600</td><td>rgb(220 38 38)</td><td style="background-color: rgb(220 38 38);"></td></tr>
///         <tr><td>red-700</td><td>rgb(185 28 28)</td><td style="background-color: rgb(185 28 28);"></td></tr>
///         <tr><td>red-800</td><td>rgb(153 27 27)</td><td style="background-color: rgb(153 27 27);"></td></tr>
///         <tr><td>red-900</td><td>rgb(127 29 29)</td><td style="background-color: rgb(127 29 29);"></td></tr>
///         <tr><td>orange-50</td><td>rgb(255 247 237)</td><td style="background-color: rgb(255 247 237);"></td></tr>
///         <tr><td>orange-100</td><td>rgb(255 237 213)</td><td style="background-color: rgb(255 237 213);"></td></tr>
///         <tr><td>orange-200</td><td>rgb(254 215 170)</td><td style="background-color: rgb(254 215 170);"></td></tr>
///         <tr><td>orange-300</td><td>rgb(253 186 116)</td><td style="background-color: rgb(253 186 116);"></td></tr>
///         <tr><td>orange-400</td><td>rgb(251 146 60)</td><td style="background-color: rgb(251 146 60);"></td></tr>
///         <tr><td>orange-500</td><td>rgb(249 115 22)</td><td style="background-color: rgb(249 115 22);"></td></tr>
///         <tr><td>orange-600</td><td>rgb(234 88 12)</td><td style="background-color: rgb(234 88 12);"></td></tr>
///         <tr><td>orange-700</td><td>rgb(194 65 12)</td><td style="background-color: rgb(194 65 12);"></td></tr>
///         <tr><td>orange-800</td><td>rgb(154 52 18)</td><td style="background-color: rgb(154 52 18);"></td></tr>
///         <tr><td>orange-900</td><td>rgb(124 45 18)</td><td style="background-color: rgb(124 45 18);"></td></tr>
///         <tr><td>amber-50</td><td>rgb(255 251 235)</td><td style="background-color: rgb(255 251 235);"></td></tr>
///         <tr><td>amber-100</td><td>rgb(254 243 199)</td><td style="background-color: rgb(254 243 199);"></td></tr>
///         <tr><td>amber-200</td><td>rgb(253 230 138)</td><td style="background-color: rgb(253 230 138);"></td></tr>
///         <tr><td>amber-300</td><td>rgb(252 211 77)</td><td style="background-color: rgb(252 211 77);"></td></tr>
///         <tr><td>amber-400</td><td>rgb(251 191 36)</td><td style="background-color: rgb(251 191 36);"></td></tr>
///         <tr><td>amber-500</td><td>rgb(245 158 11)</td><td style="background-color: rgb(245 158 11);"></td></tr>
///         <tr><td>amber-600</td><td>rgb(217 119 6)</td><td style="background-color: rgb(217 119 6);"></td></tr>
///         <tr><td>amber-700</td><td>rgb(180 83 9)</td><td style="background-color: rgb(180 83 9);"></td></tr>
///         <tr><td>amber-800</td><td>rgb(146 64 14)</td><td style="background-color: rgb(146 64 14);"></td></tr>
///         <tr><td>amber-900</td><td>rgb(120 53 15)</td><td style="background-color: rgb(120 53 15);"></td></tr>
///         <tr><td>yellow-50</td><td>rgb(254 252 232)</td><td style="background-color: rgb(254 252 232);"></td></tr>
///         <tr><td>yellow-100</td><td>rgb(254 249 195)</td><td style="background-color: rgb(254 249 195);"></td></tr>
///         <tr><td>yellow-200</td><td>rgb(254 240 138)</td><td style="background-color: rgb(254 240 138);"></td></tr>
///         <tr><td>yellow-300</td><td>rgb(253 224 71)</td><td style="background-color: rgb(253 224 71);"></td></tr>
///         <tr><td>yellow-400</td><td>rgb(250 204 21)</td><td style="background-color: rgb(250 204 21);"></td></tr>
///         <tr><td>yellow-500</td><td>rgb(234 179 8)</td><td style="background-color: rgb(234 179 8);"></td></tr>
///         <tr><td>yellow-600</td><td>rgb(202 138 4)</td><td style="background-color: rgb(202 138 4);"></td></tr>
///         <tr><td>yellow-700</td><td>rgb(161 98 7)</td><td style="background-color: rgb(161 98 7);"></td></tr>
///         <tr><td>yellow-800</td><td>rgb(133 77 14)</td><td style="background-color: rgb(133 77 14);"></td></tr>
///         <tr><td>yellow-900</td><td>rgb(113 63 18)</td><td style="background-color: rgb(113 63 18);"></td></tr>
///         <tr><td>lime-50</td><td>rgb(247 254 231)</td><td style="background-color: rgb(247 254 231);"></td></tr>
///         <tr><td>lime-100</td><td>rgb(236 252 203)</td><td style="background-color: rgb(236 252 203);"></td></tr>
///         <tr><td>lime-200</td><td>rgb(217 249 157)</td><td style="background-color: rgb(217 249 157);"></td></tr>
///         <tr><td>lime-300</td><td>rgb(190 242 100)</td><td style="background-color: rgb(190 242 100);"></td></tr>
///         <tr><td>lime-400</td><td>rgb(163 230 53)</td><td style="background-color: rgb(163 230 53);"></td></tr>
///         <tr><td>lime-500</td><td>rgb(132 204 22)</td><td style="background-color: rgb(132 204 22);"></td></tr>
///         <tr><td>lime-600</td><td>rgb(101 163 13)</td><td style="background-color: rgb(101 163 13);"></td></tr>
///         <tr><td>lime-700</td><td>rgb(77 124 15)</td><td style="background-color: rgb(77 124 15);"></td></tr>
///         <tr><td>lime-800</td><td>rgb(63 98 18)</td><td style="background-color: rgb(63 98 18);"></td></tr>
///         <tr><td>lime-900</td><td>rgb(54 83 20)</td><td style="background-color: rgb(54 83 20);"></td></tr>
///         <tr><td>green-50</td><td>rgb(240 253 244)</td><td style="background-color: rgb(240 253 244);"></td></tr>
///         <tr><td>green-100</td><td>rgb(220 252 231)</td><td style="background-color: rgb(220 252 231);"></td></tr>
///         <tr><td>green-200</td><td>rgb(187 247 208)</td><td style="background-color: rgb(187 247 208);"></td></tr>
///         <tr><td>green-300</td><td>rgb(134 239 172)</td><td style="background-color: rgb(134 239 172);"></td></tr>
///         <tr><td>green-400</td><td>rgb(74 222 128)</td><td style="background-color: rgb(74 222 128);"></td></tr>
///         <tr><td>green-500</td><td>rgb(34 197 94)</td><td style="background-color: rgb(34 197 94);"></td></tr>
///         <tr><td>green-600</td><td>rgb(22 163 74)</td><td style="background-color: rgb(22 163 74);"></td></tr>
///         <tr><td>green-700</td><td>rgb(21 128 61)</td><td style="background-color: rgb(21 128 61);"></td></tr>
///         <tr><td>green-800</td><td>rgb(22 101 52)</td><td style="background-color: rgb(22 101 52);"></td></tr>
///         <tr><td>green-900</td><td>rgb(20 83 45)</td><td style="background-color: rgb(20 83 45);"></td></tr>
///         <tr><td>emerald-50</td><td>rgb(236 253 245)</td><td style="background-color: rgb(236 253 245);"></td></tr>
///         <tr><td>emerald-100</td><td>rgb(209 250 229)</td><td style="background-color: rgb(209 250 229);"></td></tr>
///         <tr><td>emerald-200</td><td>rgb(167 243 208)</td><td style="background-color: rgb(167 243 208);"></td></tr>
///         <tr><td>emerald-300</td><td>rgb(110 231 183)</td><td style="background-color: rgb(110 231 183);"></td></tr>
///         <tr><td>emerald-400</td><td>rgb(52 211 153)</td><td style="background-color: rgb(52 211 153);"></td></tr>
///         <tr><td>emerald-500</td><td>rgb(16 185 129)</td><td style="background-color: rgb(16 185 129);"></td></tr>
///         <tr><td>emerald-600</td><td>rgb(5 150 105)</td><td style="background-color: rgb(5 150 105);"></td></tr>
///         <tr><td>emerald-700</td><td>rgb(4 120 87)</td><td style="background-color: rgb(4 120 87);"></td></tr>
///         <tr><td>emerald-800</td><td>rgb(6 95 70)</td><td style="background-color: rgb(6 95 70);"></td></tr>
///         <tr><td>emerald-900</td><td>rgb(6 78 59)</td><td style="background-color: rgb(6 78 59);"></td></tr>
///         <tr><td>teal-50</td><td>rgb(240 253 250)</td><td style="background-color: rgb(240 253 250);"></td></tr>
///         <tr><td>teal-100</td><td>rgb(204 251 241)</td><td style="background-color: rgb(204 251 241);"></td></tr>
///         <tr><td>teal-200</td><td>rgb(153 246 228)</td><td style="background-color: rgb(153 246 228);"></td></tr>
///         <tr><td>teal-300</td><td>rgb(94 234 212)</td><td style="background-color: rgb(94 234 212);"></td></tr>
///         <tr><td>teal-400</td><td>rgb(45 212 191)</td><td style="background-color: rgb(45 212 191);"></td></tr>
///         <tr><td>teal-500</td><td>rgb(20 184 166)</td><td style="background-color: rgb(20 184 166);"></td></tr>
///         <tr><td>teal-600</td><td>rgb(13 148 136)</td><td style="background-color: rgb(13 148 136);"></td></tr>
///         <tr><td>teal-700</td><td>rgb(15 118 110)</td><td style="background-color: rgb(15 118 110);"></td></tr>
///         <tr><td>teal-800</td><td>rgb(17 94 89)</td><td style="background-color: rgb(17 94 89);"></td></tr>
///         <tr><td>teal-900</td><td>rgb(19 78 74)</td><td style="background-color: rgb(19 78 74);"></td></tr>
///         <tr><td>cyan-50</td><td>rgb(236 254 255)</td><td style="background-color: rgb(236 254 255);"></td></tr>
///         <tr><td>cyan-100</td><td>rgb(207 250 254)</td><td style="background-color: rgb(207 250 254);"></td></tr>
///         <tr><td>cyan-200</td><td>rgb(165 243 252)</td><td style="background-color: rgb(165 243 252);"></td></tr>
///         <tr><td>cyan-300</td><td>rgb(103 232 249)</td><td style="background-color: rgb(103 232 249);"></td></tr>
///         <tr><td>cyan-400</td><td>rgb(34 211 238)</td><td style="background-color: rgb(34 211 238);"></td></tr>
///         <tr><td>cyan-500</td><td>rgb(6 182 212)</td><td style="background-color: rgb(6 182 212);"></td></tr>
///         <tr><td>cyan-600</td><td>rgb(8 145 178)</td><td style="background-color: rgb(8 145 178);"></td></tr>
///         <tr><td>cyan-700</td><td>rgb(14 116 144)</td><td style="background-color: rgb(14 116 144);"></td></tr>
///         <tr><td>cyan-800</td><td>rgb(21 94 117)</td><td style="background-color: rgb(21 94 117);"></td></tr>
///         <tr><td>cyan-900</td><td>rgb(22 78 99)</td><td style="background-color: rgb(22 78 99);"></td></tr>
///         <tr><td>sky-50</td><td>rgb(240 249 255)</td><td style="background-color: rgb(240 249 255);"></td></tr>
///         <tr><td>sky-100</td><td>rgb(224 242 254)</td><td style="background-color: rgb(224 242 254);"></td></tr>
///         <tr><td>sky-200</td><td>rgb(186 230 253)</td><td style="background-color: rgb(186 230 253);"></td></tr>
///         <tr><td>sky-300</td><td>rgb(125 211 252)</td><td style="background-color: rgb(125 211 252);"></td></tr>
///         <tr><td>sky-400</td><td>rgb(56 189 248)</td><td style="background-color: rgb(56 189 248);"></td></tr>
///         <tr><td>sky-500</td><td>rgb(14 165 233)</td><td style="background-color: rgb(14 165 233);"></td></tr>
///         <tr><td>sky-600</td><td>rgb(2 132 199)</td><td style="background-color: rgb(2 132 199);"></td></tr>
///         <tr><td>sky-700</td><td>rgb(3 105 161)</td><td style="background-color: rgb(3 105 161);"></td></tr>
///         <tr><td>sky-800</td><td>rgb(7 89 133)</td><td style="background-color: rgb(7 89 133);"></td></tr>
///         <tr><td>sky-900</td><td>rgb(12 74 110)</td><td style="background-color: rgb(12 74 110);"></td></tr>
///         <tr><td>blue-50</td><td>rgb(239 246 255)</td><td style="background-color: rgb(239 246 255);"></td></tr>
///         <tr><td>blue-100</td><td>rgb(219 234 254)</td><td style="background-color: rgb(219 234 254);"></td></tr>
///         <tr><td>blue-200</td><td>rgb(191 219 254)</td><td style="background-color: rgb(191 219 254);"></td></tr>
///         <tr><td>blue-300</td><td>rgb(147 197 253)</td><td style="background-color: rgb(147 197 253);"></td></tr>
///         <tr><td>blue-400</td><td>rgb(96 165 250)</td><td style="background-color: rgb(96 165 250);"></td></tr>
///         <tr><td>blue-500</td><td>rgb(59 130 246)</td><td style="background-color: rgb(59 130 246);"></td></tr>
///         <tr><td>blue-600</td><td>rgb(37 99 235)</td><td style="background-color: rgb(37 99 235);"></td></tr>
///         <tr><td>blue-700</td><td>rgb(29 78 216)</td><td style="background-color: rgb(29 78 216);"></td></tr>
///         <tr><td>blue-800</td><td>rgb(30 64 175)</td><td style="background-color: rgb(30 64 175);"></td></tr>
///         <tr><td>blue-900</td><td>rgb(30 58 138)</td><td style="background-color: rgb(30 58 138);"></td></tr>
///         <tr><td>indigo-50</td><td>rgb(238 242 255)</td><td style="background-color: rgb(238 242 255);"></td></tr>
///         <tr><td>indigo-100</td><td>rgb(224 231 255)</td><td style="background-color: rgb(224 231 255);"></td></tr>
///         <tr><td>indigo-200</td><td>rgb(199 210 254)</td><td style="background-color: rgb(199 210 254);"></td></tr>
///         <tr><td>indigo-300</td><td>rgb(165 180 252)</td><td style="background-color: rgb(165 180 252);"></td></tr>
///         <tr><td>indigo-400</td><td>rgb(129 140 248)</td><td style="background-color: rgb(129 140 248);"></td></tr>
///         <tr><td>indigo-500</td><td>rgb(99 102 241)</td><td style="background-color: rgb(99 102 241);"></td></tr>
///         <tr><td>indigo-600</td><td>rgb(79 70 229)</td><td style="background-color: rgb(79 70 229);"></td></tr>
///         <tr><td>indigo-700</td><td>rgb(67 56 202)</td><td style="background-color: rgb(67 56 202);"></td></tr>
///         <tr><td>indigo-800</td><td>rgb(55 48 163)</td><td style="background-color: rgb(55 48 163);"></td></tr>
///         <tr><td>indigo-900</td><td>rgb(49 46 129)</td><td style="background-color: rgb(49 46 129);"></td></tr>
///         <tr><td>violet-50</td><td>rgb(245 243 255)</td><td style="background-color: rgb(245 243 255);"></td></tr>
///         <tr><td>violet-100</td><td>rgb(237 233 254)</td><td style="background-color: rgb(237 233 254);"></td></tr>
///         <tr><td>violet-200</td><td>rgb(221 214 254)</td><td style="background-color: rgb(221 214 254);"></td></tr>
///         <tr><td>violet-300</td><td>rgb(196 181 253)</td><td style="background-color: rgb(196 181 253);"></td></tr>
///         <tr><td>violet-400</td><td>rgb(167 139 250)</td><td style="background-color: rgb(167 139 250);"></td></tr>
///         <tr><td>violet-500</td><td>rgb(139 92 246)</td><td style="background-color: rgb(139 92 246);"></td></tr>
///         <tr><td>violet-600</td><td>rgb(124 58 237)</td><td style="background-color: rgb(124 58 237);"></td></tr>
///         <tr><td>violet-700</td><td>rgb(109 40 217)</td><td style="background-color: rgb(109 40 217);"></td></tr>
///         <tr><td>violet-800</td><td>rgb(91 33 182)</td><td style="background-color: rgb(91 33 182);"></td></tr>
///         <tr><td>violet-900</td><td>rgb(76 29 149)</td><td style="background-color: rgb(76 29 149);"></td></tr>
///         <tr><td>purple-50</td><td>rgb(250 245 255)</td><td style="background-color: rgb(250 245 255);"></td></tr>
///         <tr><td>purple-100</td><td>rgb(243 232 255)</td><td style="background-color: rgb(243 232 255);"></td></tr>
///         <tr><td>purple-200</td><td>rgb(233 213 255)</td><td style="background-color: rgb(233 213 255);"></td></tr>
///         <tr><td>purple-300</td><td>rgb(216 180 254)</td><td style="background-color: rgb(216 180 254);"></td></tr>
///         <tr><td>purple-400</td><td>rgb(192 132 252)</td><td style="background-color: rgb(192 132 252);"></td></tr>
///         <tr><td>purple-500</td><td>rgb(168 85 247)</td><td style="background-color: rgb(168 85 247);"></td></tr>
///         <tr><td>purple-600</td><td>rgb(147 51 234)</td><td style="background-color: rgb(147 51 234);"></td></tr>
///         <tr><td>purple-700</td><td>rgb(126 34 206)</td><td style="background-color: rgb(126 34 206);"></td></tr>
///         <tr><td>purple-800</td><td>rgb(107 33 168)</td><td style="background-color: rgb(107 33 168);"></td></tr>
///         <tr><td>purple-900</td><td>rgb(88 28 135)</td><td style="background-color: rgb(88 28 135);"></td></tr>
///         <tr><td>fuchsia-50</td><td>rgb(253 244 255)</td><td style="background-color: rgb(253 244 255);"></td></tr>
///         <tr><td>fuchsia-100</td><td>rgb(250 232 255)</td><td style="background-color: rgb(250 232 255);"></td></tr>
///         <tr><td>fuchsia-200</td><td>rgb(245 208 254)</td><td style="background-color: rgb(245 208 254);"></td></tr>
///         <tr><td>fuchsia-300</td><td>rgb(240 171 252)</td><td style="background-color: rgb(240 171 252);"></td></tr>
///         <tr><td>fuchsia-400</td><td>rgb(232 121 249)</td><td style="background-color: rgb(232 121 249);"></td></tr>
///         <tr><td>fuchsia-500</td><td>rgb(217 70 239)</td><td style="background-color: rgb(217 70 239);"></td></tr>
///         <tr><td>fuchsia-600</td><td>rgb(192 38 211)</td><td style="background-color: rgb(192 38 211);"></td></tr>
///         <tr><td>fuchsia-700</td><td>rgb(162 28 175)</td><td style="background-color: rgb(162 28 175);"></td></tr>
///         <tr><td>fuchsia-800</td><td>rgb(134 25 143)</td><td style="background-color: rgb(134 25 143);"></td></tr>
///         <tr><td>fuchsia-900</td><td>rgb(112 26 117)</td><td style="background-color: rgb(112 26 117);"></td></tr>
///         <tr><td>pink-50</td><td>rgb(253 242 248)</td><td style="background-color: rgb(253 242 248);"></td></tr>
///         <tr><td>pink-100</td><td>rgb(252 231 243)</td><td style="background-color: rgb(252 231 243);"></td></tr>
///         <tr><td>pink-200</td><td>rgb(251 207 232)</td><td style="background-color: rgb(251 207 232);"></td></tr>
///         <tr><td>pink-300</td><td>rgb(249 168 212)</td><td style="background-color: rgb(249 168 212);"></td></tr>
///         <tr><td>pink-400</td><td>rgb(244 114 182)</td><td style="background-color: rgb(244 114 182);"></td></tr>
///         <tr><td>pink-500</td><td>rgb(236 72 153)</td><td style="background-color: rgb(236 72 153);"></td></tr>
///         <tr><td>pink-600</td><td>rgb(219 39 119)</td><td style="background-color: rgb(219 39 119);"></td></tr>
///         <tr><td>pink-700</td><td>rgb(190 24 93)</td><td style="background-color: rgb(190 24 93);"></td></tr>
///         <tr><td>pink-800</td><td>rgb(157 23 77)</td><td style="background-color: rgb(157 23 77);"></td></tr>
///         <tr><td>pink-900</td><td>rgb(131 24 67)</td><td style="background-color: rgb(131 24 67);"></td></tr>
///         <tr><td>rose-50</td><td>rgb(255 241 242)</td><td style="background-color: rgb(255 241 242);"></td></tr>
///         <tr><td>rose-100</td><td>rgb(255 228 230)</td><td style="background-color: rgb(255 228 230);"></td></tr>
///         <tr><td>rose-200</td><td>rgb(254 205 211)</td><td style="background-color: rgb(254 205 211);"></td></tr>
///         <tr><td>rose-300</td><td>rgb(253 164 175)</td><td style="background-color: rgb(253 164 175);"></td></tr>
///         <tr><td>rose-400</td><td>rgb(251 113 133)</td><td style="background-color: rgb(251 113 133);"></td></tr>
///         <tr><td>rose-500</td><td>rgb(244 63 94)</td><td style="background-color: rgb(244 63 94);"></td></tr>
///         <tr><td>rose-600</td><td>rgb(225 29 72)</td><td style="background-color: rgb(225 29 72);"></td></tr>
///         <tr><td>rose-700</td><td>rgb(190 18 60)</td><td style="background-color: rgb(190 18 60);"></td></tr>
///         <tr><td>rose-800</td><td>rgb(159 18 57)</td><td style="background-color: rgb(159 18 57);"></td></tr>
///         <tr><td>rose-900</td><td>rgb(136 19 55)</td><td style="background-color: rgb(136 19 55);"></td></tr>
///     </tbody>
/// </table>
///
/// Based on [Tailwind's default color palette](https://tailwindcss.com/docs/customizing-colors).
pub const BUILTIN_COLORS: &[(&str, (u8, u8, u8))] = &[
    ("slate-50", (248, 250, 252)),
    ("slate-100", (241, 245, 249)),
    ("slate-200", (226, 232, 240)),
    ("slate-300", (203, 213, 225)),
    ("slate-400", (148, 163, 184)),
    ("slate-500", (100, 116, 139)),
    ("slate-600", (71, 85, 105)),
    ("slate-700", (51, 65, 85)),
    ("slate-800", (30, 41, 59)),
    ("slate-900", (15, 23, 42)),
    ("gray-50", (249, 250, 251)),
    ("gray-100", (243, 244, 246)),
    ("gray-200", (229, 231, 235)),
    ("gray-300", (209, 213, 219)),
    ("gray-400", (156, 163, 175)),
    ("gray-500", (107, 114, 128)),
    ("gray-600", (75, 85, 99)),
    ("gray-700", (55, 65, 81)),
    ("gray-800", (31, 41, 55)),
    ("gray-900", (17, 24, 39)),
    ("zinc-50", (250, 250, 250)),
    ("zinc-100", (244, 244, 245)),
    ("zinc-200", (228, 228, 231)),
    ("zinc-300", (212, 212, 216)),
    ("zinc-400", (161, 161, 170)),
    ("zinc-500", (113, 113, 122)),
    ("zinc-600", (82, 82, 91)),
    ("zinc-700", (63, 63, 70)),
    ("zinc-800", (39, 39, 42)),
    ("zinc-900", (24, 24, 27)),
    ("neutral-50", (250, 250, 250)),
    ("neutral-100", (245, 245, 245)),
    ("neutral-200", (229, 229, 229)),
    ("neutral-300", (212, 212, 212)),
    ("neutral-400", (163, 163, 163)),
    ("neutral-500", (115, 115, 115)),
    ("neutral-600", (82, 82, 82)),
    ("neutral-700", (64, 64, 64)),
    ("neutral-800", (38, 38, 38)),
    ("neutral-900", (23, 23, 23)),
    ("stone-50", (250, 250, 249)),
    ("stone-100", (245, 245, 244)),
    ("stone-200", (231, 229, 228)),
    ("stone-300", (214, 211, 209)),
    ("stone-400", (168, 162, 158)),
    ("stone-500", (120, 113, 108)),
    ("stone-600", (87, 83, 78)),
    ("stone-700", (68, 64, 60)),
    ("stone-800", (41, 37, 36)),
    ("stone-900", (28, 25, 23)),
    ("red-50", (254, 242, 242)),
    ("red-100", (254, 226, 226)),
    ("red-200", (254, 202, 202)),
    ("red-300", (252, 165, 165)),
    ("red-400", (248, 113, 113)),
    ("red-500", (239, 68, 68)),
    ("red-600", (220, 38, 38)),
    ("red-700", (185, 28, 28)),
    ("red-800", (153, 27, 27)),
    ("red-900", (127, 29, 29)),
    ("orange-50", (255, 247, 237)),
    ("orange-100", (255, 237, 213)),
    ("orange-200", (254, 215, 170)),
    ("orange-300", (253, 186, 116)),
    ("orange-400", (251, 146, 60)),
    ("orange-500", (249, 115, 22)),
    ("orange-600", (234, 88, 12)),
    ("orange-700", (194, 65, 12)),
    ("orange-800", (154, 52, 18)),
    ("orange-900", (124, 45, 18)),
    ("amber-50", (255, 251, 235)),
    ("amber-100", (254, 243, 199)),
    ("amber-200", (253, 230, 138)),
    ("amber-300", (252, 211, 77)),
    ("amber-400", (251, 191, 36)),
    ("amber-500", (245, 158, 11)),
    ("amber-600", (217, 119, 6)),
    ("amber-700", (180, 83, 9)),
    ("amber-800", (146, 64, 14)),
    ("amber-900", (120, 53, 15)),
    ("yellow-50", (254, 252, 232)),
    ("yellow-100", (254, 249, 195)),
    ("yellow-200", (254, 240, 138)),
    ("yellow-300", (253, 224, 71)),
    ("yellow-400", (250, 204, 21)),
    ("yellow-500", (234, 179, 8)),
    ("yellow-600", (202, 138, 4)),
    ("yellow-700", (161, 98, 7)),
    ("yellow-800", (133, 77, 14)),
    ("yellow-900", (113, 63, 18)),
    ("lime-50", (247, 254, 231)),
    ("lime-100", (236, 252, 203)),
    ("lime-200", (217, 249, 157)),
    ("lime-300", (190, 242, 100)),
    ("lime-400", (163, 230, 53)),
    ("lime-500", (132, 204, 22)),
    ("lime-600", (101, 163, 13)),
    ("lime-700", (77, 124, 15)),
    ("lime-800", (63, 98, 18)),
    ("lime-900", (54, 83, 20)),
    ("green-50", (240, 253, 244)),
    ("green-100", (220, 252, 231)),
    ("green-200", (187, 247, 208)),
    ("green-300", (134, 239, 172)),
    ("green-400", (74, 222, 128)),
    ("green-500", (34, 197, 94)),
    ("green-600", (22, 163, 74)),
    ("green-700", (21, 128, 61)),
    ("green-800", (22, 101, 52)),
    ("green-900", (20, 83, 45)),
    ("emerald-50", (236, 253, 245)),
    ("emerald-100", (209, 250, 229)),
    ("emerald-200", (167, 243, 208)),
    ("emerald-300", (110, 231, 183)),
    ("emerald-400", (52, 211, 153)),
    ("emerald-500", (16, 185, 129)),
    ("emerald-600", (5, 150, 105)),
    ("emerald-700", (4, 120, 87)),
    ("emerald-800", (6, 95, 70)),
    ("emerald-900", (6, 78, 59)),
    ("teal-50", (240, 253, 250)),
    ("teal-100", (204, 251, 241)),
    ("teal-200", (153, 246, 228)),
    ("teal-300", (94, 234, 212)),
    ("teal-400", (45, 212, 191)),
    ("teal-500", (20, 184, 166)),
    ("teal-600", (13, 148, 136)),
    ("teal-700", (15, 118, 110)),
    ("teal-800", (17, 94, 89)),
    ("teal-900", (19, 78, 74)),
    ("cyan-50", (236, 254, 255)),
    ("cyan-100", (207, 250, 254)),
    ("cyan-200", (165, 243, 252)),
    ("cyan-300", (103, 232, 249)),
    ("cyan-400", (34, 211, 238)),
    ("cyan-500", (6, 182, 212)),
    ("cyan-600", (8, 145, 178)),
    ("cyan-700", (14, 116, 144)),
    ("cyan-800", (21, 94, 117)),
    ("cyan-900", (22, 78, 99)),
    ("sky-50", (240, 249, 255)),
    ("sky-100", (224, 242, 254)),
    ("sky-200", (186, 230, 253)),
    ("sky-300", (125, 211, 252)),
    ("sky-400", (56, 189, 248)),
    ("sky-500", (14, 165, 233)),
    ("sky-600", (2, 132, 199)),
    ("sky-700", (3, 105, 161)),
    ("sky-800", (7, 89, 133)),
    ("sky-900", (12, 74, 110)),
    ("blue-50", (239, 246, 255)),
    ("blue-100", (219, 234, 254)),
    ("blue-200", (191, 219, 254)),
    ("blue-300", (147, 197, 253)),
    ("blue-400", (96, 165, 250)),
    ("blue-500", (59, 130, 246)),
    ("blue-600", (37, 99, 235)),
    ("blue-700", (29, 78, 216)),
    ("blue-800", (30, 64, 175)),
    ("blue-900", (30, 58, 138)),
    ("indigo-50", (238, 242, 255)),
    ("indigo-100", (224, 231, 255)),
    ("indigo-200", (199, 210, 254)),
    ("indigo-300", (165, 180, 252)),
    ("indigo-400", (129, 140, 248)),
    ("indigo-500", (99, 102, 241)),
    ("indigo-600", (79, 70, 229)),
    ("indigo-700", (67, 56, 202)),
    ("indigo-800", (55, 48, 163)),
    ("indigo-900", (49, 46, 129)),
    ("violet-50", (245, 243, 255)),
    ("violet-100", (237, 233, 254)),
    ("violet-200", (221, 214, 254)),
    ("violet-300", (196, 181, 253)),
    ("violet-400", (167, 139, 250)),
    ("violet-500", (139, 92, 246)),
    ("violet-600", (124, 58, 237)),
    ("violet-700", (109, 40, 217)),
    ("violet-800", (91, 33, 182)),
    ("violet-900", (76, 29, 149)),
    ("purple-50", (250, 245, 255)),
    ("purple-100", (243, 232, 255)),
    ("purple-200", (233, 213, 255)),
    ("purple-300", (216, 180, 254)),
    ("purple-400", (192, 132, 252)),
    ("purple-500", (168, 85, 247)),
    ("purple-600", (147, 51, 234)),
    ("purple-700", (126, 34, 206)),
    ("purple-800", (107, 33, 168)),
    ("purple-900", (88, 28, 135)),
    ("fuchsia-50", (253, 244, 255)),
    ("fuchsia-100", (250, 232, 255)),
    ("fuchsia-200", (245, 208, 254)),
    ("fuchsia-300", (240, 171, 252)),
    ("fuchsia-400", (232, 121, 249)),
    ("fuchsia-500", (217, 70, 239)),
    ("fuchsia-600", (192, 38, 211)),
    ("fuchsia-700", (162, 28, 175)),
    ("fuchsia-800", (134, 25, 143)),
    ("fuchsia-900", (112, 26, 117)),
    ("pink-50", (253, 242, 248)),
    ("pink-100", (252, 231, 243)),
    ("pink-200", (251, 207, 232)),
    ("pink-300", (249, 168, 212)),
    ("pink-400", (244, 114, 182)),
    ("pink-500", (236, 72, 153)),
    ("pink-600", (219, 39, 119)),
    ("pink-700", (190, 24, 93)),
    ("pink-800", (157, 23, 77)),
    ("pink-900", (131, 24, 67)),
    ("rose-50", (255, 241, 242)),
    ("rose-100", (255, 228, 230)),
    ("rose-200", (254, 205, 211)),
    ("rose-300", (253, 164, 175)),
    ("rose-400", (251, 113, 133)),
    ("rose-500", (244, 63, 94)),
    ("rose-600", (225, 29, 72)),
    ("rose-700", (190, 18, 60)),
    ("rose-800", (159, 18, 57)),
    ("rose-900", (136, 19, 55)),
];

/// The list of all default screen breakpoints.
///
/// Based on [Tailwind's default screen breakpoints](https://tailwindcss.com/docs/screens).
pub const BUILTIN_SCREENS: &[(&str, &str)] = &[
    ("sm", "640px"),
    ("md", "768px"),
    ("lg", "1024px"),
    ("xl", "1280px"),
    ("2xl", "1536px"),
];

/// The list of all default variants.
///
/// Based on [Tailwind's default variants](https://tailwindcss.com/docs/hover-focus-and-other-states).
#[rustfmt::skip]
pub const BUILTIN_VARIANTS: &[(Cow<'static, str>, VariantType)] = &[
    // --- Pseudo element ---
    (Cow::Borrowed("first-letter"), VariantType::PseudoElement("first-letter")),
    (Cow::Borrowed("first-line"), VariantType::PseudoElement("first-line")),
    (Cow::Borrowed("marker"), VariantType::WrapClass(Cow::Borrowed("& *::marker, &::marker"))),
    (Cow::Borrowed("selection"), VariantType::WrapClass(Cow::Borrowed("& *::selection, &::selection"))),
    (Cow::Borrowed("file"), VariantType::WrapClass(Cow::Borrowed("&::file-selector-button, &::-webkit-file-upload-button"))),
    (Cow::Borrowed("placeholder"), VariantType::PseudoElement("placeholder")),
    (Cow::Borrowed("backdrop"), VariantType::PseudoElement("backdrop")),
    (Cow::Borrowed("before"), VariantType::PseudoElement("before")),
    (Cow::Borrowed("after"), VariantType::PseudoElement("after")),
    (Cow::Borrowed("all"), VariantType::WrapClass(Cow::Borrowed("& *"))),
    (Cow::Borrowed("children"), VariantType::WrapClass(Cow::Borrowed("& > *"))),
    (Cow::Borrowed("siblings"), VariantType::WrapClass(Cow::Borrowed("& ~ *"))),
    (Cow::Borrowed("sibling"), VariantType::WrapClass(Cow::Borrowed("& + *"))),

    // --- Pseudo class ---
    (Cow::Borrowed("first"), VariantType::PseudoClass("first-child")),
    (Cow::Borrowed("not-first"), VariantType::PseudoClass("not(:first-child)")),
    (Cow::Borrowed("last"), VariantType::PseudoClass("last-child")),
    (Cow::Borrowed("not-last"), VariantType::PseudoClass("not(:last-child)")),
    (Cow::Borrowed("only"), VariantType::PseudoClass("only-child")),
    (Cow::Borrowed("not-only"), VariantType::PseudoClass("not(:only-child)")),
    (Cow::Borrowed("odd"), VariantType::PseudoClass("nth-child(odd)")),
    (Cow::Borrowed("even"), VariantType::PseudoClass("nth-child(even)")),
    (Cow::Borrowed("first-of-type"), VariantType::PseudoClass("first-of-type")),
    (Cow::Borrowed("last-of-type"), VariantType::PseudoClass("last-of-type")),
    (Cow::Borrowed("only-of-type"), VariantType::PseudoClass("only-of-type")),
    (Cow::Borrowed("not-first-of-type"), VariantType::PseudoClass("not(:first-of-type)")),
    (Cow::Borrowed("not-last-of-type"), VariantType::PseudoClass("not(:last-of-type)")),
    (Cow::Borrowed("not-only-of-type"), VariantType::PseudoClass("not(:only-of-type)")),
    (Cow::Borrowed("visited"), VariantType::PseudoClass("visited")),
    (Cow::Borrowed("target"), VariantType::PseudoClass("target")),
    (Cow::Borrowed("open"), VariantType::WrapClass(Cow::Borrowed("&[open]"))),
    (Cow::Borrowed("default"), VariantType::PseudoClass("default")),
    (Cow::Borrowed("checked"), VariantType::PseudoClass("checked")),
    (Cow::Borrowed("not-checked"), VariantType::PseudoClass("not(:checked)")),
    (Cow::Borrowed("indeterminate"), VariantType::PseudoClass("indeterminate")),
    (Cow::Borrowed("placeholder-shown"), VariantType::PseudoClass("placeholder-shown")),
    (Cow::Borrowed("autofill"), VariantType::PseudoClass("autofill")),
    (Cow::Borrowed("optional"), VariantType::PseudoClass("optional")),
    (Cow::Borrowed("required"), VariantType::PseudoClass("required")),
    (Cow::Borrowed("valid"), VariantType::PseudoClass("valid")),
    (Cow::Borrowed("invalid"), VariantType::PseudoClass("invalid")),
    (Cow::Borrowed("in-range"), VariantType::PseudoClass("in-range")),
    (Cow::Borrowed("out-of-range"), VariantType::PseudoClass("out-of-range")),
    (Cow::Borrowed("read-only"), VariantType::PseudoClass("read-only")),
    (Cow::Borrowed("read-write"), VariantType::PseudoClass("read-write")),
    (Cow::Borrowed("empty"), VariantType::PseudoClass("empty")),
    (Cow::Borrowed("focus-within"), VariantType::PseudoClass("focus-within")),
    (Cow::Borrowed("hover"), VariantType::PseudoClass("hover")),
    (Cow::Borrowed("focus"), VariantType::PseudoClass("focus")),
    (Cow::Borrowed("focus-visible"), VariantType::PseudoClass("focus-visible")),
    (Cow::Borrowed("active"), VariantType::PseudoClass("active")),
    (Cow::Borrowed("enabled"), VariantType::PseudoClass("enabled")),
    (Cow::Borrowed("disabled"), VariantType::PseudoClass("disabled")),
    (Cow::Borrowed("ltr"), VariantType::WrapClass(Cow::Borrowed("[dir=\"ltr\"] &"))),
    (Cow::Borrowed("rtl"), VariantType::WrapClass(Cow::Borrowed("[dir=\"rtl\"] &"))),

    // --- At rules ---
    (Cow::Borrowed("motion-safe"), VariantType::AtRule(Cow::Borrowed("@media (prefers-reduced-motion: no-preference)"))),
    (Cow::Borrowed("motion-reduce"), VariantType::AtRule(Cow::Borrowed("@media (prefers-reduced-motion: reduce)"))),
    (Cow::Borrowed("print"), VariantType::AtRule(Cow::Borrowed("@media print"))),
    (Cow::Borrowed("portrait"), VariantType::AtRule(Cow::Borrowed("@media (orientation: portrait)"))),
    (Cow::Borrowed("landscape"), VariantType::AtRule(Cow::Borrowed("@media (orientation: landscape)"))),
    (Cow::Borrowed("contrast-more"), VariantType::AtRule(Cow::Borrowed("@media (prefers-contrast: more)"))),
    (Cow::Borrowed("contrast-less"), VariantType::AtRule(Cow::Borrowed("@media (prefers-contrast: less)"))),
];

/// The list of all default plugins.
///
/// Sorted following [Tailwind's order](https://github.com/tailwindlabs/tailwindcss/blob/master/src/corePlugins.js).
pub const BUILTIN_PLUGINS: [&'static (dyn Plugin + Send + Sync); 227] = [
    &layout::container::PluginDefinition,
    &accessibility::screen_reader::PluginDefinition,
    &interactivity::pointer_events::PluginDefinition,
    &layout::visibility::PluginDefinition,
    &layout::position::PluginDefinition,
    &layout::placement::PluginInsetDefinition,
    &layout::placement::PluginInsetXDefinition,
    &layout::placement::PluginInsetYDefinition,
    &layout::placement::PluginTopDefinition,
    &layout::placement::PluginRightDefinition,
    &layout::placement::PluginBottomDefinition,
    &layout::placement::PluginLeftDefinition,
    &layout::isolation::PluginDefinition,
    &layout::z_index::PluginDefinition,
    &flexbox::order::PluginDefinition,
    &grid::grid_column::PluginDefinition,
    &grid::grid_row::PluginDefinition,
    &layout::floats::PluginDefinition,
    &layout::clear::PluginDefinition,
    &spacing::margin::PluginDefinition,
    &spacing::margin::PluginXDefinition,
    &spacing::margin::PluginYDefinition,
    &spacing::margin::PluginTopDefinition,
    &spacing::margin::PluginRightDefinition,
    &spacing::margin::PluginBottomDefinition,
    &spacing::margin::PluginLeftDefinition,
    &layout::box_sizing::PluginDefinition,
    &layout::display::PluginDefinition,
    &layout::aspect_ratio::PluginDefinition,
    &sizing::height::PluginDefinition,
    &sizing::max_height::PluginDefinition,
    &sizing::min_height::PluginDefinition,
    &sizing::width::PluginDefinition,
    &sizing::min_width::PluginDefinition,
    &sizing::max_width::PluginDefinition,
    &flexbox::flex::PluginDefinition,
    &flexbox::flex_shrink::PluginDefinition,
    &flexbox::flex_grow::PluginDefinition,
    &flexbox::flex_basis::PluginDefinition,
    &table::table_layout::PluginDefinition,
    &table::border_collapse::PluginDefinition,
    &table::border_spacing::PluginDefinition,
    &table::border_spacing::PluginDefinition,
    &table::border_spacing::PluginDefinition,
    &transform::transform_origin::PluginDefinition,
    &transform::translate::PluginXDefinition,
    &transform::translate::PluginYDefinition,
    &transform::rotate::PluginDefinition,
    &transform::skew::PluginXDefinition,
    &transform::skew::PluginYDefinition,
    &transform::scale::PluginDefinition,
    &transform::scale::PluginXDefinition,
    &transform::scale::PluginYDefinition,
    &transform::transform_type::PluginDefinition,
    &transition::animation::PluginDefinition,
    &interactivity::cursor::PluginDefinition,
    &interactivity::touch_action::PluginDefinition,
    &interactivity::user_select::PluginDefinition,
    &interactivity::resize::PluginDefinition,
    &interactivity::scroll_snap_type::PluginDefinition,
    &interactivity::scroll_snap_align::PluginDefinition,
    &interactivity::scroll_snap_stop::PluginDefinition,
    &interactivity::scroll_margin::PluginDefinition,
    &interactivity::scroll_margin::PluginXDefinition,
    &interactivity::scroll_margin::PluginYDefinition,
    &interactivity::scroll_margin::PluginTopDefinition,
    &interactivity::scroll_margin::PluginRightDefinition,
    &interactivity::scroll_margin::PluginBottomDefinition,
    &interactivity::scroll_margin::PluginLeftDefinition,
    &interactivity::scroll_padding::PluginDefinition,
    &interactivity::scroll_padding::PluginXDefinition,
    &interactivity::scroll_padding::PluginYDefinition,
    &interactivity::scroll_padding::PluginTopDefinition,
    &interactivity::scroll_padding::PluginRightDefinition,
    &interactivity::scroll_padding::PluginBottomDefinition,
    &interactivity::scroll_padding::PluginLeftDefinition,
    &typography::list_style_position::PluginDefinition,
    &typography::list_style_type::PluginDefinition,
    &interactivity::appearance::PluginDefinition,
    &layout::columns::PluginDefinition,
    &layout::break_before::PluginDefinition,
    &layout::break_inside::PluginDefinition,
    &layout::break_after::PluginDefinition,
    &grid::grid_auto_columns::PluginDefinition,
    &grid::grid_auto_flow::PluginDefinition,
    &grid::grid_auto_rows::PluginDefinition,
    &grid::grid_template_columns::PluginDefinition,
    &grid::grid_template_rows::PluginDefinition,
    &flexbox::flex_direction::PluginDefinition,
    &flexbox::flex_wrap::PluginDefinition,
    &flexbox::place_content::PluginDefinition,
    &flexbox::place_items::PluginDefinition,
    &flexbox::align_content::PluginDefinition,
    &flexbox::align_items::PluginDefinition,
    &flexbox::justify_content::PluginDefinition,
    &flexbox::justify_items::PluginDefinition,
    &grid::gap::PluginDefinition,
    &grid::gap::PluginXDefinition,
    &grid::gap::PluginYDefinition,
    &spacing::space_between::PluginXDefinition,
    &spacing::space_between::PluginYDefinition,
    &border::divide_width::PluginXDefinition,
    &border::divide_width::PluginYDefinition,
    &border::divide_style::PluginDefinition,
    &border::divide_color::PluginDefinition,
    &border::divide_opacity::PluginDefinition,
    &flexbox::place_self::PluginDefinition,
    &flexbox::align_self::PluginDefinition,
    &flexbox::justify_self::PluginDefinition,
    &layout::overflow::PluginDefinition,
    &layout::overscroll_behavior::PluginDefinition,
    &interactivity::scroll_behavior::PluginDefinition,
    &typography::text_overflow::PluginDefinition,
    &typography::whitespace::PluginDefinition,
    &typography::word_break::PluginDefinition,
    &border::border_radius::PluginDefinition,
    &border::border_radius::PluginTopDefinition,
    &border::border_radius::PluginRightDefinition,
    &border::border_radius::PluginBottomDefinition,
    &border::border_radius::PluginLeftDefinition,
    &border::border_radius::PluginTopRightDefinition,
    &border::border_radius::PluginTopLeftDefinition,
    &border::border_radius::PluginBottomRightDefinition,
    &border::border_radius::PluginBottomLeftDefinition,
    &border::border_width::PluginDefinition,
    &border::border_width::PluginXDefinition,
    &border::border_width::PluginYDefinition,
    &border::border_width::PluginTopDefinition,
    &border::border_width::PluginRightDefinition,
    &border::border_width::PluginBottomDefinition,
    &border::border_width::PluginLeftDefinition,
    &border::border_style::PluginDefinition,
    &border::border_color::PluginDefinition,
    &border::border_color::PluginXDefinition,
    &border::border_color::PluginYDefinition,
    &border::border_color::PluginTopDefinition,
    &border::border_color::PluginRightDefinition,
    &border::border_color::PluginBottomDefinition,
    &border::border_color::PluginLeftDefinition,
    &border::border_opacity::PluginDefinition,
    &background::background_color::PluginDefinition,
    &background::background_opacity::PluginDefinition,
    &background::background_image::PluginDefinition,
    &background::gradient_color_stops::PluginFromDefinition,
    &background::gradient_color_stops::PluginViaDefinition,
    &background::gradient_color_stops::PluginToDefinition,
    &layout::box_decoration_break::PluginDefinition,
    &background::background_size::PluginDefinition,
    &background::background_attachment::PluginDefinition,
    &background::background_clip::PluginDefinition,
    &background::background_position::PluginDefinition,
    &background::background_repeat::PluginDefinition,
    &background::background_origin::PluginDefinition,
    &svg::fill::PluginDefinition,
    &svg::stroke::PluginDefinition,
    &svg::stroke_width::PluginDefinition,
    &layout::object_fit::PluginDefinition,
    &layout::object_position::PluginDefinition,
    &spacing::padding::PluginDefinition,
    &spacing::padding::PluginXDefinition,
    &spacing::padding::PluginYDefinition,
    &spacing::padding::PluginTopDefinition,
    &spacing::padding::PluginRightDefinition,
    &spacing::padding::PluginBottomDefinition,
    &spacing::padding::PluginLeftDefinition,
    &typography::text_align::PluginDefinition,
    &typography::text_indent::PluginDefinition,
    &typography::vertical_align::PluginDefinition,
    &typography::font_family::PluginDefinition,
    &typography::font_size::PluginDefinition,
    &typography::font_weight::PluginDefinition,
    &typography::text_transform::PluginDefinition,
    &typography::font_style::PluginDefinition,
    &typography::font_variant_numeric::PluginDefinition,
    &typography::letter_spacing::PluginDefinition,
    &typography::line_height::PluginDefinition,
    &typography::text_color::PluginDefinition,
    &typography::text_opacity::PluginDefinition,
    &typography::text_decoration::PluginDefinition,
    &typography::text_decoration_color::PluginDefinition,
    &typography::text_decoration_style::PluginDefinition,
    &typography::text_decoration_thickness::PluginDefinition,
    &typography::text_underline_offset::PluginDefinition,
    &typography::font_smoothing::PluginDefinition,
    &interactivity::caret_color::PluginDefinition,
    &interactivity::accent_color::PluginDefinition,
    &effect::opacity::PluginDefinition,
    &effect::background_blend_mode::PluginDefinition,
    &effect::mix_blend_mode::PluginDefinition,
    &effect::box_shadow::PluginDefinition,
    &effect::box_shadow_color::PluginDefinition,
    &border::outline_style::PluginDefinition,
    &border::outline_width::PluginDefinition,
    &border::outline_offset::PluginDefinition,
    &border::outline_color::PluginDefinition,
    &border::ring_width::PluginDefinition,
    &border::ring_color::PluginDefinition,
    &border::ring_opacity::PluginDefinition,
    &border::ring_offset_width::PluginDefinition,
    &border::ring_offset_color::PluginDefinition,
    &filter::blur::PluginDefinition,
    &filter::brightness::PluginDefinition,
    &filter::contrast::PluginDefinition,
    &filter::drop_shadow::PluginDefinition,
    &filter::grayscale::PluginDefinition,
    &filter::hue_rotate::PluginDefinition,
    &filter::invert::PluginDefinition,
    &filter::saturate::PluginDefinition,
    &filter::sepia::PluginDefinition,
    &filter::filter_type::PluginDefinition,
    &filter::backdrop_blur::PluginDefinition,
    &filter::backdrop_brightness::PluginDefinition,
    &filter::backdrop_contrast::PluginDefinition,
    &filter::backdrop_grayscale::PluginDefinition,
    &filter::backdrop_hue_rotate::PluginDefinition,
    &filter::backdrop_invert::PluginDefinition,
    &filter::backdrop_opacity::PluginDefinition,
    &filter::backdrop_saturate::PluginDefinition,
    &filter::backdrop_sepia::PluginDefinition,
    &filter::backdrop_filter::PluginDefinition,
    &transition::transition_property::PluginDefinition,
    &transition::transition_delay::PluginDefinition,
    &transition::transition_duration::PluginDefinition,
    &transition::transition_timing_function::PluginDefinition,
    &interactivity::will_change::PluginDefinition,
    &typography::content::PluginDefinition,
    &typography::line_clamp::PluginDefinition,
];

/// Configuration for the [`Theme::dark_mode`] field.
///
/// It defines how the `dark:` variant should behaves.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DarkMode {
    /// The `dark:` variant will modify the class of the selector. You'll then need to toggle this
    /// class to enable the dark theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use encre_css::{EncreGenerator, Config, config::DarkMode};
    ///
    /// let mut config = Config::default();
    /// config.theme.dark_mode = DarkMode::new_class("body.dark");
    ///
    /// let mut generator = EncreGenerator::from_config(config);
    /// generator.add_selector("dark:text-white");
    ///
    /// assert!(generator.generate().ends_with(r#"body.dark .dark\:text-white {
    ///   --en-text-opacity: 1;
    ///   color: rgb(255 255 255 / var(--en-text-opacity));
    /// }"#));
    /// ```
    Class(Cow<'static, str>),

    /// The `dark:` variant will generates a `@media (prefers-color-scheme: dark)` rule to enable
    /// the dark theme following user preference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use encre_css::{EncreGenerator, Config, config::DarkMode};
    ///
    /// let mut config = Config::default();
    /// config.theme.dark_mode = DarkMode::Media;
    ///
    /// let mut generator = EncreGenerator::from_config(config);
    /// generator.add_selector("dark:text-white");
    ///
    /// assert!(generator.generate().ends_with(r#"@media (prefers-color-scheme: dark) {
    ///   .dark\:text-white {
    ///     --en-text-opacity: 1;
    ///     color: rgb(255 255 255 / var(--en-text-opacity));
    ///   }
    /// }"#));
    /// ```
    Media,
}

impl Default for DarkMode {
    fn default() -> Self {
        Self::Media
    }
}

impl DarkMode {
    /// Quickly build a [`DarkMode::Class`] value.
    pub fn new_class<T: Into<Cow<'static, str>>>(class: T) -> Self {
        Self::Class(class.into())
    }
}

/// Configuration for the [`Theme::screens`] field.
///
/// It defines a list of custom screen breakpoints.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Screens(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Screens {
    /// Add a custom screen breakpoint to the list.
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<Cow<'static, str>>>(
        &mut self,
        key: T1,
        val: T2,
    ) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove a custom screen breakpoint from the list.
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Cow<'static, str>, &Cow<'static, str>)> {
        self.0.iter()
    }
}

/// Configuration for the [`Theme::colors`] field.
///
/// It defines a list of custom colors.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Colors(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Colors {
    /// Add a custom color to the list.
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<Cow<'static, str>>>(
        &mut self,
        key: T1,
        val: T2,
    ) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove a custom color from the list.
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    pub(crate) fn get<'a, T: Into<Cow<'a, str>>>(&self, key: T) -> Option<&Cow<'a, str>> {
        self.0.get(&key.into())
    }

    pub(crate) fn contains<'a, T: Into<Cow<'a, str>>>(&self, key: T) -> bool {
        self.0.contains_key(&key.into())
    }
}

/// Configuration for the [`Config::theme`] field.
///
/// It defines some design system specific values like custom colors or screen breakpoints.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Theme {
    /// Dark mode configuration.
    ///
    /// The default value is [`DarkMode::Media`].
    #[serde(default)]
    pub dark_mode: DarkMode,

    /// Custom screen breakpoints configuration.
    ///
    /// The default value is an empty map.
    #[serde(default)]
    pub screens: Screens,

    /// Custom colors configuration.
    ///
    /// The default value is an empty map.
    #[serde(default)]
    pub colors: Colors,
}

/// The configuration of the [`EncreGenerator`] structure.
///
/// It can either be manually created or deserialized from a
/// [TOML](https://toml.io) file using [`Config::from_file`].
///
/// Based on [Tailwind's configuration](https://tailwindcss.com/docs/configuration).
///
/// [`EncreGenerator`]: crate::EncreGenerator
#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    /// Theme configuration.
    #[serde(default)]
    pub theme: Theme,

    /// Preflight configuration.
    #[serde(default)]
    pub preflight: Preflight,

    /// A custom scanner used to scan content.
    ///
    /// This field is skipped when deserializing from a [TOML](https://toml.io) file.
    #[serde(skip)]
    pub scanner: Scanner,

    /// A list of custom plugins.
    ///
    /// This field is skipped when deserializing from a [TOML](https://toml.io) file.
    #[serde(skip)]
    pub(crate) custom_plugins: Vec<&'static (dyn Plugin + Send + Sync)>,

    /// A list of custom variants.
    ///
    /// This field is skipped when deserializing from a [TOML](https://toml.io) file.
    #[serde(skip)]
    custom_variants: Mutex<Vec<(Cow<'static, str>, VariantType)>>,
    // TODO: Prefix (en-), safelist, separator for {variants, arbitrary values}
}

impl Config {
    pub(crate) fn get_custom_variants(&self) -> MutexGuard<Vec<(Cow<'static, str>, VariantType)>> {
        // Initialize the list of custom variants if not already initialized
        let mut custom_variants = self
            .custom_variants
            .try_lock()
            .expect("failed to lock the list of custom variants");

        if !custom_variants.iter().any(|v| v.0 == Cow::Borrowed("dark")) {
            custom_variants.extend(
                self.theme
                    .screens
                    .iter()
                    .map(|screen| {
                        (
                            screen.0.clone(),
                            VariantType::AtRule(Cow::Owned(format!(
                                "@media (min-width: {})",
                                screen.1
                            ))),
                        )
                    })
                    .chain(BUILTIN_SCREENS.iter().map(|screen| {
                        (
                            Cow::from(screen.0),
                            VariantType::AtRule(Cow::Owned(format!(
                                "@media (min-width: {})",
                                screen.1
                            ))),
                        )
                    }))
                    .chain(iter::once(match &self.theme.dark_mode {
                        DarkMode::Media => (
                            Cow::from("dark"),
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)")),
                        ),
                        DarkMode::Class(name) => (
                            Cow::from("dark"),
                            VariantType::WrapClass(name.clone() + " &"),
                        ),
                    })),
            );
        }

        custom_variants
    }

    /// Register a custom plugin which will be used during CSS generation.
    ///
    /// Note that if you are not the maintainer of a crate providing plugins, you can ignore this
    /// function, see [`crate::plugins`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use encre_css::{Config, EncreGenerator, prelude::build_plugin::*};
    ///
    /// #[derive(Debug)]
    /// struct Prose;
    ///
    /// impl Plugin for Prose {
    ///     fn namespace(&self) -> &'static str {
    ///         "prose"
    ///     }
    ///
    ///     fn can_handle(&self, context: ContextCanHandle) -> bool {
    ///         match context.modifier {
    ///             Modifier::Builtin { value, .. } => *value == "" || *value == "invert",
    ///             Modifier::Arbitrary { .. } => false,
    ///         }
    ///     }
    ///
    ///     fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
    ///         match modifier {
    ///             Modifier::Builtin { value, .. } => match *value {
    ///                 "" => writeln!(buffer, "{indentation}color: #333;"),
    ///                 "invert" => writeln!(buffer, "{indentation}color: #eee;"),
    ///                 _ => unreachable!(),
    ///             },
    ///             Modifier::Arbitrary { .. } => unreachable!(),
    ///         }
    ///     }
    /// }
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&Prose);
    ///
    /// let mut generator = EncreGenerator::from_config(config);
    /// generator.add_selector("prose");
    /// generator.add_selector("prose-invert");
    ///
    /// assert!(generator.generate().ends_with(".prose {
    ///   color: #333;
    /// }
    ///
    /// .prose-invert {
    ///   color: #eee;
    /// }"));
    /// ```
    pub fn register_plugin(&mut self, plugin: &'static (dyn Plugin + Send + Sync)) {
        self.custom_plugins.push(plugin);
    }

    /// Register a custom variant which will be used during CSS generation.
    ///
    /// Note that if you are not the maintainer of a crate providing variants, you can ignore this
    /// function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use encre_css::{Config, EncreGenerator, selector::VariantType};
    /// use std::borrow::Cow;
    ///
    /// let mut config = Config::default();
    /// config.register_variant("headings", VariantType::WrapClass(Cow::Borrowed("& :where(h1, h2, h3, h4, h5, h6)")));
    ///
    /// let mut generator = EncreGenerator::from_config(config);
    /// generator.add_selector("headings:text-gray-700");
    ///
    /// assert!(generator.generate().ends_with(".headings\\:text-gray-700 :where(h1, h2, h3, h4, h5, h6) {
    ///   --en-text-opacity: 1;
    ///   color: rgb(55 65 81 / var(--en-text-opacity));
    /// }"));
    /// ```
    pub fn register_variant<T: Into<Cow<'static, str>>>(
        &mut self,
        variant_name: T,
        variant_type: VariantType,
    ) {
        self.custom_variants
            .try_lock()
            .expect("failed to lock the list of custom variants")
            .push((variant_name.into(), variant_type));
    }

    /// Deserialize the content of a [TOML](https://toml.io) file to get the configuration.
    ///
    /// # Example
    ///
    /// In a `encre-css.toml` file:
    ///
    /// <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[theme]</span>
    /// dark_mode = { class = <span class="string">".dark"</span> }
    /// screens = { 3xl = <span class="string">"1600px"</span>, lg = <span class="string">"2000px"</span> }<br>
    /// <span class="kw">[theme.colors]</span>
    /// primary = <span class="string">"#e5186a"</span>
    /// yellow-400 = <span class="string">"#ffef0e"</span></code></pre></div>
    ///
    /// Then parse the configuration in Rust:
    ///
    /// ```rust,ignore
    /// use encre_css::Config;
    /// let config = Config::from_file("encre-css.toml").expect("failed to parse the configuration file");
    /// ```
    ///
    /// Note that if you don't change the config after parsing it, you can just use
    /// [`EncreGenerator::new`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::ConfigFileNotFound`] if the given file does not exist.
    ///
    /// [`EncreGenerator::new`]: crate::EncreGenerator::new
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(&path).map_err(
            |e| Error::ConfigFileNotFound(path.as_ref().to_path_buf(), e),
        )?)?)
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.theme.eq(&other.theme)
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Config")
            .field("theme", &self.theme)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EncreGenerator;

    use pretty_assertions::assert_eq;

    fn base_config() -> Config {
        // Disable the preflight to simplify test assertions
        Config {
            preflight: Preflight::None,
            ..Default::default()
        }
    }

    #[test]
    fn gen_css_with_custom_config() {
        let mut config = base_config();
        config.theme.colors.add("rosa-500", "#e5186a");
        config.theme.screens.add("3xl", "1600px");

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("3xl:text-rosa-500");

        assert_eq!(
            generator.generate(),
            String::from(
                r#"@media (min-width: 1600px) {
  .\33xl\:text-rosa-500 {
    --en-text-opacity: 1;
    color: rgb(229 24 106 / var(--en-text-opacity));
  }
}"#
            )
        );
    }

    #[test]
    fn parse_config_file() {
        let mut config = base_config();
        config.theme.colors.add("rosa-500", "#e5186a");
        config.theme.colors.add("yellow-400", "#ffef0e");
        config.theme.screens.add("lg", "2000px");
        config.theme.screens.add("3xl", "1600px");
        config.theme.dark_mode = DarkMode::new_class(".dark");

        assert_eq!(
            Config::from_file("tests/fixtures/custom_config.toml").unwrap(),
            config
        );
    }

    #[test]
    fn config_is_extended_and_overridden() {
        let config = Config::from_file("tests/fixtures/custom_config.toml").unwrap();

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("bg-rosa-500");
        generator.add_selector("bg-yellow-400");
        generator.add_selector("bg-yellow-100");
        generator.add_selector("3xl:underline");
        generator.add_selector("lg:text-rosa-500");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".bg-rosa-500 {
  --en-bg-opacity: 1;
  background-color: rgb(229 24 106 / var(--en-bg-opacity));
}

.bg-yellow-100 {
  --en-bg-opacity: 1;
  background-color: rgb(254 249 195 / var(--en-bg-opacity));
}

.bg-yellow-400 {
  --en-bg-opacity: 1;
  background-color: rgb(255 239 14 / var(--en-bg-opacity));
}

@media (min-width: 1600px) {
  .\33xl\:underline {
    -webkit-text-decoration-line: underline;
    text-decoration-line: underline;
  }
}

@media (min-width: 2000px) {
  .lg\:text-rosa-500 {
    --en-text-opacity: 1;
    color: rgb(229 24 106 / var(--en-text-opacity));
  }
}"#
            )
        );
    }
}
