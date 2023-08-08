use serde::{Deserialize, Serialize};

use super::wp_material::{WPMaterial, WPMaterialPass};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct WPImageEffect {
    id: i32,
    name: String,

    #[serde(default = "default_visible")]
    visible: bool,

    version: i32,
    materials: Vec<WPMaterial>,
    passes: Vec<WPMaterialPass>,
    commands: Vec<WPEffectCommand>,
    fbos: Vec<WPEffectFbo>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct WPEffectCommand {
    command: String,
    target: String,
    source: String,

    #[serde(default = "default_afterpos")]
    afterpos: i32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct WPEffectFbo {
    name: String,
    format: String,

    #[serde(default = "default_scale")]
    scale: u32,
}

fn default_visible() -> bool {
    true
}

fn default_afterpos() -> i32 {
    0
}

fn default_scale() -> u32 {
    1
}
