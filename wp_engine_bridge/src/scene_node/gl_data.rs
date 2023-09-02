use crate::qt_ext::texture::Texture;

pub enum GlData {
    GLUint(u32),
    GLFloat(f32),
    GLVec2([f32; 2]),
    GLVec3([f32; 3]),
    GLVec4([f32; 4]),
    GLMat2x2([[f32; 2]; 2]),
    GLMat3x3([[f32; 3]; 3]),
    GLMat4x4([[f32; 4]; 4]),
    GLSampler2D(*mut Texture, u32),
}
