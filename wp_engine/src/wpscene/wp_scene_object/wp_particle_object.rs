use serde::{Deserialize, Serialize};

use crate::wpscene::{from_str_to_arr2, from_str_to_arr3};

#[derive(Debug, Deserialize, Serialize)]
pub struct WPParticleObject {
    id: i32,
    name: String,

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_origin")]
    origin: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_scale")]
    scale: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    #[serde(default = "default_angle")]
    angle: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr2")]
    #[serde(default = "default_parallax_depth")]
    parallax_depth: [f32; 2],

    #[serde(default = "default_visible")]
    visible: bool,

    particle: String,
    // particle_obj: WPParticle,
    // instanceoverride: WPParticleInstanceoverride,
}

fn default_origin() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn default_angle() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_parallax_depth() -> [f32; 2] {
    [0.0, 0.0]
}

fn default_visible() -> bool {
    true
}
