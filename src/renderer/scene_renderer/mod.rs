use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, SGNode};
use qttypes::{ImageFormat, QImage, QSize};

use crate::brige::qtdeclarative_ext::qquick_item_ext::{QQuickItemExt, QQuickItemFlag};
use crate::brige::scene_node::{SceneNode, SceneNodeTrait};
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

        let geometry = create_qsggeometry();
        let material = CustomMaterialShader::new();

        node.update_static((
            |mut n: SGNode<SceneNode>| -> SGNode<SceneNode> {
                n.new_if_null().update_state(rect, colors);

                n
            },
            |mut n: SGNode<GeometryNode>| -> SGNode<GeometryNode> {
                n.create();
                n.set_geometry(geometry);
                n.set_flag(QSGNodeFlag::OwnsGeometry);

                n.set_material(material);
                n.set_flag(QSGNodeFlag::OwnsMaterial);

                update_position(geometry, 20.0, 0.0, 50.0, 50.0, 0.0, 50.0);

                n.mark_dirty(0x1000);

                n
            },
        ));

        node
    }

    fn component_complete(&mut self) {
        self.set_flag(QQuickItemFlag::ItemHasContents);
    }
}
