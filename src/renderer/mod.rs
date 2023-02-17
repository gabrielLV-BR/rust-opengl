mod api;
mod components;

use glfw::{Glfw, Window};
use bevy_ecs::{world::World, schedule::Schedule, prelude::Query, system::Res};

use crate::server::texture_server::TextureServer;

use self::{api::{shader::{Program, UniformType}, object::GLObject}, components::{camera::Camera, transform::Transform, mesh::Mesh}};

pub struct Renderer {
    world: World,
    schedule: Schedule
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
        let schedule = Schedule::default();

        Renderer { world, schedule }
    }

    pub fn setup(&mut self) {

    }

    pub fn update(&mut self, delta: f32) {
        self.schedule.run_once(&mut self.world);

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render(
        camera: &Res<Camera>,
        texture_server: Res<&TextureServer>,
        q: Query<(&Mesh, &Program, &Transform)>
    ) {
        let model = camera.as_ref;

        for (vao, program, texture) in q.iter() {
            unsafe {
                texture.bind();
                vao.bind();
                program.bind();
                program.set_uniform("uTexture", UniformType::Int(0));
    
                program.set_uniform("uModelMatrix", UniformType::Matrix4(&model));
                program.set_uniform("uViewMatrix", UniformType::Matrix4(&view));
                program.set_uniform("uProjMatrix", UniformType::Matrix4(&proj));
    
                gl::DrawElements(
                    gl::TRIANGLES, 
                    ebo.count() as i32,
                    gl::UNSIGNED_INT, 
                    0 as *const _
                );
            }
        }
    }

    pub fn dispose(self) {

    }
}