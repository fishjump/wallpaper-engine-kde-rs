use serde::{Deserialize, Serialize};

use super::wp_scene_camera::WPSceneCamera;
use super::wp_scene_general::WPSceneGeneral;
use super::wp_scene_object::WPSceneObject;

#[derive(Debug, Deserialize, Serialize)]
pub struct WPScene {
    camera: WPSceneCamera,
    general: WPSceneGeneral,
    objects: Vec<WPSceneObject>,
    version: usize,
}
