use crate::error::{Error, Result};

use serde::Deserialize;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt, fs,
    ops::{Deref, DerefMut},
    path::Path,
};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DarkModeConfig {
    Class(Cow<'static, str>),
    Media,
}

impl Default for DarkModeConfig {
    fn default() -> Self {
        Self::Media
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ScreenConfig(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Deref for ScreenConfig {
    type Target = BTreeMap<Cow<'static, str>, Cow<'static, str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ScreenConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BTreeMap<Cow<'static, str>, Cow<'static, str>>> for ScreenConfig {
    fn from(v: BTreeMap<Cow<'static, str>, Cow<'static, str>>) -> Self {
        Self(v)
    }
}

impl Default for ScreenConfig {
    fn default() -> Self {
        let mut screens = BTreeMap::new();
        screens.insert(Cow::from("sm"), Cow::from("640px"));
        screens.insert(Cow::from("md"), Cow::from("768px"));
        screens.insert(Cow::from("lg"), Cow::from("1024px"));
        screens.insert(Cow::from("xl"), Cow::from("1280px"));
        screens.insert(Cow::from("2xl"), Cow::from("1536px"));

        Self(screens)
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ColorConfig(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Deref for ColorConfig {
    type Target = BTreeMap<Cow<'static, str>, Cow<'static, str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ColorConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        let mut colors = BTreeMap::new();
        colors.insert(Cow::from("slate-50"), Cow::from("#f8fafc"));
        colors.insert(Cow::from("slate-100"), Cow::from("#f1f5f9"));
        colors.insert(Cow::from("slate-200"), Cow::from("#e2e8f0"));
        colors.insert(Cow::from("slate-300"), Cow::from("#cbd5e1"));
        colors.insert(Cow::from("slate-400"), Cow::from("#94a3b8"));
        colors.insert(Cow::from("slate-500"), Cow::from("#64748b"));
        colors.insert(Cow::from("slate-600"), Cow::from("#475569"));
        colors.insert(Cow::from("slate-700"), Cow::from("#334155"));
        colors.insert(Cow::from("slate-800"), Cow::from("#1e293b"));
        colors.insert(Cow::from("slate-900"), Cow::from("#0f172a"));
        colors.insert(Cow::from("gray-50"), Cow::from("#f9fafb"));
        colors.insert(Cow::from("gray-100"), Cow::from("#f3f4f6"));
        colors.insert(Cow::from("gray-200"), Cow::from("#e5e7eb"));
        colors.insert(Cow::from("gray-300"), Cow::from("#d1d5db"));
        colors.insert(Cow::from("gray-400"), Cow::from("#9ca3af"));
        colors.insert(Cow::from("gray-500"), Cow::from("#6b7280"));
        colors.insert(Cow::from("gray-600"), Cow::from("#4b5563"));
        colors.insert(Cow::from("gray-700"), Cow::from("#374151"));
        colors.insert(Cow::from("gray-800"), Cow::from("#1f2937"));
        colors.insert(Cow::from("gray-900"), Cow::from("#111827"));
        colors.insert(Cow::from("zinc-50"), Cow::from("#fafafa"));
        colors.insert(Cow::from("zinc-100"), Cow::from("#f4f4f5"));
        colors.insert(Cow::from("zinc-200"), Cow::from("#e4e4e7"));
        colors.insert(Cow::from("zinc-300"), Cow::from("#d4d4d8"));
        colors.insert(Cow::from("zinc-400"), Cow::from("#a1a1aa"));
        colors.insert(Cow::from("zinc-500"), Cow::from("#71717a"));
        colors.insert(Cow::from("zinc-600"), Cow::from("#52525b"));
        colors.insert(Cow::from("zinc-700"), Cow::from("#3f3f46"));
        colors.insert(Cow::from("zinc-800"), Cow::from("#27272a"));
        colors.insert(Cow::from("zinc-900"), Cow::from("#18181b"));
        colors.insert(Cow::from("neutral-50"), Cow::from("#fafafa"));
        colors.insert(Cow::from("neutral-100"), Cow::from("#f5f5f5"));
        colors.insert(Cow::from("neutral-200"), Cow::from("#e5e5e5"));
        colors.insert(Cow::from("neutral-300"), Cow::from("#d4d4d4"));
        colors.insert(Cow::from("neutral-400"), Cow::from("#a3a3a3"));
        colors.insert(Cow::from("neutral-500"), Cow::from("#737373"));
        colors.insert(Cow::from("neutral-600"), Cow::from("#525252"));
        colors.insert(Cow::from("neutral-700"), Cow::from("#404040"));
        colors.insert(Cow::from("neutral-800"), Cow::from("#262626"));
        colors.insert(Cow::from("neutral-900"), Cow::from("#171717"));
        colors.insert(Cow::from("stone-50"), Cow::from("#fafaf9"));
        colors.insert(Cow::from("stone-100"), Cow::from("#f5f5f4"));
        colors.insert(Cow::from("stone-200"), Cow::from("#e7e5e4"));
        colors.insert(Cow::from("stone-300"), Cow::from("#d6d3d1"));
        colors.insert(Cow::from("stone-400"), Cow::from("#a8a29e"));
        colors.insert(Cow::from("stone-500"), Cow::from("#78716c"));
        colors.insert(Cow::from("stone-600"), Cow::from("#57534e"));
        colors.insert(Cow::from("stone-700"), Cow::from("#44403c"));
        colors.insert(Cow::from("stone-800"), Cow::from("#292524"));
        colors.insert(Cow::from("stone-900"), Cow::from("#1c1917"));
        colors.insert(Cow::from("red-50"), Cow::from("#fef2f2"));
        colors.insert(Cow::from("red-100"), Cow::from("#fee2e2"));
        colors.insert(Cow::from("red-200"), Cow::from("#fecaca"));
        colors.insert(Cow::from("red-300"), Cow::from("#fca5a5"));
        colors.insert(Cow::from("red-400"), Cow::from("#f87171"));
        colors.insert(Cow::from("red-500"), Cow::from("#ef4444"));
        colors.insert(Cow::from("red-600"), Cow::from("#dc2626"));
        colors.insert(Cow::from("red-700"), Cow::from("#b91c1c"));
        colors.insert(Cow::from("red-800"), Cow::from("#991b1b"));
        colors.insert(Cow::from("red-900"), Cow::from("#7f1d1d"));
        colors.insert(Cow::from("orange-50"), Cow::from("#fff7ed"));
        colors.insert(Cow::from("orange-100"), Cow::from("#ffedd5"));
        colors.insert(Cow::from("orange-200"), Cow::from("#fed7aa"));
        colors.insert(Cow::from("orange-300"), Cow::from("#fdba74"));
        colors.insert(Cow::from("orange-400"), Cow::from("#fb923c"));
        colors.insert(Cow::from("orange-500"), Cow::from("#f97316"));
        colors.insert(Cow::from("orange-600"), Cow::from("#ea580c"));
        colors.insert(Cow::from("orange-700"), Cow::from("#c2410c"));
        colors.insert(Cow::from("orange-800"), Cow::from("#9a3412"));
        colors.insert(Cow::from("orange-900"), Cow::from("#7c2d12"));
        colors.insert(Cow::from("amber-50"), Cow::from("#fffbeb"));
        colors.insert(Cow::from("amber-100"), Cow::from("#fef3c7"));
        colors.insert(Cow::from("amber-200"), Cow::from("#fde68a"));
        colors.insert(Cow::from("amber-300"), Cow::from("#fcd34d"));
        colors.insert(Cow::from("amber-400"), Cow::from("#fbbf24"));
        colors.insert(Cow::from("amber-500"), Cow::from("#f59e0b"));
        colors.insert(Cow::from("amber-600"), Cow::from("#d97706"));
        colors.insert(Cow::from("amber-700"), Cow::from("#b45309"));
        colors.insert(Cow::from("amber-800"), Cow::from("#92400e"));
        colors.insert(Cow::from("amber-900"), Cow::from("#78350f"));
        colors.insert(Cow::from("yellow-50"), Cow::from("#fefce8"));
        colors.insert(Cow::from("yellow-100"), Cow::from("#fef9c3"));
        colors.insert(Cow::from("yellow-200"), Cow::from("#fef08a"));
        colors.insert(Cow::from("yellow-300"), Cow::from("#fde047"));
        colors.insert(Cow::from("yellow-400"), Cow::from("#facc15"));
        colors.insert(Cow::from("yellow-500"), Cow::from("#eab308"));
        colors.insert(Cow::from("yellow-600"), Cow::from("#ca8a04"));
        colors.insert(Cow::from("yellow-700"), Cow::from("#a16207"));
        colors.insert(Cow::from("yellow-800"), Cow::from("#854d0e"));
        colors.insert(Cow::from("yellow-900"), Cow::from("#713f12"));
        colors.insert(Cow::from("lime-50"), Cow::from("#f7fee7"));
        colors.insert(Cow::from("lime-100"), Cow::from("#ecfccb"));
        colors.insert(Cow::from("lime-200"), Cow::from("#d9f99d"));
        colors.insert(Cow::from("lime-300"), Cow::from("#bef264"));
        colors.insert(Cow::from("lime-400"), Cow::from("#a3e635"));
        colors.insert(Cow::from("lime-500"), Cow::from("#84cc16"));
        colors.insert(Cow::from("lime-600"), Cow::from("#65a30d"));
        colors.insert(Cow::from("lime-700"), Cow::from("#4d7c0f"));
        colors.insert(Cow::from("lime-800"), Cow::from("#3f6212"));
        colors.insert(Cow::from("lime-900"), Cow::from("#365314"));
        colors.insert(Cow::from("green-50"), Cow::from("#f0fdf4"));
        colors.insert(Cow::from("green-100"), Cow::from("#dcfce7"));
        colors.insert(Cow::from("green-200"), Cow::from("#bbf7d0"));
        colors.insert(Cow::from("green-300"), Cow::from("#86efac"));
        colors.insert(Cow::from("green-400"), Cow::from("#4ade80"));
        colors.insert(Cow::from("green-500"), Cow::from("#22c55e"));
        colors.insert(Cow::from("green-600"), Cow::from("#16a34a"));
        colors.insert(Cow::from("green-700"), Cow::from("#15803d"));
        colors.insert(Cow::from("green-800"), Cow::from("#166534"));
        colors.insert(Cow::from("green-900"), Cow::from("#14532d"));
        colors.insert(Cow::from("emerald-50"), Cow::from("#ecfdf5"));
        colors.insert(Cow::from("emerald-100"), Cow::from("#d1fae5"));
        colors.insert(Cow::from("emerald-200"), Cow::from("#a7f3d0"));
        colors.insert(Cow::from("emerald-300"), Cow::from("#6ee7b7"));
        colors.insert(Cow::from("emerald-400"), Cow::from("#34d399"));
        colors.insert(Cow::from("emerald-500"), Cow::from("#10b981"));
        colors.insert(Cow::from("emerald-600"), Cow::from("#059669"));
        colors.insert(Cow::from("emerald-700"), Cow::from("#047857"));
        colors.insert(Cow::from("emerald-800"), Cow::from("#065f46"));
        colors.insert(Cow::from("emerald-900"), Cow::from("#064e3b"));
        colors.insert(Cow::from("teal-50"), Cow::from("#f0fdfa"));
        colors.insert(Cow::from("teal-100"), Cow::from("#ccfbf1"));
        colors.insert(Cow::from("teal-200"), Cow::from("#99f6e4"));
        colors.insert(Cow::from("teal-300"), Cow::from("#5eead4"));
        colors.insert(Cow::from("teal-400"), Cow::from("#2dd4bf"));
        colors.insert(Cow::from("teal-500"), Cow::from("#14b8a6"));
        colors.insert(Cow::from("teal-600"), Cow::from("#0d9488"));
        colors.insert(Cow::from("teal-700"), Cow::from("#0f766e"));
        colors.insert(Cow::from("teal-800"), Cow::from("#115e59"));
        colors.insert(Cow::from("teal-900"), Cow::from("#134e4a"));
        colors.insert(Cow::from("cyan-50"), Cow::from("#ecfeff"));
        colors.insert(Cow::from("cyan-100"), Cow::from("#cffafe"));
        colors.insert(Cow::from("cyan-200"), Cow::from("#a5f3fc"));
        colors.insert(Cow::from("cyan-300"), Cow::from("#67e8f9"));
        colors.insert(Cow::from("cyan-400"), Cow::from("#22d3ee"));
        colors.insert(Cow::from("cyan-500"), Cow::from("#06b6d4"));
        colors.insert(Cow::from("cyan-600"), Cow::from("#0891b2"));
        colors.insert(Cow::from("cyan-700"), Cow::from("#0e7490"));
        colors.insert(Cow::from("cyan-800"), Cow::from("#155e75"));
        colors.insert(Cow::from("cyan-900"), Cow::from("#164e63"));
        colors.insert(Cow::from("sky-50"), Cow::from("#f0f9ff"));
        colors.insert(Cow::from("sky-100"), Cow::from("#e0f2fe"));
        colors.insert(Cow::from("sky-200"), Cow::from("#bae6fd"));
        colors.insert(Cow::from("sky-300"), Cow::from("#7dd3fc"));
        colors.insert(Cow::from("sky-400"), Cow::from("#38bdf8"));
        colors.insert(Cow::from("sky-500"), Cow::from("#0ea5e9"));
        colors.insert(Cow::from("sky-600"), Cow::from("#0284c7"));
        colors.insert(Cow::from("sky-700"), Cow::from("#0369a1"));
        colors.insert(Cow::from("sky-800"), Cow::from("#075985"));
        colors.insert(Cow::from("sky-900"), Cow::from("#0c4a6e"));
        colors.insert(Cow::from("blue-50"), Cow::from("#eff6ff"));
        colors.insert(Cow::from("blue-100"), Cow::from("#dbeafe"));
        colors.insert(Cow::from("blue-200"), Cow::from("#bfdbfe"));
        colors.insert(Cow::from("blue-300"), Cow::from("#93c5fd"));
        colors.insert(Cow::from("blue-400"), Cow::from("#60a5fa"));
        colors.insert(Cow::from("blue-500"), Cow::from("#3b82f6"));
        colors.insert(Cow::from("blue-600"), Cow::from("#2563eb"));
        colors.insert(Cow::from("blue-700"), Cow::from("#1d4ed8"));
        colors.insert(Cow::from("blue-800"), Cow::from("#1e40af"));
        colors.insert(Cow::from("blue-900"), Cow::from("#1e3a8a"));
        colors.insert(Cow::from("indigo-50"), Cow::from("#eef2ff"));
        colors.insert(Cow::from("indigo-100"), Cow::from("#e0e7ff"));
        colors.insert(Cow::from("indigo-200"), Cow::from("#c7d2fe"));
        colors.insert(Cow::from("indigo-300"), Cow::from("#a5b4fc"));
        colors.insert(Cow::from("indigo-400"), Cow::from("#818cf8"));
        colors.insert(Cow::from("indigo-500"), Cow::from("#6366f1"));
        colors.insert(Cow::from("indigo-600"), Cow::from("#4f46e5"));
        colors.insert(Cow::from("indigo-700"), Cow::from("#4338ca"));
        colors.insert(Cow::from("indigo-800"), Cow::from("#3730a3"));
        colors.insert(Cow::from("indigo-900"), Cow::from("#312e81"));
        colors.insert(Cow::from("violet-50"), Cow::from("#f5f3ff"));
        colors.insert(Cow::from("violet-100"), Cow::from("#ede9fe"));
        colors.insert(Cow::from("violet-200"), Cow::from("#ddd6fe"));
        colors.insert(Cow::from("violet-300"), Cow::from("#c4b5fd"));
        colors.insert(Cow::from("violet-400"), Cow::from("#a78bfa"));
        colors.insert(Cow::from("violet-500"), Cow::from("#8b5cf6"));
        colors.insert(Cow::from("violet-600"), Cow::from("#7c3aed"));
        colors.insert(Cow::from("violet-700"), Cow::from("#6d28d9"));
        colors.insert(Cow::from("violet-800"), Cow::from("#5b21b6"));
        colors.insert(Cow::from("violet-900"), Cow::from("#4c1d95"));
        colors.insert(Cow::from("purple-50"), Cow::from("#faf5ff"));
        colors.insert(Cow::from("purple-100"), Cow::from("#f3e8ff"));
        colors.insert(Cow::from("purple-200"), Cow::from("#e9d5ff"));
        colors.insert(Cow::from("purple-300"), Cow::from("#d8b4fe"));
        colors.insert(Cow::from("purple-400"), Cow::from("#c084fc"));
        colors.insert(Cow::from("purple-500"), Cow::from("#a855f7"));
        colors.insert(Cow::from("purple-600"), Cow::from("#9333ea"));
        colors.insert(Cow::from("purple-700"), Cow::from("#7e22ce"));
        colors.insert(Cow::from("purple-800"), Cow::from("#6b21a8"));
        colors.insert(Cow::from("purple-900"), Cow::from("#581c87"));
        colors.insert(Cow::from("fuchsia-50"), Cow::from("#fdf4ff"));
        colors.insert(Cow::from("fuchsia-100"), Cow::from("#fae8ff"));
        colors.insert(Cow::from("fuchsia-200"), Cow::from("#f5d0fe"));
        colors.insert(Cow::from("fuchsia-300"), Cow::from("#f0abfc"));
        colors.insert(Cow::from("fuchsia-400"), Cow::from("#e879f9"));
        colors.insert(Cow::from("fuchsia-500"), Cow::from("#d946ef"));
        colors.insert(Cow::from("fuchsia-600"), Cow::from("#c026d3"));
        colors.insert(Cow::from("fuchsia-700"), Cow::from("#a21caf"));
        colors.insert(Cow::from("fuchsia-800"), Cow::from("#86198f"));
        colors.insert(Cow::from("fuchsia-900"), Cow::from("#701a75"));
        colors.insert(Cow::from("pink-50"), Cow::from("#fdf2f8"));
        colors.insert(Cow::from("pink-100"), Cow::from("#fce7f3"));
        colors.insert(Cow::from("pink-200"), Cow::from("#fbcfe8"));
        colors.insert(Cow::from("pink-300"), Cow::from("#f9a8d4"));
        colors.insert(Cow::from("pink-400"), Cow::from("#f472b6"));
        colors.insert(Cow::from("pink-500"), Cow::from("#ec4899"));
        colors.insert(Cow::from("pink-600"), Cow::from("#db2777"));
        colors.insert(Cow::from("pink-700"), Cow::from("#be185d"));
        colors.insert(Cow::from("pink-800"), Cow::from("#9d174d"));
        colors.insert(Cow::from("pink-900"), Cow::from("#831843"));
        colors.insert(Cow::from("rose-50"), Cow::from("#fff1f2"));
        colors.insert(Cow::from("rose-100"), Cow::from("#ffe4e6"));
        colors.insert(Cow::from("rose-200"), Cow::from("#fecdd3"));
        colors.insert(Cow::from("rose-300"), Cow::from("#fda4af"));
        colors.insert(Cow::from("rose-400"), Cow::from("#fb7185"));
        colors.insert(Cow::from("rose-500"), Cow::from("#f43f5e"));
        colors.insert(Cow::from("rose-600"), Cow::from("#e11d48"));
        colors.insert(Cow::from("rose-700"), Cow::from("#be123c"));
        colors.insert(Cow::from("rose-800"), Cow::from("#9f1239"));
        colors.insert(Cow::from("rose-900"), Cow::from("#881337"));

        Self(colors)
    }
}

impl From<BTreeMap<Cow<'static, str>, Cow<'static, str>>> for ColorConfig {
    fn from(v: BTreeMap<Cow<'static, str>, Cow<'static, str>>) -> Self {
        Self(v)
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ModifierSeparator(Cow<'static, str>);

impl Default for ModifierSeparator {
    fn default() -> Self {
        Self(Cow::from("-"))
    }
}

impl fmt::Display for ModifierSeparator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Cow<'static, str>> for ModifierSeparator {
    fn from(v: Cow<'static, str>) -> Self {
        Self(v)
    }
}

impl Deref for ModifierSeparator {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct ThemeConfig {
    #[serde(default)]
    pub dark_mode: DarkModeConfig,

    #[serde(default)]
    pub screens: ScreenConfig,

    #[serde(default)]
    pub colors: ColorConfig,
}

#[derive(Default, PartialEq, Debug, Deserialize)]
pub struct Config {
    /// <span class="item-info">
    ///   <div class="stab portability">
    ///     Only available when the <strong>glob_scanning</strong> feature is enabled.
    ///   </div>
    /// </span>
    ///
    /// Specify which files should be scanned using globs.
    #[cfg(feature = "glob_scanning")]
    #[serde(default)]
    pub input: Vec<std::path::PathBuf>,

    #[serde(default)]
    pub theme: ThemeConfig,

    /// A custom modifier separator
    ///
    /// For example in `bg-red-500`, `-` is the modifier separator
    ///
    /// ### Be careful when changing the default modifier and defining new colors
    ///
    /// Custom configured colors must not take into account this separator, they are always
    /// delimited with `-` in the configuration but usable with this separator after. For example,
    /// if you have configured `_` as the modifier separator and you want to add the new color `lime-500`,
    /// you must write it using hyphens in the configuration, but you'll use it as `bg_lime_500`
    /// and `text_lime_500`
    #[serde(default)]
    pub modifier_separator: ModifierSeparator,
    // custom_variants: Vec<VariantConfig>,
    // custom_plugins: Vec<PluginConfig>,

    // TODO: Prefix (en-), preflight, safelist, separator for {variants, arbitrary values}
}

impl Config {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        let mut config: Config = toml::from_str(
            &fs::read_to_string(&path)
                .map_err(|e| Error::ConfigFileNotFound(path.as_ref().to_path_buf(), e))?,
        )?;

        if config.theme.colors != ColorConfig::default() {
            let overriden_colors = config.theme.colors.clone();
            config
                .theme
                .colors
                .extend(ColorConfig::default().iter().filter_map(|c| {
                    let key = c.0.clone();
                    if !overriden_colors.contains_key(&key) {
                        Some((key, c.1.clone()))
                    } else {
                        None
                    }
                }));
        }

        if config.theme.screens != ScreenConfig::default() {
            let overriden_screens = config.theme.screens.clone();
            config
                .theme
                .screens
                .extend(ScreenConfig::default().iter().filter_map(|c| {
                    let key = c.0.clone();
                    if !overriden_screens.contains_key(&key) {
                        Some((key, c.1.clone()))
                    } else {
                        None
                    }
                }));
        }

        Ok(config)
    }
}
