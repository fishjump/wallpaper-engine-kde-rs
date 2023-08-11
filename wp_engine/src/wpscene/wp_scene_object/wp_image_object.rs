use serde::{Deserialize, Serialize};

use super::wp_image_effect::WPImageEffect;
use super::wp_material::WPMaterial;
use crate::wpscene::{from_str_to_arr2, from_str_to_arr3};

#[derive(Debug, Deserialize, Serialize)]
pub struct WPImageObject {
    #[serde(default = "default_id")]
    pub id: i32,

    pub name: Option<String>,

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_origin")]
    pub origin: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_angles")]
    pub angles: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr2")]
    #[serde(default = "default_size")]
    pub size: [f32; 2],

    #[serde(deserialize_with = "from_str_to_arr2")]
    #[serde(default = "default_parallax_depth")]
    #[serde(rename = "parallaxDepth")]
    pub parallax_depth: [f32; 2],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_color")]
    pub color: [f32; 3],

    #[serde(default = "default_color_blend_mode")]
    #[serde(rename = "colorBlendMode")]
    pub color_blend_mode: i32,

    #[serde(default = "default_alpha")]
    pub alpha: f32,

    #[serde(default = "default_brightness")]
    pub brightness: f32,

    #[serde(default = "default_fullscreen")]
    pub fullscreen: bool,

    #[serde(default = "default_nopadding")]
    pub nopadding: bool,

    #[serde(default = "default_visible")]
    pub visible: bool,

    pub image: String,

    #[serde(default = "default_alignment")]
    pub alignment: String,

    pub material: Option<WPMaterial>,

    #[serde(default)]
    pub effects: Vec<WPImageEffect>,

    #[serde(default = "default_config")]
    pub config: Config,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    passthrough: bool,
}

fn default_id() -> i32 {
    0
}

fn default_origin() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn default_angles() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_size() -> [f32; 2] {
    [2.0, 2.0]
}

fn default_parallax_depth() -> [f32; 2] {
    [0.0, 0.0]
}

fn default_color() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn default_color_blend_mode() -> i32 {
    0
}

fn default_alpha() -> f32 {
    1.0
}

fn default_brightness() -> f32 {
    1.0
}

fn default_fullscreen() -> bool {
    false
}

fn default_nopadding() -> bool {
    false
}

fn default_visible() -> bool {
    true
}

fn default_alignment() -> String {
    String::from("center")
}

fn default_config() -> Config {
    Config { passthrough: false }
}
