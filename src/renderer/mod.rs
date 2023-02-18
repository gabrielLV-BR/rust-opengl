pub mod api;
pub mod components;
pub mod render_stages;

use glfw::{Glfw, Window};
use bevy_ecs::{world::World, schedule::{Schedule, SystemStage}, prelude::{Query, Component}, system::{Res, Command, Commands, ResMut}};
use render_stages::*;

use crate::server::{texture_server::TextureServer, AssetServerTrait};

use self::{
    api::{shader::{Program, UniformType, Shader, ShaderType}, object::GLObject}, 
    components::{camera::Camera, transform::{Transform, self}, mesh::Mesh}
};

pub struct Renderer {
    world: World,
    init_schedule: Schedule,
    update_schedule: Schedule,
}

#[derive(Component)]
pub struct Monkey;

fn setup_monkey(
    mut command: Commands,
    mut texture_server: ResMut<TextureServer>
) {

    let texture = texture_server.load("assets/textures/obama.jpg").unwrap();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
        0.5, -0.5, 0.0
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2
    ];

    // mesh
    let mesh = Mesh::new(
        vertices, 
        indices, 
        Some(texture.handle)
    );
    // program
    let vertex_shader = Shader::from_file(
        "assets/shaders/monke.vert", 
        ShaderType::Vertex
    ).unwrap();
    let fragment_shader = Shader::from_file(
        "assets/shaders/monke.frag", 
        ShaderType::Fragment
    ).unwrap();
    let program = Program::new(vertex_shader, fragment_shader);

    command.spawn(Monkey)
        .insert(mesh)
        .insert(program);
}

impl Renderer {
    pub fn new(glfw: &mut Glfw, window: &Window) -> Self {
        glfw.make_context_current(Some(&window));

        gl::load_with(
            |s| glfw.get_proc_address_raw(s)
        );
        
        unsafe {
            let (width, height) = window.get_size();
            gl::Viewport(0, 0, width, height);
        }

        let world = World::new();
        let init_schedule = Schedule::default();
        let update_schedule = Schedule::default();

        let mut renderer = Renderer { 
            world, 
            init_schedule,
            update_schedule 
        };

        Self::setup(&mut renderer);

        renderer
    }

    pub fn setup(&mut self) {
        self.init_schedule.add_stage(InitStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(PrepareStage, SystemStage::parallel());
        self.update_schedule.add_stage(RenderStage, SystemStage::parallel());
        self.update_schedule.add_stage(CleanupStage, SystemStage::parallel());
    
        self.world.insert_resource(TextureServer::init());

        self.init_schedule.add_system_to_stage(InitStage, setup_monkey);
        self.update_schedule.add_system_to_stage(RenderStage, Self::render);

        self.init_schedule.run_once(&mut self.world);
    }

    pub fn update(&mut self, delta: f32) {
        self.update_schedule.run_once(&mut self.world);

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render(
        q: Query<(&Mesh, &Program)>
    ) {
        // let model = camera.as_ref;

        println!("{:?}", q);

        for (mesh, program) in q.iter() {
            mesh.vao.bind();
            mesh.vertices.bind();
            mesh.indices.bind();
            program.bind();

            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, mesh.texture_handle.expect("TOME NO CU"));
                // program.set_uniform("uTexture", UniformType::Int(0));
    
                // program.set_uniform("uModelMatrix", UniformType::Matrix4(&model));
                // program.set_uniform("uViewMatrix", UniformType::Matrix4(&view));
                // program.set_uniform("uProjMatrix", UniformType::Matrix4(&proj));
    
                gl::DrawElements(
                    gl::TRIANGLES, 
                    mesh.indices.count() as i32,
                    gl::UNSIGNED_INT, 
                    0 as *const _
                );

                gl::BindTexture(gl::TEXTURE_2D, 0);
            }

            mesh.vertices.unbind();
            mesh.indices.unbind();
        }
    }

    pub fn dispose(self) {

    }
}