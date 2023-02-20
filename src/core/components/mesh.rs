use crate::core::renderer::api::{
    buffer::{Buffer, VertexBuffer, ElementBuffer}, 
    vao::{VertexAttribute, VertexArray}, 
    object::GLObject
};
use bevy_ecs::prelude::Component;
use gl;
use tobj::Model;

#[derive(Component, Debug)]
pub struct MeshData {
    pub vao: VertexArray,
    pub vertices: Buffer<f32>,
    pub indices: Buffer<u32>,
}

impl MeshData {
    pub fn new(vertex_data: Vec<f32>, index_data: Vec<u32>) -> Self {
        let vao = VertexArray::new();
        //     .with_vertex_attributes(vec![
        //     VertexAttribute::POSITION
        // ]);
        vao.bind();

        let mut vertices = 
            VertexBuffer::new(gl::ARRAY_BUFFER);        
        vertices.set_data(gl::STATIC_DRAW, vertex_data);

        let mut indices = 
            ElementBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
        indices.set_data(gl::STATIC_DRAW, index_data);

        vertices.bind();
        indices.bind();

        MeshData {
            vao,
            vertices,
            indices
        }
    }

    pub fn handle(&self) -> u32 {
        self.vao.handle
    }

    pub fn from_model_list(models: Vec<Model>) -> MeshData {
        todo!()
    }
}

#[derive(Component, Debug)]
pub struct MeshInstance {
    pub mesh_handle: u32,
    pub element_count: usize,
    pub texture_handle: Option<u32>
}

impl MeshInstance {
    pub fn new(mesh: &MeshData) -> Self {
        Self {
            mesh_handle: mesh.vao.handle,
            element_count: mesh.indices.count(),
            texture_handle: None
        }
    }

    pub fn with_texture_handle(self, texture_handle: u32) -> Self {
        Self {
            texture_handle: Some(texture_handle),
            ..self
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.mesh_handle);
        
            if let Some(texture_handle) = self.texture_handle {
                gl::BindTexture(gl::TEXTURE_2D, texture_handle);
            }
        }
    }
}
