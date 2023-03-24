use gl::ProgramBinary;
use ultraviolet::Vec3;

use crate::core::renderer::backend::gl::{
    shader::{Program, Shader},
    texture::Texture,
};

pub trait MaterialTrait {
    fn prepare(&self);
    fn bind(&self);
}
// Material with only a simple color

pub struct BasicMaterial {
    program: Program,
    pub color: Vec3,
}

impl BasicMaterial {
    pub fn new(color: Vec3) -> Self {
        use crate::core::renderer::backend::gl::shader::ShaderType;
        let basic_material_fragment_shader =
            Shader::from_file("../../../../../../shaders/basic.frag", ShaderType::Fragment)
                .expect("Error loading fragment shader from file");

        let basic_material_vertex_shader =
            Shader::from_file("../../../shaders/basic.vert", ShaderType::Vertex)
                .expect("Error loading vertex shader from file");

        let program = Program::new(basic_material_vertex_shader, basic_material_fragment_shader);

        BasicMaterial { program, color }
    }
}

impl MaterialTrait for BasicMaterial {
    fn bind(&self) {}

    fn prepare(&self) {}
}
