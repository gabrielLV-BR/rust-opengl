use std::path::{Path, PathBuf};

use crate::core::components::transform::Transform;

use super::{
    backend::shader::{self, Program, Shader},
    components::{
        material::{MaterialTrait, MaterialType},
        mesh::Mesh,
    },
};

pub struct RenderWorld {
    meshes: Vec<Mesh>,
    materials: Vec<Box<dyn MaterialTrait>>,
    programs: Vec<Program>,
    children: Vec<Node>,
}

impl RenderWorld {
    pub(crate) fn new() -> Self {
        RenderWorld {
            meshes: vec![],
            materials: vec![],
            programs: vec![],
            children: vec![],
        }
    }

    pub fn provide_program_for_material(&mut self, material_type: MaterialType, program: Program) {
        self.programs.insert(material_type.index(), program);
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn add_node_with(
        &mut self,
        mesh: Mesh,
        material: Box<dyn MaterialTrait>,
        transform: Transform,
    ) -> usize {
        let node = Node {
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

    pub fn add_node(&mut self, node: Node) -> usize {
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

fn load_program_with_name(name: &str) -> Program {
    let fragment_shader_path =
        Path::canonicalize(&PathBuf::from(format!("assets/shaders/{}.frag.glsl", name)))
            .expect("Shader does not exist");

    let vertex_shader_path =
        Path::canonicalize(&PathBuf::from(format!("assets/shaders/{}.vert.glsl", name)))
            .expect("Shader does not exist");

    let fragment_shader = Shader::from_file(&fragment_shader_path, shader::ShaderType::Fragment)
        .expect("Error loading frag shader");

    let vertex_shader = Shader::from_file(&vertex_shader_path, shader::ShaderType::Vertex)
        .expect("Error loading vert shader");

    Program::new(vertex_shader, fragment_shader)
}

impl Default for RenderWorld {
    fn default() -> Self {
        let colored_program = load_program_with_name("colored");
        let textured_program = load_program_with_name("textured");

        let mut render_world = RenderWorld::new();

        render_world.provide_program_for_material(MaterialType::ColoredMaterial, colored_program);
        render_world.provide_program_for_material(MaterialType::TexturedMaterial, textured_program);

        return render_world;
    }
}

#[derive(Debug)]
pub struct Node {
    mesh_handle: Option<usize>,
    material_handle: Option<usize>,
    transform: Transform,
    // children: Option<Vec<usize>>
}

impl Node {
    pub fn new() -> Self {
        Node {
            mesh_handle: None,
            material_handle: None,
            transform: Transform::identity(),
        }
    }

    pub fn with_mesh(self, mesh_handle: usize) -> Self {
        Node {
            mesh_handle: Some(mesh_handle),
            ..self
        }
    }

    pub fn with_material(self, material_handle: usize) -> Self {
        Node {
            material_handle: Some(material_handle),
            ..self
        }
    }

    pub fn get_mesh_handle(&self) -> Option<usize> {
        self.mesh_handle
    }

    pub fn get_material_handle(&self) -> Option<usize> {
        self.material_handle
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            mesh_handle: None,
            material_handle: None,
            transform: Transform::identity(),
            // children: None,
        }
    }
}
