use crate::core::components::transform::Transform;

use super::components::{mesh::Mesh, material::MaterialTrait};

pub struct RenderWorld {
    meshes: Vec<Mesh>,
    materials: Vec<&'static dyn MaterialTrait>,
    root: RenderNode,
}

impl RenderWorld {
    pub fn new() -> Self {
        RenderWorld { 
            meshes: vec![], 
            materials: vec![], 
            root: RenderNode::default()  
        }
    }
}


pub struct RenderNode {
    mesh_handle: Option<usize>,
    material_handle: Option<usize>,
    transform: Transform,
    children: Option<Vec<usize>>
}

impl RenderNode {
}

impl Default for RenderNode {
    fn default() -> Self {
        RenderNode { 
            mesh_handle: None, 
            material_handle: None,
            transform: Transform::identity(),
            children: None,
        }      
    }
}