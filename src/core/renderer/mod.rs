pub mod api;
pub mod render_stages;

use glfw::{Glfw, Window};
use bevy_ecs as bevy;
use bevy::prelude::*;

use render_stages::*;
use crate::core::renderer::api::{shader::*, object::GLObject};
use crate::core::components::mesh::Mesh;

pub struct Renderer {
    world: bevy::world::World,
    init_schedule: bevy::schedule::Schedule,
    update_schedule: bevy::schedule::Schedule,
}

pub fn setup_test_object(
    mut commands: Commands
) {
    let vertices = vec![
        -0.5f32, -0.5, 0.0,
        0.0, 0.5, 0.0,
        0.5, -0.5, 0.0
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2
    ];

    let vertex_shader = Shader::from_file(
        "assets/shaders/debug.vert", 
        ShaderType::Vertex
    ).unwrap();

    let fragment_shader = Shader::from_file(
        "assets/shaders/debug.frag", 
        ShaderType::Fragment
    ).unwrap();

    let program = Program::new(vertex_shader, fragment_shader);

    commands.spawn((Mesh::new(vertices, indices), program));
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

        let world = bevy::world::World::new();
        let init_schedule = bevy::schedule::Schedule::default();
        let update_schedule = bevy::schedule::Schedule::default();

        Renderer { 
            world, 
            init_schedule,
            update_schedule,
        }
    }

    pub fn setup(&mut self) {
        self.init_schedule.add_stage(InitStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(PrepareStage, SystemStage::parallel());
        self.update_schedule.add_stage(RenderStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(CleanupStage, SystemStage::parallel());
    
        self.init_schedule.add_system_to_stage(InitStage, setup_test_object);
        self.update_schedule.add_system_to_stage(RenderStage, Self::render);

        self.init_schedule.run_once(&mut self.world);
    }

    pub fn update(&mut self, _delta: f32) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.update_schedule.run_once(&mut self.world);
    }

    pub fn render(
        q: Query<(&Mesh, &Program)>
    ) {
        q.for_each(|(mesh, program)| {
            mesh.bind();
            program.bind();
    
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.elements().count() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null()
                );
            }

            mesh.unbind();
            program.unbind();
        });
    }

    pub fn dispose(self) {
        todo!()
    }
}