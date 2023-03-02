use bevy_ecs::prelude::Component;

use crate::core::renderer::api::{buffer::{VertexBuffer, ElementBuffer}, object::GLObject, vao::{VertexArray, VertexAttribute}};

#[derive(Component, Debug)]
pub struct Mesh {
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    element_buffer: ElementBuffer
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>, vertex_attributes: Vec<VertexAttribute>) -> Self {

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

    assert!(positions.len() == normals.len());

    let mut new_vertices = Vec::with_capacity(positions.len() + normals.len() + texcoords.len());

    let vertex_count = if positions.len() > 0 { 3 } else { 0 };
    let normal_count = if normals.len()   > 0 { 3 } else { 0 };
    let uv_count     = if texcoords.len() > 0 { 2 } else { 0 };

    for i in 0..(positions.len() / 3 - 1) {
        let pos_index = i * 3;
        let nor_index = pos_index;
        let uv_index = i * 2;

        for j in 0..vertex_count {
            new_vertices.push(positions[pos_index + j]);
        }

        for j in 0..normal_count {
            new_vertices.push(normals[nor_index + j]);
        }

        for j in 0..uv_count {
            new_vertices.push(texcoords[uv_index + j]);
        }
    }

    new_vertices
}


pub struct MeshBuilder {
    positions: Option<Vec<f32>>,
    indices: Option<Vec<u32>>,
    normals: Option<Vec<f32>>,
    uv: Option<Vec<f32>>,
    attributes: Vec<VertexAttribute>
}

impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder { 
            positions: None, 
            indices: None, 
            normals: None, 
            uv: None,
            attributes: Vec::new()
        }
    }

    pub fn build(self) -> Mesh {
        let positions= self.positions.unwrap_or(vec![]);
        let indices  = self.indices.unwrap_or(vec![]);
        let normals  = self.normals.unwrap_or(vec![]);
        let uv       = self.uv.unwrap_or(vec![]);

        let vertices = interlace_vecs(positions, normals, uv);

        let mut attributes = self.attributes;
        attributes.sort_by_key(|a| a.order);

        Mesh::new(vertices, indices, attributes)
    }

    pub fn with_positions(self, positions: Vec<f32>) -> MeshBuilder {
        let mut attributes = self.attributes;
        attributes.push(VertexAttribute::POSITION);
        
        Self {
            positions: Some(positions),
            attributes,
            ..self
        }
    }

    pub fn with_indices(self, indices: Vec<u32>) -> MeshBuilder {
        Self {
            indices: Some(indices),
            ..self
        }
    }

    pub fn with_normals(self, normals: Vec<f32>) -> MeshBuilder {
        let mut attributes = self.attributes;
        attributes.push(VertexAttribute::NORMAL);

        Self {
            normals: Some(normals),
            attributes,
            ..self
        }
    }

    pub fn with_uv(self, uv: Vec<f32>) -> MeshBuilder {
        let mut attributes = self.attributes;
        attributes.push(VertexAttribute::UV);

        Self {
            uv: Some(uv),
            attributes,
            ..self
        }
    }
}