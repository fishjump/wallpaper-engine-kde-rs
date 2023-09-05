mod consts;
mod image_renderer;

use qmetaobject::prelude::*;
use qmetaobject::scenegraph::{ContainerNode, SGNode};
use qttypes::QImage;
use wp_engine_bridge::qt_ext::qquick_item_ext::{
    QQuickItemExt, QQuickItemFlag,
};
use wp_engine_bridge::qt_ext::texture::Texture;
use wp_engine_bridge::scene_node::gl_data::GlData;
use wp_engine_bridge::scene_node::{SceneObjectNode, SceneObjectTrait};
use wp_engine_bridge::utils::AsRawPtr;

use self::consts::{
    FRAGMENT_SHADER, FRAGMENT_SHADER_1, FRAGMENT_SHADER_2, VERTEX_SHADER,
    VERTEX_SHADER_1, VERTEX_SHADER_2,
};
use crate::wpscene::wp_scene::WPScene;

#[derive(QObject)]
pub struct SceneRenderer {
    base: qt_base_class!(trait QQuickItem),
    tick_tock: qt_method!(
        fn tick_tock(&mut self, interval: f32) {
            self.time += interval;
            self.time %= 100.0; // in case of overflow
            (self as &dyn QQuickItem).update();
        }
    ),

    time: f32,

    initialized: bool,
    // metadata: WPScene,
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
            tick_tock: Default::default(),
            time: Default::default(),

            initialized: false,
            // metadata: Default::default(),
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

        node.update_static((
            |mut n: SGNode<SceneObjectNode>| -> SGNode<SceneObjectNode> {
                let res = n.new();
                if let Err(e) = res {
                    println!("Failed to create SceneObject: {}", e);
                    return n;
                }

                let program_id =
                    match n.add_effect(VERTEX_SHADER, FRAGMENT_SHADER) {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Failed to add effect: {}", "error");
                            return n;
                        }
                    };

                n.update_uniform(
                    program_id,
                    "g_Point0",
                    &GlData::GLVec2([0.0, 0.0]),
                )
                .update_uniform(
                    program_id,
                    "g_Point1",
                    &GlData::GLVec2([1.0, 0.0]),
                )
                .update_uniform(
                    program_id,
                    "g_Point2",
                    &GlData::GLVec2([1.0, 1.0]),
                )
                .update_uniform(
                    program_id,
                    "g_Point3",
                    &GlData::GLVec2([0.0, 1.0]),
                )
                .update_uniform(
                    program_id,
                    "g_Direction",
                    &GlData::GLFloat(2.7933424),
                )
                .update_uniform(
                    program_id,
                    "g_Speed",
                    &GlData::GLFloat(0.79000002),
                )
                .update_uniform(program_id, "g_Scale", &GlData::GLFloat(15.63))
                .update_uniform(
                    program_id,
                    "g_Strength",
                    &GlData::GLFloat(0.029999999),
                )
                .update_uniform(
                    program_id,
                    "g_Perspective",
                    &GlData::GLFloat(0.003),
                )
                .update_uniform(
                    program_id,
                    "g_Time",
                    &GlData::GLFloat(self.time),
                )
                .update_uniform(
                    program_id,
                    "g_Texture0",
                    &GlData::GLSampler2D(self.texture_0, 0),
                )
                .update_uniform(
                    program_id,
                    "g_Texture1",
                    &GlData::GLSampler2D(self.texture_1, 1),
                )
                .update_uniform(
                    program_id,
                    "g_Texture2",
                    &GlData::GLSampler2D(self.texture_2, 2),
                )
                .update_uniform(
                    program_id,
                    "g_Texture0Resolution",
                    &GlData::GLVec4([
                        self.texture_image_0.size().width as f32,
                        self.texture_image_0.size().height as f32,
                        self.texture_image_0.size().width as f32,
                        self.texture_image_0.size().height as f32,
                    ]),
                )
                .update_uniform(
                    program_id,
                    "g_Texture1Resolution",
                    &GlData::GLVec4([
                        self.texture_image_1.size().width as f32,
                        self.texture_image_1.size().height as f32,
                        self.texture_image_1.size().width as f32,
                        self.texture_image_1.size().height as f32,
                    ]),
                )
                .update_uniform(
                    program_id,
                    "g_Texture2Resolution",
                    &GlData::GLVec4([
                        self.texture_image_2.size().width as f32,
                        self.texture_image_2.size().height as f32,
                        self.texture_image_2.size().width as f32,
                        self.texture_image_2.size().height as f32,
                    ]),
                );

                let program_id =
                    match n.add_effect(VERTEX_SHADER_2, FRAGMENT_SHADER_2) {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Failed to add effect: {}", "error");
                            return n;
                        }
                    };

                n.update_geometry(rect);

                n
            },
        ));

        node
    }

    fn component_complete(&mut self) {
        self.set_flag(QQuickItemFlag::ItemHasContents);
    }
}
