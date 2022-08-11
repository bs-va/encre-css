use serde::Deserialize;
use std::collections::BTreeMap;

fn default_icon_left() -> f32 {
    0.
}

fn default_icon_top() -> f32 {
    0.
}

fn default_icon_width() -> f32 {
    16.
}

fn default_icon_height() -> f32 {
    16.
}

fn default_icon_rotate() -> f32 {
    0.
}

fn default_icon_h_flip() -> bool {
    false
}

fn default_icon_v_flip() -> bool {
    false
}

fn default_aliases() -> BTreeMap<String, Alias> {
    BTreeMap::new()
}

fn default_chars() -> BTreeMap<String, String> {
    BTreeMap::new()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconOptional {
    #[serde(default = "default_icon_left")]
    pub left: f32,

    #[serde(default = "default_icon_top")]
    pub top: f32,

    pub width: Option<f32>,

    pub height: Option<f32>,

    #[serde(default = "default_icon_rotate")]
    pub rotate: f32,

    #[serde(default = "default_icon_h_flip")]
    pub h_flip: bool,

    #[serde(default = "default_icon_v_flip")]
    pub v_flip: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub body: String,

    #[serde(flatten)]
    pub optional: IconOptional,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alias {
    pub parent: String,

    #[serde(flatten)]
    pub optional: IconOptional,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub prefix: String,
    pub icons: BTreeMap<String, Icon>,

    #[serde(default = "default_aliases")]
    pub aliases: BTreeMap<String, Alias>,

    #[serde(default = "default_chars")]
    pub chars: BTreeMap<String, String>,

    #[serde(default = "default_icon_width")]
    pub width: f32,

    #[serde(default = "default_icon_height")]
    pub height: f32,
}
