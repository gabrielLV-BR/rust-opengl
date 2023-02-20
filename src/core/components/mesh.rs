use gl;

use crate::core::renderer::api::{buffer::{VertexBuffer, ElementBuffer}, object::GLObject, vao::{VertexArray, VertexAttribute}};

pub struct Mesh {
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    element_buffer: ElementBuffer
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        let vertex_array = VertexArray::new()
            .with_vertex_attributes(vec![VertexAttribute::POSITION]);

        let vertex_buffer = VertexBuffer::vertex_buffer()
            .with_data(gl::STATIC_DRAW, vertices);

        let element_buffer = ElementBuffer::element_buffer()
            .with_data(gl::STATIC_DRAW, indices);

        vertex_array.bind_with(vec![&vertex_buffer, &element_buffer]);

        Mesh {
            vertex_array,
            vertex_buffer,
            element_buffer
        }
    }
}

impl GLObject for Mesh {
    fn bind(&self) {
        
    }
    
    fn unbind(&self) {
        
    }

    fn handle(&self) -> u32 {
        self.vertex_array.handle()
    }
}

impl From<tobj::Mesh> for Mesh {
    fn from(value: tobj::Mesh) -> Self {
        let vertex_array = VertexArray::new()
            .with_vertex_attributes(vec![VertexAttribute::POSITION]);

        let vertex_buffer = VertexBuffer::vertex_buffer()
            .with_data(gl::STATIC_DRAW, value.positions);

        let element_buffer = ElementBuffer::element_buffer()
            .with_data(gl::STATIC_DRAW, value.indices);

        vertex_array.bind_with(vec![&vertex_buffer, &element_buffer]);

        Mesh { 
            vertex_array,
            vertex_buffer,
            element_buffer
        }
    }
}