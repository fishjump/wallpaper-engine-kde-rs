pub mod gl_data;

use anyhow::anyhow;
use anyhow::{Ok, Result};
use cpp::cpp;
use qmetaobject::scenegraph::SGNode;
use qttypes::QRectF;
use std::ffi::CString;
use std::os::raw::c_void;

use gl_data::GlData;

cpp! {{
    #include "src/scene_node/scene_object.hpp"
}}

pub struct SceneObject {}

pub trait SceneObjectTrait {
    fn new(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<&mut Self>;

    fn update_uniform(&mut self, name: &str, value: &GlData) -> &mut Self;
    fn update_geometry(&mut self, rect: QRectF) -> &mut Self;
}

impl SceneObjectTrait for SGNode<SceneObject> {
    fn new(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<&mut Self> {
        if !self.raw.is_null() {
            return Ok(self);
        }

        let vertex_shader = CString::new(vertex_shader)?.into_raw();
        let fragment_shader = CString::new(fragment_shader)?.into_raw();

        self.raw = cpp!(unsafe [vertex_shader as "const char *", fragment_shader as "const char *"] -> *mut c_void as "void *" {
            auto raw = new SceneObject{};
            raw->set_vertex_shader(vertex_shader);
            raw->set_fragment_shader(fragment_shader);

            if (!raw->initialize()) {
                return nullptr;
            }

            return raw;
        });

        if self.raw.is_null() {
            Err(anyhow!("Failed to create SceneObject"))
        } else {
            Ok(self)
        }
    }

    fn update_uniform(&mut self, name: &str, value: &GlData) -> &mut Self {
        if self.raw.is_null() {
            return self;
        }

        let name = CString::new(name).unwrap().into_raw();
        let raw = self.raw;

        match value {
            GlData::GLUint(value) => {
                let value = value.clone();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "GLuint"] {
                    raw->update_uniform(name, value);
                })
            }
            GlData::GLFloat(value) => {
                let value = value.clone();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "GLfloat"] {
                    raw->update_uniform(name, value);
                })
            }
            GlData::GLVec2(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "float *"] {
                    raw->update_uniform(name, QVector2D{value[0], value[1]});
                })
            }
            GlData::GLVec3(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "float *"] {
                    raw->update_uniform(name, QVector3D{value[0], value[1], value[2]});
                })
            }
            GlData::GLVec4(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "float *"] {
                    raw->update_uniform(name, QVector4D{value[0], value[1], value[2], value[3]});
                })
            }
            GlData::GLMat2x2(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "float *"] {
                    raw->update_uniform(name, QMatrix2x2{new float[] {value[0], value[1], value[2], value[3]}});
                })
            }
            GlData::GLMat3x3(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "float *"] {
                    raw->update_uniform(name, QMatrix3x3{new float[] {value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7], value[8]}});
                })
            }
            GlData::GLMat4x4(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", value as "float *"] {
                    raw->update_uniform(name, QMatrix4x4{new float[] {value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7], value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15]}});
                })
            }
            GlData::GLSampler2D(texture, id) => {
                let texture = texture.clone();
                let id = id.clone();
                cpp!(unsafe [raw as "SceneObject *", name as "const char *", texture as "QSGTexture *", id as "GLuint"] {
                    raw->update_uniform(name, Sampler2D{texture, id});
                })
            }
        }

        self
    }

    fn update_geometry(&mut self, rect: QRectF) -> &mut Self {
        if self.raw.is_null() {
            return self;
        }

        let raw = self.raw;
        cpp! (unsafe [raw as "SceneObject *", rect as "QRectF"] {
            raw->update_geometry(rect);
        });

        self
    }
}
