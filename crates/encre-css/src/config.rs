use crate::error::{Result, Error};

use serde::{Deserialize, Deserializer, de::{Visitor, MapAccess, value::MapAccessDeserializer}};
use serde_derive::Deserialize;
use std::{fs, fmt, ops::{Deref, DerefMut}, path::PathBuf, borrow::Cow, collections::BTreeMap};

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
pub struct ColorConfig(BTreeMap<Cow<'static, str>, [u8; 3]>);

impl Deref for ColorConfig {
    type Target = BTreeMap<Cow<'static, str>, [u8; 3]>;

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
        colors.insert(Cow::from("slate-50"), [248, 250, 252]);
        colors.insert(Cow::from("slate-100"), [241, 245, 249]);
        colors.insert(Cow::from("slate-200"), [226, 232, 240]);
        colors.insert(Cow::from("slate-300"), [203, 213, 225]);
        colors.insert(Cow::from("slate-400"), [148, 163, 184]);
        colors.insert(Cow::from("slate-500"), [100, 116, 139]);
        colors.insert(Cow::from("slate-600"), [71, 85, 105]);
        colors.insert(Cow::from("slate-700"), [51, 65, 85]);
        colors.insert(Cow::from("slate-800"), [30, 41, 59]);
        colors.insert(Cow::from("slate-900"), [15, 23, 42]);
        colors.insert(Cow::from("gray-50"), [249, 250, 251]);
        colors.insert(Cow::from("gray-100"), [243, 244, 246]);
        colors.insert(Cow::from("gray-200"), [229, 231, 235]);
        colors.insert(Cow::from("gray-300"), [209, 213, 219]);
        colors.insert(Cow::from("gray-400"), [156, 163, 175]);
        colors.insert(Cow::from("gray-500"), [107, 114, 128]);
        colors.insert(Cow::from("gray-600"), [75, 85, 99]);
        colors.insert(Cow::from("gray-700"), [55, 65, 81]);
        colors.insert(Cow::from("gray-800"), [31, 41, 55]);
        colors.insert(Cow::from("gray-900"), [17, 24, 39]);
        colors.insert(Cow::from("zinc-50"), [250, 250, 250]);
        colors.insert(Cow::from("zinc-100"), [244, 244, 245]);
        colors.insert(Cow::from("zinc-200"), [228, 228, 231]);
        colors.insert(Cow::from("zinc-300"), [212, 212, 216]);
        colors.insert(Cow::from("zinc-400"), [161, 161, 170]);
        colors.insert(Cow::from("zinc-500"), [113, 113, 122]);
        colors.insert(Cow::from("zinc-600"), [82, 82, 91]);
        colors.insert(Cow::from("zinc-700"), [63, 63, 70]);
        colors.insert(Cow::from("zinc-800"), [39, 39, 42]);
        colors.insert(Cow::from("zinc-900"), [24, 24, 27]);
        colors.insert(Cow::from("neutral-50"), [250, 250, 250]);
        colors.insert(Cow::from("neutral-100"), [245, 245, 245]);
        colors.insert(Cow::from("neutral-200"), [229, 229, 229]);
        colors.insert(Cow::from("neutral-300"), [212, 212, 212]);
        colors.insert(Cow::from("neutral-400"), [163, 163, 163]);
        colors.insert(Cow::from("neutral-500"), [115, 115, 115]);
        colors.insert(Cow::from("neutral-600"), [82, 82, 82]);
        colors.insert(Cow::from("neutral-700"), [64, 64, 64]);
        colors.insert(Cow::from("neutral-800"), [38, 38, 38]);
        colors.insert(Cow::from("neutral-900"), [23, 23, 23]);
        colors.insert(Cow::from("stone-50"), [250, 250, 249]);
        colors.insert(Cow::from("stone-100"), [245, 245, 244]);
        colors.insert(Cow::from("stone-200"), [231, 229, 228]);
        colors.insert(Cow::from("stone-300"), [214, 211, 209]);
        colors.insert(Cow::from("stone-400"), [168, 162, 158]);
        colors.insert(Cow::from("stone-500"), [120, 113, 108]);
        colors.insert(Cow::from("stone-600"), [87, 83, 78]);
        colors.insert(Cow::from("stone-700"), [68, 64, 60]);
        colors.insert(Cow::from("stone-800"), [41, 37, 36]);
        colors.insert(Cow::from("stone-900"), [28, 25, 23]);
        colors.insert(Cow::from("red-50"), [254, 242, 242]);
        colors.insert(Cow::from("red-100"), [254, 226, 226]);
        colors.insert(Cow::from("red-200"), [254, 202, 202]);
        colors.insert(Cow::from("red-300"), [252, 165, 165]);
        colors.insert(Cow::from("red-400"), [248, 113, 113]);
        colors.insert(Cow::from("red-500"), [239, 68, 68]);
        colors.insert(Cow::from("red-600"), [220, 38, 38]);
        colors.insert(Cow::from("red-700"), [185, 28, 28]);
        colors.insert(Cow::from("red-800"), [153, 27, 27]);
        colors.insert(Cow::from("red-900"), [127, 29, 29]);
        colors.insert(Cow::from("orange-50"), [255, 247, 237]);
        colors.insert(Cow::from("orange-100"), [255, 237, 213]);
        colors.insert(Cow::from("orange-200"), [254, 215, 170]);
        colors.insert(Cow::from("orange-300"), [253, 186, 116]);
        colors.insert(Cow::from("orange-400"), [251, 146, 60]);
        colors.insert(Cow::from("orange-500"), [249, 115, 22]);
        colors.insert(Cow::from("orange-600"), [234, 88, 12]);
        colors.insert(Cow::from("orange-700"), [194, 65, 12]);
        colors.insert(Cow::from("orange-800"), [154, 52, 18]);
        colors.insert(Cow::from("orange-900"), [124, 45, 18]);
        colors.insert(Cow::from("amber-50"), [255, 251, 235]);
        colors.insert(Cow::from("amber-100"), [254, 243, 199]);
        colors.insert(Cow::from("amber-200"), [253, 230, 138]);
        colors.insert(Cow::from("amber-300"), [252, 211, 77]);
        colors.insert(Cow::from("amber-400"), [251, 191, 36]);
        colors.insert(Cow::from("amber-500"), [245, 158, 11]);
        colors.insert(Cow::from("amber-600"), [217, 119, 6]);
        colors.insert(Cow::from("amber-700"), [180, 83, 9]);
        colors.insert(Cow::from("amber-800"), [146, 64, 14]);
        colors.insert(Cow::from("amber-900"), [120, 53, 15]);
        colors.insert(Cow::from("yellow-50"), [254, 252, 232]);
        colors.insert(Cow::from("yellow-100"), [254, 249, 195]);
        colors.insert(Cow::from("yellow-200"), [254, 240, 138]);
        colors.insert(Cow::from("yellow-300"), [253, 224, 71]);
        colors.insert(Cow::from("yellow-400"), [250, 204, 21]);
        colors.insert(Cow::from("yellow-500"), [234, 179, 8]);
        colors.insert(Cow::from("yellow-600"), [202, 138, 4]);
        colors.insert(Cow::from("yellow-700"), [161, 98, 7]);
        colors.insert(Cow::from("yellow-800"), [133, 77, 14]);
        colors.insert(Cow::from("yellow-900"), [113, 63, 18]);
        colors.insert(Cow::from("lime-50"), [247, 254, 231]);
        colors.insert(Cow::from("lime-100"), [236, 252, 203]);
        colors.insert(Cow::from("lime-200"), [217, 249, 157]);
        colors.insert(Cow::from("lime-300"), [190, 242, 100]);
        colors.insert(Cow::from("lime-400"), [163, 230, 53]);
        colors.insert(Cow::from("lime-500"), [132, 204, 22]);
        colors.insert(Cow::from("lime-600"), [101, 163, 13]);
        colors.insert(Cow::from("lime-700"), [77, 124, 15]);
        colors.insert(Cow::from("lime-800"), [63, 98, 18]);
        colors.insert(Cow::from("lime-900"), [54, 83, 20]);
        colors.insert(Cow::from("green-50"), [240, 253, 244]);
        colors.insert(Cow::from("green-100"), [220, 252, 231]);
        colors.insert(Cow::from("green-200"), [187, 247, 208]);
        colors.insert(Cow::from("green-300"), [134, 239, 172]);
        colors.insert(Cow::from("green-400"), [74, 222, 128]);
        colors.insert(Cow::from("green-500"), [34, 197, 94]);
        colors.insert(Cow::from("green-600"), [22, 163, 74]);
        colors.insert(Cow::from("green-700"), [21, 128, 61]);
        colors.insert(Cow::from("green-800"), [22, 101, 52]);
        colors.insert(Cow::from("green-900"), [20, 83, 45]);
        colors.insert(Cow::from("emerald-50"), [236, 253, 245]);
        colors.insert(Cow::from("emerald-100"), [209, 250, 229]);
        colors.insert(Cow::from("emerald-200"), [167, 243, 208]);
        colors.insert(Cow::from("emerald-300"), [110, 231, 183]);
        colors.insert(Cow::from("emerald-400"), [52, 211, 153]);
        colors.insert(Cow::from("emerald-500"), [16, 185, 129]);
        colors.insert(Cow::from("emerald-600"), [5, 150, 105]);
        colors.insert(Cow::from("emerald-700"), [4, 120, 87]);
        colors.insert(Cow::from("emerald-800"), [6, 95, 70]);
        colors.insert(Cow::from("emerald-900"), [6, 78, 59]);
        colors.insert(Cow::from("teal-50"), [240, 253, 250]);
        colors.insert(Cow::from("teal-100"), [204, 251, 241]);
        colors.insert(Cow::from("teal-200"), [153, 246, 228]);
        colors.insert(Cow::from("teal-300"), [94, 234, 212]);
        colors.insert(Cow::from("teal-400"), [45, 212, 191]);
        colors.insert(Cow::from("teal-500"), [20, 184, 166]);
        colors.insert(Cow::from("teal-600"), [13, 148, 136]);
        colors.insert(Cow::from("teal-700"), [15, 118, 110]);
        colors.insert(Cow::from("teal-800"), [17, 94, 89]);
        colors.insert(Cow::from("teal-900"), [19, 78, 74]);
        colors.insert(Cow::from("cyan-50"), [236, 254, 255]);
        colors.insert(Cow::from("cyan-100"), [207, 250, 254]);
        colors.insert(Cow::from("cyan-200"), [165, 243, 252]);
        colors.insert(Cow::from("cyan-300"), [103, 232, 249]);
        colors.insert(Cow::from("cyan-400"), [34, 211, 238]);
        colors.insert(Cow::from("cyan-500"), [6, 182, 212]);
        colors.insert(Cow::from("cyan-600"), [8, 145, 178]);
        colors.insert(Cow::from("cyan-700"), [14, 116, 144]);
        colors.insert(Cow::from("cyan-800"), [21, 94, 117]);
        colors.insert(Cow::from("cyan-900"), [22, 78, 99]);
        colors.insert(Cow::from("sky-50"), [240, 249, 255]);
        colors.insert(Cow::from("sky-100"), [224, 242, 254]);
        colors.insert(Cow::from("sky-200"), [186, 230, 253]);
        colors.insert(Cow::from("sky-300"), [125, 211, 252]);
        colors.insert(Cow::from("sky-400"), [56, 189, 248]);
        colors.insert(Cow::from("sky-500"), [14, 165, 233]);
        colors.insert(Cow::from("sky-600"), [2, 132, 199]);
        colors.insert(Cow::from("sky-700"), [3, 105, 161]);
        colors.insert(Cow::from("sky-800"), [7, 89, 133]);
        colors.insert(Cow::from("sky-900"), [12, 74, 110]);
        colors.insert(Cow::from("blue-50"), [239, 246, 255]);
        colors.insert(Cow::from("blue-100"), [219, 234, 254]);
        colors.insert(Cow::from("blue-200"), [191, 219, 254]);
        colors.insert(Cow::from("blue-300"), [147, 197, 253]);
        colors.insert(Cow::from("blue-400"), [96, 165, 250]);
        colors.insert(Cow::from("blue-500"), [59, 130, 246]);
        colors.insert(Cow::from("blue-600"), [37, 99, 235]);
        colors.insert(Cow::from("blue-700"), [29, 78, 216]);
        colors.insert(Cow::from("blue-800"), [30, 64, 175]);
        colors.insert(Cow::from("blue-900"), [30, 58, 138]);
        colors.insert(Cow::from("indigo-50"), [238, 242, 255]);
        colors.insert(Cow::from("indigo-100"), [224, 231, 255]);
        colors.insert(Cow::from("indigo-200"), [199, 210, 254]);
        colors.insert(Cow::from("indigo-300"), [165, 180, 252]);
        colors.insert(Cow::from("indigo-400"), [129, 140, 248]);
        colors.insert(Cow::from("indigo-500"), [99, 102, 241]);
        colors.insert(Cow::from("indigo-600"), [79, 70, 229]);
        colors.insert(Cow::from("indigo-700"), [67, 56, 202]);
        colors.insert(Cow::from("indigo-800"), [55, 48, 163]);
        colors.insert(Cow::from("indigo-900"), [49, 46, 129]);
        colors.insert(Cow::from("violet-50"), [245, 243, 255]);
        colors.insert(Cow::from("violet-100"), [237, 233, 254]);
        colors.insert(Cow::from("violet-200"), [221, 214, 254]);
        colors.insert(Cow::from("violet-300"), [196, 181, 253]);
        colors.insert(Cow::from("violet-400"), [167, 139, 250]);
        colors.insert(Cow::from("violet-500"), [139, 92, 246]);
        colors.insert(Cow::from("violet-600"), [124, 58, 237]);
        colors.insert(Cow::from("violet-700"), [109, 40, 217]);
        colors.insert(Cow::from("violet-800"), [91, 33, 182]);
        colors.insert(Cow::from("violet-900"), [76, 29, 149]);
        colors.insert(Cow::from("purple-50"), [250, 245, 255]);
        colors.insert(Cow::from("purple-100"), [243, 232, 255]);
        colors.insert(Cow::from("purple-200"), [233, 213, 255]);
        colors.insert(Cow::from("purple-300"), [216, 180, 254]);
        colors.insert(Cow::from("purple-400"), [192, 132, 252]);
        colors.insert(Cow::from("purple-500"), [168, 85, 247]);
        colors.insert(Cow::from("purple-600"), [147, 51, 234]);
        colors.insert(Cow::from("purple-700"), [126, 34, 206]);
        colors.insert(Cow::from("purple-800"), [107, 33, 168]);
        colors.insert(Cow::from("purple-900"), [88, 28, 135]);
        colors.insert(Cow::from("fuchsia-50"), [253, 244, 255]);
        colors.insert(Cow::from("fuchsia-100"), [250, 232, 255]);
        colors.insert(Cow::from("fuchsia-200"), [245, 208, 254]);
        colors.insert(Cow::from("fuchsia-300"), [240, 171, 252]);
        colors.insert(Cow::from("fuchsia-400"), [232, 121, 249]);
        colors.insert(Cow::from("fuchsia-500"), [217, 70, 239]);
        colors.insert(Cow::from("fuchsia-600"), [192, 38, 211]);
        colors.insert(Cow::from("fuchsia-700"), [162, 28, 175]);
        colors.insert(Cow::from("fuchsia-800"), [134, 25, 143]);
        colors.insert(Cow::from("fuchsia-900"), [112, 26, 117]);
        colors.insert(Cow::from("pink-50"), [253, 242, 248]);
        colors.insert(Cow::from("pink-100"), [252, 231, 243]);
        colors.insert(Cow::from("pink-200"), [251, 207, 232]);
        colors.insert(Cow::from("pink-300"), [249, 168, 212]);
        colors.insert(Cow::from("pink-400"), [244, 114, 182]);
        colors.insert(Cow::from("pink-500"), [236, 72, 153]);
        colors.insert(Cow::from("pink-600"), [219, 39, 119]);
        colors.insert(Cow::from("pink-700"), [190, 24, 93]);
        colors.insert(Cow::from("pink-800"), [157, 23, 77]);
        colors.insert(Cow::from("pink-900"), [131, 24, 67]);
        colors.insert(Cow::from("rose-50"), [255, 241, 242]);
        colors.insert(Cow::from("rose-100"), [255, 228, 230]);
        colors.insert(Cow::from("rose-200"), [254, 205, 211]);
        colors.insert(Cow::from("rose-300"), [253, 164, 175]);
        colors.insert(Cow::from("rose-400"), [251, 113, 133]);
        colors.insert(Cow::from("rose-500"), [244, 63, 94]);
        colors.insert(Cow::from("rose-600"), [225, 29, 72]);
        colors.insert(Cow::from("rose-700"), [190, 18, 60]);
        colors.insert(Cow::from("rose-800"), [159, 18, 57]);
        colors.insert(Cow::from("rose-900"), [136, 19, 55]);

        Self(colors)
    }
}

