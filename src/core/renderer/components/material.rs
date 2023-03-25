use ultraviolet::Vec3;

use crate::core::renderer::backend::gl::shader::{Program, UniformType};

pub enum MaterialType {
    BasicMaterial = 0,
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

pub struct BasicMaterial {
    pub color: Vec3,
}

impl BasicMaterial {
    pub fn new(color: Vec3) -> Self {
        BasicMaterial { color }
    }
}

impl MaterialTrait for BasicMaterial {
    fn bind(&self, program: &Program) {
        // Program must already be bound
        program.set_uniform(Program::COLOR_UNIFORM, UniformType::Vec3(&self.color));
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::BasicMaterial
    }
}
