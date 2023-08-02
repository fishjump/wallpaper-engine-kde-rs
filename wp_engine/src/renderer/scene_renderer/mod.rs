use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, SGNode};
use wp_engine_bridge::qtdeclarative_ext::qquick_item_ext::{QQuickItemExt, QQuickItemFlag};
use wp_engine_bridge::scene_node::{SceneNode, SceneNodeTrait};

#[derive(QObject, Default)]
pub struct SceneRenderer {
    base: qt_base_class!(trait QQuickItem),
}

impl QQuickItem for SceneRenderer {
    fn geometry_changed(&mut self, _new_geometry: QRectF, _old_geometry: QRectF) {
        (self as &dyn QQuickItem).update();
    }

    fn update_paint_node(&mut self, mut node: SGNode<ContainerNode>) -> SGNode<ContainerNode> {
        let rect = (self as &dyn QQuickItem).bounding_rect();
        let colors = [
            QColor::from_rgb(255, 0, 0),
            QColor::from_rgb(0, 255, 0),
            QColor::from_rgb(0, 0, 255),
        ];

        node.update_static(|mut n: SGNode<SceneNode>| -> SGNode<SceneNode> {
            n.new_if_null().update_state(rect, colors);

            n
        });

        node
    }

    fn component_complete(&mut self) {
        self.set_flag(QQuickItemFlag::ItemHasContents);
    }
}
