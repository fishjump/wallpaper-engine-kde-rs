pub mod wp_image_effect;
pub mod wp_image_object;
pub mod wp_material;
pub mod wp_particle_object;

use serde::{Deserialize, Serialize};

use self::wp_image_object::WPImageObject;
use self::wp_particle_object::WPParticleObject;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WPSceneObject {
    WPImageObject(WPImageObject),
    WPParticleObject(WPParticleObject),
    WPSoundObject(WPSoundObject),
    WPLightObject(WPLightObject),
}

// TBD
#[derive(Debug, Deserialize, Serialize)]
pub struct WPSoundObject {
    sound: Vec<String>,
}

// TBD
#[derive(Debug, Deserialize, Serialize)]
pub struct WPLightObject {
    light: String,
}
