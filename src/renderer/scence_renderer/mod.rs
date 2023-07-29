use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, RectangleNode, SGNode};

#[cfg(not(no_qt))]
use cpp::cpp;

cpp! {{
    #include <QtQuick/QQuickItem>
}}

#[derive(QObject, Default)]
pub struct ScenceRenderer {
    base: qt_base_class!(trait QQuickItem),
}

impl ScenceRenderer {
    fn set_flag(&mut self) {
        let obj = self.get_cpp_object();
        let flag = QQuickItemFlag::ItemHasContents;
        assert!(!obj.is_null());
        cpp!(unsafe [obj as "QQuickItem *", flag as "QQuickItem::Flag"] {
            obj->setFlag(flag);
        });
    }
}

impl QQuickItem for ScenceRenderer {
    fn geometry_changed(&mut self, _new_geometry: QRectF, _old_geometry: QRectF) {
        (self as &dyn QQuickItem).update();
    }

    fn update_paint_node(&mut self, mut node: SGNode<ContainerNode>) -> SGNode<ContainerNode> {
        let rect = (self as &dyn QQuickItem).bounding_rect();
        node.update_static((|mut n: SGNode<RectangleNode>| -> SGNode<RectangleNode> {
            n.create(self);
            n.set_rect(rect);
            n.set_color(QColor::from_name("steelblue"));
            n
        },));
        node
    }

    fn component_complete(&mut self) {
        self.set_flag();
    }
}

#[repr(C)]
#[allow(unused)]
enum QQuickItemFlag {
    ItemClipsChildrenToShape = 0x01,
    ItemAcceptsInputMethod = 0x02,
    ItemIsFocusScope = 0x04,
    ItemHasContents = 0x08,
    ItemAcceptsDrops = 0x10,
}
