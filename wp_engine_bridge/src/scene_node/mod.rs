pub mod gl_data;

use std::ffi::{CStr, CString};
use std::os::raw::c_void;

use anyhow::{anyhow, Ok, Result};
use cpp::cpp;
use cstr::cstr;
use gl_data::GlData;
use qmetaobject::scenegraph::SGNode;
use qttypes::QRectF;

cpp! {{
    #include "src/scene_node/scene_object.hpp"
    #include "src/scene_node/scene_object_node.hpp"
}}

pub struct SceneObjectNode {}

pub trait SceneObjectTrait {
    fn new(&mut self) -> Result<&mut Self>;

    fn update_uniform(
        &mut self,
        program_id: u32,
        name: &str,
        value: &GlData,
    ) -> &mut Self;

    fn update_geometry(&mut self, rect: QRectF) -> &mut Self;

    fn add_effect(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<u32>;
}

impl SceneObjectTrait for SGNode<SceneObjectNode> {
    fn new(&mut self) -> Result<&mut Self> {
        if !self.raw.is_null() {
            return Ok(self);
        }

        self.raw = cpp!(unsafe [] -> *mut c_void as "void *" {
            auto raw = new SceneObjectNode{};

            return raw;
        });

        if self.raw.is_null() {
            Err(anyhow!("Failed to create SceneObjectNode"))
        } else {
            Ok(self)
        }
    }

    fn update_uniform(
        &mut self,
        program_id: u32,
        name: &str,
        value: &GlData,
    ) -> &mut Self {
        if self.raw.is_null() {
            return self;
        }

        let name = CString::new(name).unwrap().into_raw();
        let raw = self.raw;

        match value {
            GlData::GLUint(value) => {
                let value = value.clone();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "GLuint"] {
                    raw->update_uniform(program_id, name, value);
                })
            }
            GlData::GLFloat(value) => {
                let value = value.clone();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "GLfloat"] {
                    raw->update_uniform(program_id, name, value);
                })
            }
            GlData::GLVec2(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "float *"] {
                    raw->update_uniform(program_id, name, QVector2D{value[0], value[1]});
                })
            }
            GlData::GLVec3(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "float *"] {
                    raw->update_uniform(program_id, name, QVector3D{value[0], value[1], value[2]});
                })
            }
            GlData::GLVec4(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "float *"] {
                    raw->update_uniform(program_id, name, QVector4D{value[0], value[1], value[2], value[3]});
                })
            }
            GlData::GLMat2x2(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "float *"] {
                    raw->update_uniform(program_id, name, QMatrix2x2{new float[] {value[0], value[1], value[2], value[3]}});
                })
            }
            GlData::GLMat3x3(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "float *"] {
                    raw->update_uniform(program_id, name, QMatrix3x3{new float[] {value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7], value[8]}});
                })
            }
            GlData::GLMat4x4(value) => {
                let value = value.as_ptr();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", value as "float *"] {
                    raw->update_uniform(program_id, name, QMatrix4x4{new float[] {value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7], value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15]}});
                })
            }
            GlData::GLSampler2D(texture, id) => {
                let texture = texture.clone();
                let id = id.clone();
                cpp!(unsafe [raw as "SceneObjectNode *", program_id as "GLuint", name as "const char *", texture as "QSGTexture *", id as "GLuint"] {
                    raw->update_uniform(program_id, name, Sampler2D{texture, id});
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
        cpp! (unsafe [raw as "SceneObjectNode *", rect as "QRectF"] {
            raw->update_geometry(rect);
        });

        self
    }

    fn add_effect(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<u32> {
        if self.raw.is_null() {
            return Err(anyhow!("SceneObjectNode is null"));
        }

        let vertex_shader: *mut i8 =
            CString::new(vertex_shader).unwrap().into_raw();
        let fragment_shader: *mut i8 =
            CString::new(fragment_shader).unwrap().into_raw();

        let raw = self.raw;
        let result = cpp!(unsafe [raw as "SceneObjectNode *", vertex_shader as "const char *", fragment_shader as "const char *"] -> u32 as "GLuint" {
            return raw->add_effect(vertex_shader, fragment_shader);
        });

        Ok(result)
    }
}
