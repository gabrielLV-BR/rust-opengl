use ultraviolet::Vec3;

use crate::core::renderer::api::texture::Texture;

pub enum MaterialShaderType {
    BasicMaterialShader,
    TexturedMaterialShader
}

// Material with only a simple color

pub struct BasicMaterial {
    pub material_type: MaterialShaderType,
    pub color: Vec3
}

impl BasicMaterial {
    pub fn new(color: Vec3) -> Self {
        BasicMaterial { 
            material_type: MaterialShaderType::BasicMaterialShader, 
            color
        }
    }
}

// Material with a texture

pub struct TexturedMaterial {
    pub material_type: MaterialShaderType,
    pub texture: Texture
}

impl TexturedMaterial {
    pub fn new(texture: Texture) -> Self {
        TexturedMaterial { 
            material_type: MaterialShaderType::TexturedMaterialShader, 
            texture
        }
    }
}