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
    children: Option<Vec<usize>>
}

impl RenderNode {
    pub fn new() -> Self {
        RenderNode { 
            mesh_handle: None, 
            material_handle: None, 
            children: None,
        }
    }
}

impl Default for RenderNode {
    fn default() -> Self {
        RenderNode { 
            mesh_handle: None, 
            material_handle: None, 
            children: None,
        }      
    }
}