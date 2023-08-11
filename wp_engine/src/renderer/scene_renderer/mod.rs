use std::os::raw::c_void;

use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, SGNode};
use qttypes::{ImageFormat, QImage, QSize};
use wp_engine_bridge::qtdeclarative_ext::qquick_item_ext::{
    QQuickItemExt, QQuickItemFlag,
};
use wp_engine_bridge::scene_node::{
    pass_window_to_c, SceneObject, SceneObjectTrait,
};

#[derive(QObject, Default)]
pub struct SceneRenderer {
    base: qt_base_class!(trait QQuickItem),
    timer_inc: qt_method!(
        fn timer_inc(&mut self, interval: f32) {
            self.time += interval;
            (self as &dyn QQuickItem).update();
        }
    ),
    time: f32,
}

impl SceneRenderer {
    fn time(&self) -> f32 {
        self.time
    }
}

impl QQuickItem for SceneRenderer {
    fn geometry_changed(
        &mut self,
        _new_geometry: QRectF,
        _old_geometry: QRectF,
    ) {
        (self as &dyn QQuickItem).update();
    }

    fn update_paint_node(
        &mut self,
        mut node: SGNode<ContainerNode>,
    ) -> SGNode<ContainerNode> {
        let rect = (self as &dyn QQuickItem).bounding_rect();

        pass_window_to_c(self.get_cpp_object());

        node.update_static(
            |mut n: SGNode<SceneObject>| -> SGNode<SceneObject> {
                n.new_if_null().update_state(rect, self.time());

                n
            },
        );

        node
    }

    fn component_complete(&mut self) {
        self.set_flag(QQuickItemFlag::ItemHasContents);
    }
}
