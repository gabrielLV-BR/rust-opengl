use crate::core::components::transform::Transform;

use super::{
    backend::gl::shader::{self, Program, Shader},
    components::{
        material::{MaterialTrait, MaterialType},
        mesh::Mesh,
    },
};

pub struct RenderWorld {
    meshes: Vec<Mesh>,
    materials: Vec<Box<dyn MaterialTrait>>,
    programs: Vec<Program>,
    children: Vec<RenderNode>,
}

impl RenderWorld {
    pub fn new() -> Self {
        RenderWorld {
            meshes: vec![],
            materials: vec![],
            programs: vec![],
            children: vec![],
        }
    }

    fn load_programs(shader_names: Vec<String>) -> Vec<Program> {
        shader_names
            .iter()
            .map(|shader_name| {
                let fragment_shader = Shader::from_file(
                    format!("shaders/{}.frag", shader_name).as_str(),
                    shader::ShaderType::Fragment,
                )
                .expect("Error loading frag shader");

                let vertex_shader = Shader::from_file(
                    format!("shaders/{}.vert", shader_name).as_str(),
                    shader::ShaderType::Vertex,
                )
                .expect("Error loading vert shader");

                Program::new(vertex_shader, fragment_shader)
            })
            .collect()
    }

    pub fn provide_program_for_material(&mut self, material_type: MaterialType, program: Program) {
        self.programs.insert(material_type.index(), program);
    }

    pub fn nodes(&self) -> &Vec<RenderNode> {
        &self.children
    }

    pub fn add_node_with(
        &mut self,
        mesh: Mesh,
        material: Box<dyn MaterialTrait>,
        transform: Transform,
    ) -> usize {
        let node = RenderNode {
            mesh_handle: Some(self.add_mesh(mesh)),
            material_handle: Some(self.add_material(material)),
            transform,
        };
        self.add_node(node)
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        self.meshes.push(mesh);
        self.meshes.len() - 1
    }

    pub fn add_material(&mut self, material: Box<dyn MaterialTrait>) -> usize {
        assert!(self.is_shader_loaded_for_material(material.as_ref()));
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn add_node(&mut self, node: RenderNode) -> usize {
        self.children.push(node);
        self.children.len() - 1
    }

    pub fn get_material(&self, handle: usize) -> Option<&Box<dyn MaterialTrait>> {
        self.materials.get(handle)
    }

    pub fn get_mesh(&self, handle: usize) -> Option<&Mesh> {
        self.meshes.get(handle)
    }

    pub fn get_shader(&self, material: &dyn MaterialTrait) -> Option<&Program> {
        self.programs.get(material.material_type().index())
    }

    //

    fn is_shader_loaded_for_material(&self, material: &dyn MaterialTrait) -> bool {
        let index = material.material_type().index();
        (0..self.programs.len()).contains(&index)
    }
}

impl Default for RenderWorld {
    fn default() -> Self {
        let available_shaders: Vec<String> = vec!["basic".into()];

        RenderWorld {
            meshes: vec![],
            materials: vec![],
            programs: Self::load_programs(available_shaders),
            children: vec![],
        }
    }
}

#[derive(Debug)]
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

    pub fn with_mesh(self, mesh_handle: usize) -> Self {
        RenderNode {
            mesh_handle: Some(mesh_handle),
            ..self
        }
    }

    pub fn with_material(self, material_handle: usize) -> Self {
        RenderNode {
            material_handle: Some(material_handle),
            ..self
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
