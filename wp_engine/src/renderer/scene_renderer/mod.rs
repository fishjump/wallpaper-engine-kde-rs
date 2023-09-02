mod consts;

use std::default;

use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, SGNode};
use qttypes::QImage;
use wp_engine_bridge::qt_ext::qquick_item_ext::{
    QQuickItemExt, QQuickItemFlag,
};
use wp_engine_bridge::qt_ext::texture::Texture;
use wp_engine_bridge::scene_node::gl_data::GlData;
use wp_engine_bridge::scene_node::{SceneObject, SceneObjectTrait};
use wp_engine_bridge::utils::AsRawPtr;

use self::consts::{FRAGMENT_SHADER, VERTEX_SHADER};

#[derive(QObject)]
pub struct SceneRenderer {
    base: qt_base_class!(trait QQuickItem),
    timer_inc: qt_method!(
        fn timer_inc(&mut self, interval: f32) {
            self.time += interval;
            (self as &dyn QQuickItem).update();
        }
    ),

    time: f32,
    initialized: bool,
    texture_image_0: QImage,
    texture_image_1: QImage,
    texture_image_2: QImage,
    texture_0: *mut Texture,
    texture_1: *mut Texture,
    texture_2: *mut Texture,
}

impl Default for SceneRenderer {
    fn default() -> Self {
        Self {
            base: Default::default(),
            timer_inc: Default::default(),

            time: Default::default(),
            initialized: false,

            texture_image_0: Default::default(),
            texture_image_1: Default::default(),
            texture_image_2: Default::default(),

            texture_0: std::ptr::null_mut(),
            texture_1: std::ptr::null_mut(),
            texture_2: std::ptr::null_mut(),
        }
    }
}

impl SceneRenderer {
    fn initialize(&mut self) -> &mut Self {
        if self.initialized {
            return self;
        }

        let window = self.window();

        let texture_image_0 = QImage::load_from_file(
                QString::from("/home/yuey/repos/wallpaper-engine-kde-rs/assets/wallpaper/materials/00009.png")
            );
        let texture_image_1 =  QImage::load_from_file(
                QString::from("/home/yuey/repos/wallpaper-engine-kde-rs/assets/wallpaper/materials/masks/waterwaves_mask_c40ed93b.png")
            );
        let texture_image_2 = QImage::load_from_file(
                QString::from("/home/yuey/.local/share/Steam/steamapps/common/wallpaper_engine/assets/materials/util/black.png")
            );

        let texture_0 = window
            .create_texture_from_image(&texture_image_0)
            .as_raw_ptr();
        let texture_1 = window
            .create_texture_from_image(&texture_image_1)
            .as_raw_ptr();
        let texture_2 = window
            .create_texture_from_image(&texture_image_2)
            .as_raw_ptr();

        self.initialized = true;

        self.texture_image_0 = texture_image_0;
        self.texture_image_1 = texture_image_1;
        self.texture_image_2 = texture_image_2;

        self.texture_0 = texture_0;
        self.texture_1 = texture_1;
        self.texture_2 = texture_2;

        self
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
        self.initialize();

        let rect = (self as &dyn QQuickItem).bounding_rect();

        node.update_static(
            |mut n: SGNode<SceneObject>| -> SGNode<SceneObject> {
                let res = n.new(VERTEX_SHADER, FRAGMENT_SHADER);
                if let Err(e) = res {
                    println!("Failed to create SceneObject: {}", e);
                    return n;
                }

                // from assets/wallpaper/scene.json
                n.update_geometry(rect)
                    .update_uniform("g_Point0", &GlData::GLVec2([0.0, 0.0]))
                    .update_uniform("g_Point1", &GlData::GLVec2([1.0, 0.0]))
                    .update_uniform("g_Point2", &GlData::GLVec2([1.0, 1.0]))
                    .update_uniform("g_Point3", &GlData::GLVec2([0.0, 1.0]))
                    .update_uniform("g_Direction", &GlData::GLFloat(2.7933424))
                    .update_uniform("g_Speed", &GlData::GLFloat(0.79000002))
                    .update_uniform("g_Scale", &GlData::GLFloat(15.63))
                    .update_uniform("g_Strength", &GlData::GLFloat(0.029999999))
                    .update_uniform("g_Perspective", &GlData::GLFloat(0.003))
                    .update_uniform("g_Time", &GlData::GLFloat(self.time))
                    .update_uniform(
                        "g_Texture0",
                        &GlData::GLSampler2D(self.texture_0, 0),
                    )
                    .update_uniform(
                        "g_Texture1",
                        &GlData::GLSampler2D(self.texture_1, 1),
                    )
                    .update_uniform(
                        "g_Texture2",
                        &GlData::GLSampler2D(self.texture_2, 2),
                    )
                    .update_uniform(
                        "g_Texture0Resolution",
                        &GlData::GLVec4([
                            self.texture_image_0.size().width as f32,
                            self.texture_image_0.size().height as f32,
                            self.texture_image_0.size().width as f32,
                            self.texture_image_0.size().height as f32,
                        ]),
                    )
                    .update_uniform(
                        "g_Texture1Resolution",
                        &GlData::GLVec4([
                            self.texture_image_1.size().width as f32,
                            self.texture_image_1.size().height as f32,
                            self.texture_image_1.size().width as f32,
                            self.texture_image_1.size().height as f32,
                        ]),
                    )
                    .update_uniform(
                        "g_Texture2Resolution",
                        &GlData::GLVec4([
                            self.texture_image_2.size().width as f32,
                            self.texture_image_2.size().height as f32,
                            self.texture_image_2.size().width as f32,
                            self.texture_image_2.size().height as f32,
                        ]),
                    );

                n
            },
        );

        node
    }

    fn component_complete(&mut self) {
        self.set_flag(QQuickItemFlag::ItemHasContents);
    }
}
