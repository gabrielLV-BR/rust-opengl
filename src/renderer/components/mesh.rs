use crate::renderer::api::{buffer::Buffer, texture::Texture, vao::{VertexAttribute, VertexArray}, object::GLObject};
use bevy_ecs::prelude::Component;
use gl;

#[derive(Component, Debug)]
pub struct Mesh {
    pub vao: VertexArray,
    pub vertices: Buffer<f32>,
    pub indices: Buffer<u32>,
    pub texture_handle: Option<u32>,
}

impl Mesh {
    pub fn new(vertex_data: Vec<f32>, index_data: Vec<u32>, texture_handle: Option<u32>) -> Self {
        let mut vertices : Buffer<f32> = Buffer::new(gl::STATIC_DRAW);
        let mut indices : Buffer<u32> = Buffer::new(gl::STATIC_DRAW);
        let mut vao = VertexArray::new();

        vao.vertex_attributes(vec![
            VertexAttribute::POSITION
        ]);
        vao.bind();

        vertices.bind();
        indices.bind();

        vertices.set_data(gl::ARRAY_BUFFER, vertex_data);
        indices.set_data(gl::ELEMENT_ARRAY_BUFFER, index_data); 

        vao.unbind();

        Mesh {
            vao,
            vertices,
            indices,
            texture_handle
        }
    }
}