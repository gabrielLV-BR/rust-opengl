use ultraviolet::Vec3;

use super::texture::Texture;
use crate::core::renderer::backend::shader::{Program, UniformType};

#[derive(Debug)]
pub enum MaterialType {
    ColoredMaterial = 0,
    TexturedMaterial = 1,
}

impl MaterialType {
    pub fn index(self) -> usize {
        self as usize
    }
}

pub trait MaterialTrait {
    fn bind(&self, program: &Program);
    fn material_type(&self) -> MaterialType;
}

// Material with only a simple color

pub struct ColoredMaterial {
    pub color: Vec3,
}

impl ColoredMaterial {
    pub fn new(color: Vec3) -> Self {
        ColoredMaterial { color }
    }
}

impl MaterialTrait for ColoredMaterial {
    fn bind(&self, program: &Program) {
        // Program must already be bound
        program.set_uniform(Program::COLOR_UNIFORM, UniformType::Vec3(&self.color));
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::ColoredMaterial
    }
}

// Material with a diffuse texture

pub struct TexturedMaterial {
    pub color: Vec3,
    pub texture: Texture,
}

impl TexturedMaterial {
    pub fn new(color: Vec3, texture: Texture) -> Self {
        TexturedMaterial { color, texture }
    }
}

impl MaterialTrait for TexturedMaterial {
    fn bind(&self, program: &Program) {
        // Program must already be bound
        program.set_uniform(Program::COLOR_UNIFORM, UniformType::Vec3(&self.color));
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::TexturedMaterial
    }
}
