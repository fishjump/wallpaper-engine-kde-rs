use std::os::raw::c_void;

use cpp::cpp;
use qmetaobject::scenegraph::SGNode;
use qttypes::QRectF;

use super::texture;

cpp! {{
    #include <QtQuick/QSGSimpleTextureNode>
    #include <QtQuick/QSGTexture>
}}

pub enum SimpleTextureNode {}

pub trait SimpleTextureNodeTrait {
    fn filtering(&self) -> texture::Filtering;
    fn owns_texture(&self) -> bool;
    fn rect(&self) -> QRectF;
    fn set_filtering(&mut self, filtering: texture::Filtering);
    fn set_own_texture(&mut self, owns: bool);
    fn set_rect(&mut self, rect: QRectF);
    fn set_source_rect(&mut self, rect: QRectF);
    fn set_texture(&mut self, texture: *mut texture::Texture);
    fn set_texture_coordinates_transform(&mut self, mode: TextureCoordinatesTransformMode);
    fn source_rect(&self) -> QRectF;
    fn texture(&self) -> &mut texture::Texture;
    fn texture_coordinates_transform(&self) -> TextureCoordinatesTransformMode;
    fn create(&mut self);
}

impl SimpleTextureNodeTrait for SGNode<SimpleTextureNode> {
    fn filtering(&self) -> texture::Filtering {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGSimpleTextureNode*"] -> texture::Filtering as  "QSGTexture::Filtering" {
            return raw->filtering();
        })
    }

    fn owns_texture(&self) -> bool {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGSimpleTextureNode*"] -> bool as "bool" {
            return raw->ownsTexture();
        })
    }

    fn rect(&self) -> QRectF {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGSimpleTextureNode*"] -> QRectF as "QRectF" {
            return raw->rect();
        })
    }

    fn set_filtering(&mut self, filtering: texture::Filtering) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGSimpleTextureNode*", filtering as "QSGTexture::Filtering"] {
            if (raw) raw->setFiltering(filtering);
        });
    }

    fn set_own_texture(&mut self, owns: bool) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGSimpleTextureNode*", owns as "bool"] {
            if (raw) raw->setOwnsTexture(owns);
        });
    }

    fn set_rect(&mut self, rect: QRectF) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGSimpleTextureNode*", rect as "QRectF"] {
            if (raw) raw->setRect(rect);
        });
    }

    fn set_source_rect(&mut self, rect: QRectF) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGSimpleTextureNode*", rect as "QRectF"] {
            if (raw) raw->setSourceRect(rect);
        });
    }

    fn set_texture(&mut self, texture: *mut texture::Texture) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGSimpleTextureNode*", texture as "QSGTexture*"] {
            if (raw) raw->setTexture(texture);
        });
    }

    fn set_texture_coordinates_transform(&mut self, mode: TextureCoordinatesTransformMode) {
        let raw = self.raw;
        cpp!(unsafe [raw as "QSGSimpleTextureNode*", mode as "QSGSimpleTextureNode::TextureCoordinatesTransformMode"] {
            if (raw) raw->setTextureCoordinatesTransform(mode);
        });
    }

    fn source_rect(&self) -> QRectF {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGSimpleTextureNode*"] -> QRectF as "QRectF" {
            return raw->sourceRect();
        })
    }

    fn texture(&self) -> &mut texture::Texture {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGSimpleTextureNode*"] -> &mut texture::Texture as "QSGTexture*" {
            return raw->texture();
        })
    }

    fn texture_coordinates_transform(&self) -> TextureCoordinatesTransformMode {
        let raw = self.raw;
        assert!(!raw.is_null());
        cpp!(unsafe [raw as "QSGSimpleTextureNode*"] -> TextureCoordinatesTransformMode as "QSGSimpleTextureNode::TextureCoordinatesTransformMode" {
            return raw->textureCoordinatesTransform();
        })
    }

    fn create(&mut self) {
        if !self.raw.is_null() {
            return;
        }
        self.raw = cpp!(unsafe [] -> &mut c_void as "void*" {
           return new QSGSimpleTextureNode();
        });
    }
}

#[repr(C)]
pub enum TextureCoordinatesTransformMode {
    NoTransform = 0x00,
    MirrorHorizontall = 0x01,
    MirrorVertically = 0x02,
}
