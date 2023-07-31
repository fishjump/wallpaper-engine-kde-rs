use std::os::raw::c_float;

use cpp::cpp;

use crate::brige::scenegraph_ext::geometry_node::QSGGeometry;
use crate::brige::scenegraph_ext::material::QSGMaterial;

cpp! {{
    #include "src/brige/shader/custom_shader.cpp"
}}

pub enum CustomMaterialShader {}

impl CustomMaterialShader {
    pub fn new() -> *mut QSGMaterial {
        cpp!(unsafe [] -> *mut QSGMaterial as "QSGSimpleMaterial<MaterialData> *" {
            return CustomMaterialShader::createMaterial();
        })
    }
}

pub fn create_qsggeometry() -> *mut QSGGeometry {
    cpp!(unsafe [] -> *mut QSGGeometry as "QSGGeometry*" {
        return createQSGGeometry();
    })
}

pub fn update_position(
    geometry: *mut QSGGeometry,
    x1: c_float,
    y1: c_float,
    x2: c_float,
    y2: c_float,
    x3: c_float,
    y3: c_float,
) {
    cpp!(unsafe [geometry as "QSGGeometry*", x1 as "float", y1 as "float", x2 as "float", y2 as "float", x3 as "float", y3 as "float"] {
        updatePosition(geometry, x1, y1, x2, y2, x3, y3);
    });
}
