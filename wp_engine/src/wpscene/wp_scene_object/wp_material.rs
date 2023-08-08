use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::wpscene::{from_str_to_arr2, from_str_to_arr3};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct WPMaterial {
    #[serde(default = "default_blending")]
    blending: String,

    #[serde(default = "default_cullmode")]
    cullmode: String,

    shader: String,

    #[serde(default = "default_depthtest")]
    depthtest: String,

    #[serde(default = "default_depthwrite")]
    depthwrite: String,

    textures: Vec<String>,
    combos: HashMap<String, i32>,
    constantshadervalues: HashMap<String, Vec<f32>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct WPMaterialPass {
    textures: Vec<Option<String>>,
    combos: HashMap<String, i32>,
    constantshadervalues: HashMap<String, WPMaterialVariant>,
    target: String,
    bind: Vec<WPMaterialPassBindItem>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WPMaterialVariant {
    Float(f32),

    #[serde(deserialize_with = "from_str_to_arr2")]
    Vec2([f32; 2]),

    #[serde(deserialize_with = "from_str_to_arr3")]
    Vec3([f32; 3]),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WPMaterialPassBindItem {
    name: String,
    index: i32,
}

fn default_blending() -> String {
    String::from("translucent")
}

fn default_cullmode() -> String {
    String::from("nocull")
}

fn default_depthtest() -> String {
    String::from("disabled")
}

fn default_depthwrite() -> String {
    String::from("disabled")
}
