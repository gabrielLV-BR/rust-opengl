use bevy_ecs::prelude::Component;

use crate::core::renderer::api::{buffer::{VertexBuffer, ElementBuffer}, object::GLObject, vao::{VertexArray, VertexAttribute}};

#[derive(Component, Debug)]
pub struct Mesh {
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    element_buffer: ElementBuffer
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {

        let vertex_buffer = VertexBuffer::vertex_buffer()
            .with_data(gl::STATIC_DRAW, vertices);

        let element_buffer = ElementBuffer::element_buffer()
            .with_data(gl::STATIC_DRAW, indices);

        let vertex_array = VertexArray::new()
            .bound_with(vec![&vertex_buffer, &element_buffer])
            .with_vertex_attributes(vec![VertexAttribute::POSITION])
            .unbound();

        Mesh {
            vertex_array,
            vertex_buffer,
            element_buffer
        }
    }

    pub fn vertices(&self) -> &VertexBuffer {
        &self.vertex_buffer
    }

    pub fn elements(&self) -> &ElementBuffer {
        &self.element_buffer
    }
}

impl GLObject for Mesh {
    fn bind(&self) {
        self.vertex_array.bind();
    }
    
    fn unbind(&self) {
        self.vertex_array.unbind();
    }

    fn handle(&self) -> u32 {
        self.vertex_array.handle()
    }
}

impl From<tobj::Mesh> for Mesh {
    fn from(value: tobj::Mesh) -> Self {
        Mesh::new(value.positions, value.indices)
    }
}