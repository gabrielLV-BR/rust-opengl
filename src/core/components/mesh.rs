use crate::core::renderer::api::{
    buffer::Buffer, 
    vao::{VertexAttribute, VertexArray}, 
    object::GLObject
};
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
        let mut vertices : Buffer<f32> = Buffer::new(gl::ARRAY_BUFFER);
        let mut indices : Buffer<u32> = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
        let mut vao = VertexArray::new();

        vao.set_vertex_attributes(vec![
            VertexAttribute::POSITION
        ]);
        vao.bind();

        vertices.bind();
        indices.bind();

        vertices.set_data(gl::STATIC_DRAW, vertex_data);
        indices.set_data(gl::STATIC_DRAW, index_data); 

        vao.unbind();
        vertices.unbind();
        indices.unbind();

        Mesh {
            vao,
            vertices,
            indices,
            texture_handle
        }
    }
}