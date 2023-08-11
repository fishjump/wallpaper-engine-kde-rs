use std::os::raw::c_void;

use cpp::cpp;
use qmetaobject::scenegraph::SGNode;
use qttypes::QRectF;

cpp! {{
    #include "src/scene_node/bundle.cpp"
}}

pub fn pass_window_to_c(qquickitem: *mut c_void) {
    cpp!(unsafe [qquickitem as "QQuickItem *"] {
        window = qquickitem->window();
    });
}

pub struct SceneObject {}

pub trait SceneObjectTrait {
    fn new_if_null(&mut self) -> &mut Self;
    fn update_state(&mut self, rect: QRectF, time: f32) -> &mut Self;
}

impl SceneObjectTrait for SGNode<SceneObject> {
    fn new_if_null(&mut self) -> &mut SGNode<SceneObject> {
        if !self.raw.is_null() {
            return self;
        }

        self.raw = cpp!(unsafe [] -> *mut c_void as "void *" {
            SceneContext* ctx = new SceneContext();
            return new SceneObject{ctx};
        });

        self
    }

    fn update_state(
        &mut self,
        rect: QRectF,
        time: f32,
    ) -> &mut SGNode<SceneObject> {
        if self.raw.is_null() {
            return self;
        }

        let raw = self.raw;
        cpp! (unsafe [raw as "SceneObject *", rect as "QRectF", time as "float"] {
            raw->updateState(rect, time);
        });

        self
    }
}
