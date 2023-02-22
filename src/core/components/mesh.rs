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

fn interlace_vecs(positions: Vec<f32>, normals: Vec<f32>, texcoords: Vec<f32>) -> Vec<f32> {
    let mut new_vertices = Vec::with_capacity(positions.len() + normals.len() + texcoords.len());

    for i in (0..positions.len() / 3) {
        let pos_index = i * 3;
        let nor_index = pos_index;
        let uv_index = i * 2;

        for j in 0..3 {
            new_vertices.push(positions[pos_index + j]);
        }

        for j in 0..3 {
            new_vertices.push(normals[nor_index + j]);
        }

        for j in 0..2 {
            new_vertices.push(texcoords[uv_index + j]);
        }
    }

    new_vertices
}

impl From<&[tobj::Model]> for Mesh {
    fn from(value: &[tobj::Model]) -> Self {
        // for now, we only care about the first one
        let mesh = match value.get(0) {
            Some(model) => model.mesh.to_owned(),
            None => tobj::Mesh::default()
        };

        Self::new(
            interlace_vecs(mesh.positions, mesh.normals, mesh.texcoords), 
            mesh.indices
        )
    }
}