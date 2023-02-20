pub mod api;
pub mod render_stages;

use glfw::{Glfw, Window};
use bevy_ecs as bevy;
use bevy::prelude::*;

use render_stages::*;

pub struct Renderer {
    world: bevy::world::World,
    init_schedule: bevy::schedule::Schedule,
    update_schedule: bevy::schedule::Schedule,
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

        Self::setup(&mut renderer);

        renderer
    }

    pub fn setup(&mut self) {
        self.init_schedule.add_stage(InitStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(PrepareStage, SystemStage::parallel());
        self.update_schedule.add_stage(RenderStage, SystemStage::parallel());
        self.update_schedule.add_stage(CleanupStage, SystemStage::parallel());
    
        self.init_schedule.run_once(&mut self.world);
    }

    pub fn update(&mut self, _delta: f32) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.update_schedule.run_once(&mut self.world);
    }

    pub fn render() {}
    pub fn dispose(self) {}
}