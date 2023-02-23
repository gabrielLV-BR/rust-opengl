use bevy_ecs::prelude::Component;
use ultraviolet::Vec3;

use crate::core::renderer::api::{object::GLObject, texture::Texture, shader::Program};

#[derive(Component, Debug)]
pub struct Material {
    program_handle: u32,
    textures: Vec<Texture>,
    diffuse_color: Vec3
}

impl Material {
    pub fn with_program(self, program: &Program) -> Self {
        Material { 
            program_handle: program.handle(),
            ..self
        }
    }

    pub fn with_textures(self, textures: Vec<Texture>) -> Self {
        Material {
            textures,
            ..self
        }
    }

    pub fn set_diffuse_color(&mut self, color: Vec3) {
        self.diffuse_color = color;
    }
}

impl Default for Material {
    fn default() -> Self {
        Material { 
            program_handle: 0, 
            textures: vec![], 
            diffuse_color: Vec3::one() 
        }
    }
}

impl From<&[tobj::Material]> for Material {
    fn from(value: &[tobj::Material]) -> Self {
        // for now, we only care about the first material
        let material = match value.get(0) {
            Some(material) => material.to_owned(),
            None => tobj::Material::default()
        };

        println!("{}", material.diffuse_texture);

        Material::default()
    }
}

impl GLObject for Material {
    fn bind(&self) {
        
    }

    fn unbind(&self) {
        
    }

    fn handle(&self) -> u32 {
        0
    }
}