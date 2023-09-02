use cpp::cpp;
use qttypes::QImage;

use crate::qt_ext::texture::Texture;

cpp! {{
    #include <QtQuick/QQuickWindow>
    #include <QtGui/QImage>
}}

pub enum QQuickWindow {}

impl QQuickWindow {
    pub fn create_texture_from_image(
        &mut self,
        image: &QImage,
    ) -> &mut Texture {
        cpp!(unsafe [self as "QQuickWindow*", image as "QImage *"] -> &mut Texture as "QSGTexture*" {
            return self->createTextureFromImage(*image);
        })
    }
}
