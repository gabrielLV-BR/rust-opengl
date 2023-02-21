pub mod api;
pub mod render_stages;

use glfw::{Glfw, Window};
use bevy_ecs as bevy;
use bevy::prelude::*;

use render_stages::*;
use crate::core::renderer::api::{buffer::*, vao::*, shader::*, object::GLObject};
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

    // let vao = VertexArray::new()
    //     .with_vertex_attributes(vec![VertexAttribute::POSITION]);
    // vao.bind();

    // let vbo = VertexBuffer::vertex_buffer()
    //     .with_data(gl::STATIC_DRAW, vertices);
    // vbo.bind();

    // let ebo = ElementBuffer::element_buffer()
    //     .with_data(gl::STATIC_DRAW, indices);
    // ebo.bind();

    // vao.unbind();

    let vertex_shader = Shader::from_file("assets/shaders/debug.vert", ShaderType::Vertex).unwrap();
    let fragment_shader = Shader::from_file("assets/shaders/debug.frag", ShaderType::Fragment).unwrap();

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

        let vertices = vec![
            -0.5f32, -0.5, 0.0,
            0.0, 0.5, 0.0,
            0.5, -0.5, 0.0
        ];

        let mut renderer = Renderer { 
            world, 
            init_schedule,
            update_schedule,
        };

        renderer
    }

    pub fn setup(&mut self) {
        self.init_schedule.add_stage(InitStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(PrepareStage, SystemStage::parallel());
        self.update_schedule.add_stage(RenderStage, SystemStage::parallel());
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
        for (mesh, program) in q.iter() {
            mesh.bind();
            program.bind();

            unsafe {
                gl::DrawArrays(
                    gl::TRIANGLES,
                    0,
                    3
                );

                // gl::DrawElements(
                //     gl::TRIANGLES,
                //     mesh.element_count() as i32,
                //     gl::UNSIGNED_INT,
                //     0 as *const _
                // );
            }
        }
    }

    pub fn dispose(self) {}
}