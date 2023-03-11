use crate::core::renderer::api::{
    buffer::{VertexBuffer, ElementBuffer}, 
    object::GLObject, 
    vao::{VertexArray, VertexAttribute}, 
    vertex::Vertex
};

#[allow(dead_code)]
pub struct Mesh {
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    element_buffer: ElementBuffer
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, vertex_attributes: Vec<VertexAttribute>) -> Self {

        let vertex_buffer = VertexBuffer::vertex_buffer()
            .with_data(gl::STATIC_DRAW, vertices);

        let element_buffer = ElementBuffer::element_buffer()
            .with_data(gl::STATIC_DRAW, indices);

        let vertex_array = VertexArray::new()
            .bound_with(vec![&vertex_buffer, &element_buffer])
            .with_vertex_attributes(vertex_attributes)
            .unbound();

        Mesh {
            vertex_array,
            vertex_buffer,
            element_buffer
        }
    }

    pub fn _vertices(&self) -> &VertexBuffer {
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

fn _interlace_vecs(positions: Vec<f32>, normals: Vec<f32>, texcoords: Vec<f32>) -> Vec<Vertex> {
    assert!(positions.len() == normals.len());

    let new_vertices: Vec<Vertex> = Vec::with_capacity(positions.len());

    let vertex_count = if positions.len() > 0 { 3 } else { 0 };
    let normal_count = if normals.len()   > 0 { 3 } else { 0 };
    let uv_count     = if texcoords.len() > 0 { 2 } else { 0 };

    for i in 0..(positions.len() / 3 - 1) {
        let pos_index = i * 3;
        let nor_index = pos_index;
        let uv_index = i * 2;

        let mut v: Vec<f32> = Vec::new();
        let mut n: Vec<f32> = Vec::new();
        let mut u: Vec<f32> = Vec::new();

        for j in 0..vertex_count {
            v.push(positions[pos_index + j]);
        }

        for j in 0..normal_count {
            n.push(normals[nor_index + j]);
        }

        for j in 0..uv_count {
            u.push(texcoords[uv_index + j]);
        }
    }

    new_vertices
}