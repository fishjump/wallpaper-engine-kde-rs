use std::os::raw::c_void;

use cpp::cpp;
use qmetaobject::scenegraph::SGNode;
use qttypes::{QColor, QRectF};

cpp! {{
    #include "src/scene_node/scene_node.cpp"
    #include "src/scene_node/scene_shader.cpp"
}}

pub fn pass_window_to_c(qquickitem: *mut c_void) {
    cpp!(unsafe [qquickitem as "QQuickItem *"] {
        window = qquickitem->window();
    });
}

pub struct SceneNode {}

pub trait SceneNodeTrait {
    fn new_if_null(&mut self) -> &mut Self;
    fn update_state(&mut self, rect: QRectF, time: f32) -> &mut Self;
}

impl SceneNodeTrait for SGNode<SceneNode> {
    fn new_if_null(&mut self) -> &mut SGNode<SceneNode> {
        if !self.raw.is_null() {
            return self;
        }

        self.raw = cpp!(unsafe [] -> *mut c_void as "void *" {
           return new SceneNode();
        });

        self
    }

    fn update_state(&mut self, rect: QRectF, time: f32) -> &mut SGNode<SceneNode> {
        if self.raw.is_null() {
            return self;
        }

        let raw = self.raw;
        cpp! (unsafe [raw as "SceneNode *", rect as "QRectF", time as "float"] {
            raw->updateState(rect, time);
        });

        self
    }
}
