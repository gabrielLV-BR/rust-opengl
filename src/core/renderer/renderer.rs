use glfw::{Glfw, Window};

use super::{backend::gl::object::GLObject, RenderWorld};

pub struct Renderer {}

impl Renderer {
    pub fn new(glfw: &mut Glfw, window: &Window) -> Self {
        glfw.make_context_current(Some(&window));

        gl::load_with(|s| glfw.get_proc_address_raw(s));

        unsafe {
            let (width, height) = window.get_size();
            gl::Viewport(0, 0, width, height);
        }

        Renderer {}
    }

    pub fn render(&self, render_world: &RenderWorld) {
        for node in render_world.nodes() {
            if let Some(mesh_handle) = node.mesh_handle() {
                let mesh = render_world
                    .get_mesh(mesh_handle)
                    .expect("Invalid mesh handle");

                mesh.bind();

                if let Some(material_handle) = node.material_handle() {
                    let material = render_world
                        .get_material(material_handle)
                        .expect("Invalid material handle");

                    let program = render_world
                        .get_shader(material.as_ref())
                        .expect("No shader provided for material");

                    material.bind(program);
                }

                //TODO maybe abstract this even more? Definitely for later

                unsafe {
                    gl::DrawElements(
                        gl::TRIANGLES,
                        mesh.elements().count() as i32,
                        gl::UNSIGNED_INT,
                        0 as *const std::os::raw::c_void,
                    )
                }
            }
        }
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
