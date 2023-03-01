use std::collections::HashMap;

use bevy_ecs::system::Resource;

use crate::core::{
    renderer::api::shader::{Program, Shader}, 
    components::material::MaterialShaderType
};

use super::AssetServer;

#[derive(Resource)]
pub struct ProgramServer {
    map: HashMap<MaterialShaderType, Program>
}

impl ProgramServer {
    pub fn new() -> Self {
        use crate::core::renderer::api::shader::ShaderType::*;

        let basic_program = Program::new(
            MaterialShaderType::BasicMaterialShader,
            Shader::from_file("assets/shaders/basic.vert", Vertex).unwrap(),
            Shader::from_file("assets/shaders/basic.frag", Fragment).unwrap(),
        );

        let texture_program = Program::new(
            MaterialShaderType::TexturedMaterialShader,
            Shader::from_file("assets/shaders/textured.vert", Vertex).unwrap(),
            Shader::from_file("assets/shaders/textured.frag", Fragment).unwrap(),
        );
            

        let mut program_server = ProgramServer { map: HashMap::new() };

        program_server.store(basic_program);
        program_server.store(texture_program);

        program_server
    }
}

impl AssetServer<Program> for ProgramServer {
    type Handle = MaterialShaderType;

    fn get(&self, key: Self::Handle) -> Option<&Program> {
        self.map.get(&key)
    }

    fn store(&mut self, item: Program) -> Self::Handle {
        let handle = item.material_type();
        self.map.insert(handle, item);
        handle
    }
}