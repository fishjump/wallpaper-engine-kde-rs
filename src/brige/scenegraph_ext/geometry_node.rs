use std::os::raw::c_void;

use cpp::cpp;
use qmetaobject::scenegraph::SGNode;
use qttypes::QRectF;

use super::material::QSGMaterial;

cpp! {{
    #include <QtQuick/QSGGeometryNode>
    #include <QtQuick/QSGMaterial>
    #include <QtQuick/QSGGeometry>
}}

pub enum GeometryNode {}

pub enum QSGGeometry {}

pub trait GeometryNodeTrait {
    fn material(&self) -> &mut QSGMaterial;
    fn geometry(&self) -> &mut QSGGeometry;
    fn set_material(&self, material: *mut QSGMaterial);
    fn set_geometry(&self, geometry: *mut QSGGeometry);
    fn set_flag(&self, flag: QSGNodeFlag);
    fn mark_dirty(&self, bits: u32);
    fn create(&mut self);
}

impl GeometryNodeTrait for SGNode<GeometryNode> {
    fn material(&self) -> &mut QSGMaterial {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGGeometryNode*"] -> &mut QSGMaterial as  "QSGMaterial*" {
            return raw->material();
        })
    }

    fn geometry(&self) -> &mut QSGGeometry {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGGeometryNode*"] -> &mut QSGGeometry as  "QSGGeometry *" {
            return raw->geometry();
        })
    }

    fn set_material(&self, material: *mut QSGMaterial) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGGeometryNode*", material as "QSGMaterial*"] {
            if (raw) raw->setMaterial(material);
        });
    }

    fn set_geometry(&self, geometry: *mut QSGGeometry) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGGeometryNode*", geometry as "QSGGeometry*"] {
            if (raw) raw->setGeometry(geometry);
        });
    }

    fn set_flag(&self, flag: QSGNodeFlag) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGGeometryNode*", flag as "QSGNode::Flag"] {
            if (raw) raw->setFlag(flag);
        });
    }

    fn mark_dirty(&self, bits: u32) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGGeometryNode*", bits as "QSGNode::DirtyState"] {
            if (raw) raw->markDirty(bits);
        });
    }

    fn create(&mut self) {
        if !self.raw.is_null() {
            return;
        }

        self.raw = cpp!(unsafe [] -> &mut c_void as "void*" {
           return new QSGGeometryNode();
        });
    }
}

#[repr(C)]
pub enum QSGNodeFlag {
    OwnedByParent = 0x0001,
    UsePreprocess = 0x0002,
    OwnsGeometry = 0x00010000,
    OwnsMaterial = 0x00020000,
    OwnsOpaqueMaterial = 0x00040000,
    InternalReserved = 0x01000000,
}

#[repr(C)]
pub enum QSGNodeDirtyState {
    DirtyMatrix = 0x0100,
    DirtyNodeAdded = 0x0400,
    DirtyNodeRemoved = 0x0800,
    DirtyGeometry = 0x1000,
    DirtyMaterial = 0x2000,
    DirtyOpacity = 0x4000,
    DirtySubtreeBlocked = 0x0080,
}
