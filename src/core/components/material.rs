use bevy_ecs::prelude::Component;
use ultraviolet::Vec3;

#[derive(Component, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum MaterialShaderType {
    BasicMaterialShader
}

#[derive(Component)]
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