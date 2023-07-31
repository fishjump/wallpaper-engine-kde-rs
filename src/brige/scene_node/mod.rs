use std::os::raw::c_void;

use cpp::cpp;
use qmetaobject::scenegraph::SGNode;
use qttypes::{QColor, QRectF};

cpp! {{
    #include "src/brige/scene_node/scene_node.cpp"
}}

pub struct SceneNode {}

pub trait SceneNodeTrait {
    fn new_if_null(&mut self) -> &mut Self;
    fn update_state(&mut self, rect: QRectF, colors: [QColor; 3]) -> &mut Self;
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

    fn update_state(&mut self, rect: QRectF, colors: [QColor; 3]) -> &mut SGNode<SceneNode> {
        if self.raw.is_null() {
            return self;
        }

        let raw = self.raw;
        let colors = colors.as_ptr();
        cpp! (unsafe [raw as "SceneNode *", rect as "QRectF", colors as "QColor const *"] {
            raw->updateState(rect, colors);
        });

        self
    }
}
