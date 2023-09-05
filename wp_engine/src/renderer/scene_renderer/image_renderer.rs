use std::collections::HashMap;

use qttypes::QImage;
use wp_engine_bridge::scene_node::gl_data::GlData;
// use wp_engine_bridge::scene_node::SceneObject;

pub struct ImageRenderer {
    image: QImage,
    effects: Vec<Effect>,
}

pub struct Effect {
    // object: SceneObject,
    
    uniforms: HashMap<String, GlData>,
}
