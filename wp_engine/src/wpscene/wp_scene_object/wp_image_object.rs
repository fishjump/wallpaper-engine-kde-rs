use serde::{Deserialize, Serialize};

use super::wp_image_effect::WPImageEffect;
use super::wp_material::WPMaterial;
use crate::wpscene::{from_str_to_arr2, from_str_to_arr3};

#[derive(Debug, Deserialize, Serialize)]
pub struct WPImageObject {
    #[serde(default = "default_id")]
    id: i32,

    name: Option<String>,

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_origin")]
    origin: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_scale")]
    scale: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_angles")]
    angles: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr2")]
    #[serde(default = "default_size")]
    size: [f32; 2],

    #[serde(deserialize_with = "from_str_to_arr2")]
    #[serde(default = "default_parallax_depth")]
    #[serde(rename = "parallaxDepth")]
    parallax_depth: [f32; 2],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_color")]
    color: [f32; 3],

    #[serde(default = "default_color_blend_mode")]
    #[serde(rename = "colorBlendMode")]
    color_blend_mode: i32,

    #[serde(default = "default_alpha")]
    alpha: f32,

    #[serde(default = "default_brightness")]
    brightness: f32,

    #[serde(default = "default_fullscreen")]
    fullscreen: bool,

    #[serde(default = "default_nopadding")]
    nopadding: bool,

    #[serde(default = "default_visible")]
    visible: bool,

    image: String,

    #[serde(default = "default_alignment")]
    alignment: String,

    material: Option<WPMaterial>,

    #[serde(default)]
    effects: Vec<WPImageEffect>,

    #[serde(default = "default_config")]
    config: Config,
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
