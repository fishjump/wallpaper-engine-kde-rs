use cpp::cpp;
use qttypes::QImage;

use crate::brige::scenegraph_ext::texture::Texture;

cpp! {{
    #include <QtQuick/QQuickWindow>
    #include <QtGui/QImage>
}}

pub enum QQuickWindow {}

impl QQuickWindow {
    pub fn create_texture_from_image(&mut self, image: QImage) -> &mut Texture {
        cpp!(unsafe [self as "QQuickWindow*", image as "QImage"] -> &mut Texture as "QSGTexture*" {
            return self->createTextureFromImage(image);
        })
    }

    pub fn create_texture_from_image_option(
        &mut self,
        image: QImage,
        options: CreateTextureOptions,
    ) -> &mut Texture {
        cpp!(unsafe [self as "QQuickWindow*", image as "QImage", options as "QQuickWindow::CreateTextureOptions"] -> &mut Texture as "QSGTexture*" {
            return self->createTextureFromImage(image, options);
        })
    }
}

#[repr(C)]
pub enum CreateTextureOptions {
    TextureHasAlphaChannel = 0x0001,
    TextureHasMipmaps = 0x0002,
    TextureOwnsGLTexture = 0x0004,
    TextureCanUseAtlas = 0x0008,
    TextureIsOpaque = 0x0010,
}
