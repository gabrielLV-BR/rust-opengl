use glfw::{Glfw, Window};

use bevy_ecs::{world::World, schedule::Schedule};

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
    }

    pub fn dispose(self) {

    }
}