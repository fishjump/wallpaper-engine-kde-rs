use serde::{Deserialize, Serialize};

use super::from_str_to_arr3;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WPSceneCamera {
    #[serde(deserialize_with = "from_str_to_arr3")]
    center: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    eye: [f32; 3],

    #[serde(deserialize_with = "from_str_to_arr3")]
    up: [f32; 3],
}
