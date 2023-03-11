use ultraviolet::{Vec3, Vec2};

use crate::core::renderer::api::{vertex::Vertex, vao::VertexAttribute};

use super::mesh::Mesh;

pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl From<tobj::Model> for Model {
    fn from(model: tobj::Model) -> Self {
        let mut vertices : Vec<Vertex> = vec![];
        let mut indices  : Vec<u32>    = vec![];

        for _ in &model.mesh.indices {
            let vertex = Vertex {
                position: Vec3::zero(),
                normal: Vec3::zero(),
                uv: Vec2::zero(),
            };

            vertices.push(vertex);
            indices.push(indices.len() as u32);
        }

        Model {
            meshes: vec![
                Mesh::new(vertices, indices, vec![
                    VertexAttribute::POSITION,
                ])
            ]
        }
    }
}
