use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, SGNode};
use qttypes::{ImageFormat, QImage, QSize};

use crate::brige::qtdeclarative_ext::qquick_item_ext::{QQuickItemExt, QQuickItemFlag};
use crate::brige::scenegraph_ext::simple_texture_node::{
    SimpleTextureNode, SimpleTextureNodeTrait,
};

#[derive(QObject, Default)]
pub struct ScenceRenderer {
    base: qt_base_class!(trait QQuickItem),
}

impl QQuickItem for ScenceRenderer {
    fn geometry_changed(&mut self, _new_geometry: QRectF, _old_geometry: QRectF) {
        (self as &dyn QQuickItem).update();
    }

    fn update_paint_node(&mut self, mut node: SGNode<ContainerNode>) -> SGNode<ContainerNode> {
        let rect = (self as &dyn QQuickItem).bounding_rect();
        let mut image = QImage::new(
            QSize {
                width: 128,
                height: 128,
            },
            ImageFormat::RGB32,
        );
        image.fill(QColor::from_name("red"));

        let texture = self.window().create_texture_from_image(image).into();

        node.update_static((
            |mut n: SGNode<SimpleTextureNode>| -> SGNode<SimpleTextureNode> {
                n.create();
                n.set_texture(texture);
                n.set_rect(rect);
                n
            },
        ));
        node
    }

    fn component_complete(&mut self) {
        self.set_flag(QQuickItemFlag::ItemHasContents);
    }
}
