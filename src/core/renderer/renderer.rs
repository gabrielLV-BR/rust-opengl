use glfw::{Glfw, Window};

use super::RenderWorld;

pub struct Renderer {

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

        Renderer {}
    }

    pub fn render(&self, render_world: &RenderWorld) {
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn dispose(self) {
        todo!()
    }
}