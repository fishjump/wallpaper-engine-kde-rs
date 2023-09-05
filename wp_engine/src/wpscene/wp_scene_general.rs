use serde::{Deserialize, Serialize};

use super::from_str_to_arr3;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WPSceneGeneral {
    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_ambientcolor")]
    ambientcolor: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_clearcolor")]
    clearcolor: [f32; 3],

    #[serde(default = "default_cameraparallax")]
    cameraparallax: bool,

    cameraparallaxamount: f32,
    cameraparallaxdelay: f32,
    cameraparallaxmouseinfluence: f32,

    #[serde(default = "default_farz")]
    farz: f32,

    #[serde(default = "default_fov")]
    fov: f32,

    #[serde(default = "default_nearz")]
    nearz: f32,

    #[serde(default = "default_orthogonalprojection")]
    orthogonalprojection: Option<Orthogonalprojection>,

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_skylightcolor")]
    skylightcolor: [f32; 3],

    #[serde(default = "default_zoom")]
    zoom: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Orthogonalprojection {
    width: i32,
    height: i32,

    #[serde(default = "default_auto")]
    auto: bool,
}

fn default_ambientcolor() -> [f32; 3] {
    [0.2, 0.2, 0.2]
}

fn default_clearcolor() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_cameraparallax() -> bool {
    false
}

fn default_farz() -> f32 {
    10000.0
}

fn default_fov() -> f32 {
    50.0
}

fn default_nearz() -> f32 {
    0.01
}

fn default_orthogonalprojection() -> Option<Orthogonalprojection> {
    Some(Orthogonalprojection {
        width: 1920,
        height: 1080,
        auto: false,
    })
}

fn default_skylightcolor() -> [f32; 3] {
    [0.3, 0.3, 0.3]
}

fn default_zoom() -> f32 {
    1.0
}

fn default_auto() -> bool {
    false
}
