use crate::core::components::transform::Transform;

use super::components::{material::MaterialTrait, mesh::Mesh};

pub struct RenderWorld {
    meshes: Vec<Mesh>,
    materials: Vec<&'static dyn MaterialTrait>,
    children: Vec<RenderNode>,
}

impl RenderWorld {
    pub fn new() -> Self {
        RenderWorld {
            meshes: vec![],
            materials: vec![],
            children: vec![],
        }
    }

    pub fn nodes(&self) -> &Vec<RenderNode> {
        &self.children
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        self.meshes.push(mesh);
        self.meshes.len() - 1
    }

    pub fn add_material(&mut self, material: &'static dyn MaterialTrait) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn add_node(&mut self, node: RenderNode) -> usize {
        self.children.push(node);
        self.children.len() - 1
    }

    pub fn get_material(&self, handle: usize) -> Option<&&dyn MaterialTrait> {
        self.materials.get(handle)
    }

    pub fn get_mesh(&self, handle: usize) -> Option<&Mesh> {
        self.meshes.get(handle)
    }
}

pub struct RenderNode {
    mesh_handle: Option<usize>,
    material_handle: Option<usize>,
    transform: Transform,
    // children: Option<Vec<usize>>
}

impl RenderNode {
    pub fn new() -> Self {
        RenderNode {
            mesh_handle: None,
            material_handle: None,
            transform: Transform::identity(),
        }
    }

    pub fn mesh_handle(&self) -> Option<usize> {
        self.mesh_handle
    }

    pub fn material_handle(&self) -> Option<usize> {
        self.material_handle
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}

impl Default for RenderNode {
    fn default() -> Self {
        RenderNode {
            mesh_handle: None,
            material_handle: None,
            transform: Transform::identity(),
            // children: None,
        }
    }
}
