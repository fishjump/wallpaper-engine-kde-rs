use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, SGNode};
use qttypes::{ImageFormat, QImage, QSize};

use crate::brige::qtdeclarative_ext::qquick_item_ext::{QQuickItemExt, QQuickItemFlag};
use crate::brige::scenegraph_ext::geometry_node::{
    GeometryNode, GeometryNodeTrait, QSGGeometry, QSGNodeFlag,
};
use crate::brige::scenegraph_ext::material;
use crate::brige::scenegraph_ext::simple_texture_node::{
    SimpleTextureNode, SimpleTextureNodeTrait,
};
use crate::brige::shader;
use crate::brige::shader::custom_shader::{
    create_qsggeometry, update_position, CustomMaterialShader,
};
use crate::brige::utils::AsRawPtr;

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
        // let mut image = QImage::new(
        //     QSize {
        //         width: 128,
        //         height: 128,
        //     },
        //     ImageFormat::RGB32,
        // );
        // image.fill(QColor::from_name("red"));

        // let texture = self.window().create_texture_from_image(image).as_raw_ptr();

        let material = CustomMaterialShader::new();
        let geometry = create_qsggeometry();

        node.update_static((|mut n: SGNode<GeometryNode>| -> SGNode<GeometryNode> {
            n.create();
            n.set_geometry(geometry);
            n.set_flag(QSGNodeFlag::OwnsGeometry);

            n.set_material(material);
            n.set_flag(QSGNodeFlag::OwnsMaterial);

            update_position(geometry, 50.0, 0.0, 100.0, 100.0, 0.0, 100.0);

            n.mark_dirty(0x1000);

            n
        },));

        node
    }

    fn component_complete(&mut self) {
        self.set_flag(QQuickItemFlag::ItemHasContents);
    }
}
