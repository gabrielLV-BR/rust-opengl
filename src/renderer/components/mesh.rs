use crate::renderer::api::{buffer::Buffer, texture::Texture};
use bevy_ecs::prelude::Component;
use gl;

#[derive(Component)]
pub struct Mesh {
    pub vertices: Buffer<f32>,
    pub indices: Buffer<u32>,
    pub texture_handle: Option<u32>,
}

impl Mesh {
    pub fn new(vertex_data: Vec<f32>, index_data: Vec<u32>, texture_handle: Option<u32>) -> Self {
        let mut vertices : Buffer<f32> = Buffer::new(gl::STATIC_DRAW);
        let mut indices : Buffer<u32> = Buffer::new(gl::STATIC_DRAW);

        vertices.set_data(gl::ARRAY_BUFFER, vertex_data);
        indices.set_data(gl::ELEMENT_ARRAY_BUFFER, index_data); 
;        
        Mesh {
            vertices,
            indices,
            texture_handle
        }
    }
}