impl From<BTreeMap<Cow<'static, str>, [u8; 3]>> for ColorConfig {
    fn from(v: BTreeMap<Cow<'static, str>, [u8; 3]>) -> Self {
        Self(v)
    }
}

pub fn hex_to_rgb(hex: String) -> [u8; 3] {
    // Remove the useless `#` from the start of the color
    let hex = if let Some(hex) = hex.strip_prefix('#') {
        Cow::from(hex)
    } else {
        Cow::from(hex)
    };

    // Support the hexadecimal shorthand
    let hex = if hex.len() == 3 {
        hex.clone() + hex
    } else {
        hex
    };

    // TODO: Handle errors
    [
        u8::from_str_radix(&hex[0..2], 16).expect("failed to decode hex to rgb"),
        u8::from_str_radix(&hex[2..4], 16).expect("failed to decode hex to rgb"),
        u8::from_str_radix(&hex[4..6], 16).expect("failed to decode hex to rgb"),
    ]
}

fn convert_hex_to_rgb<'de, D>(d: D) -> std::result::Result<ColorConfig, D::Error> where D: Deserializer<'de> {
    struct ColorConfigHex;

    impl<'de> Visitor<'de> for ColorConfigHex {
        type Value = BTreeMap<Cow<'static, str>, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("map")
        }

        fn visit_map<M>(self, map: M) -> std::result::Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(MapAccessDeserializer::new(map))
        }
    }

    let mut colors_rgb_content = BTreeMap::new();

    let colors_hex = d.deserialize_any(ColorConfigHex)?;

    for color in colors_hex {
        colors_rgb_content.insert(color.0, hex_to_rgb(color.1));
    }

    Ok(ColorConfig(colors_rgb_content))
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct ThemeConfig {
    #[serde(default)]
    pub dark_mode: DarkModeConfig,

    #[serde(default)]
    pub screens: ScreenConfig,

    #[serde(default, deserialize_with = "convert_hex_to_rgb")]
    pub colors: ColorConfig,
}

#[derive(Default, PartialEq, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub input: Vec<PathBuf>,

    #[serde(default)]
    pub theme: ThemeConfig,

    // custom_variants: Vec<VariantConfig>,
    // custom_plugins: Vec<PluginConfig>,

    // TODO: Prefix (en-), preflight, safelist, separator for {variants, arbitrary values, modifiers}
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let mut config: Config = toml::from_str(&fs::read_to_string(&path).map_err(|e| Error::ConfigFileNotFound(path, e))?)?;

        if config.theme.colors != ColorConfig::default() {
            let overriden_colors = config.theme.colors.clone();
            config.theme.colors.extend(ColorConfig::default().iter().filter_map(|c| {
                let key = c.0.clone();
                if !overriden_colors.contains_key(&key) {
                    Some((key, *c.1))
                } else {
                    None
                }
            }));
        }

        if config.theme.screens != ScreenConfig::default() {
            let overriden_screens = config.theme.screens.clone();
            config.theme.screens.extend(ScreenConfig::default().iter().filter_map(|c| {
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
