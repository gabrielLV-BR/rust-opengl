mod core;
mod servers;

use glfw::{self, Context};
use std::time::Duration;
use ultraviolet::Vec3;

use crate::core::{
    components::transform::Transform,
    renderer::{
        backend::gl::{vao::VertexAttribute, vertex::Vertex},
        components::{material::BasicMaterial, mesh::Mesh},
        RenderNode, RenderWorld,
    },
};

fn main() {
    const WIDTH: i32 = 500;
    const HEIGHT: i32 = 500;
    const TARGET_FPS: i32 = 60;
    const FRAME_DURATION: f32 = 1f32 / TARGET_FPS as f32;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));

    let (mut window, _) = glfw
        .create_window(
            WIDTH as u32,
            HEIGHT as u32,
            "Hello",
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    glfw.make_context_current(Some(&window));
    // glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    glfw.make_context_current(Some(&window));

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    let mut renderer = core::renderer::Renderer::new(&mut glfw, &window);
    let mut render_world = RenderWorld::new();

    let vertices = vec![
        Vertex::position(-0.5, -0.5, 0.0),
        Vertex::position(0.0, 0.5, 0.0),
        Vertex::position(0.5, -0.5, 0.0),
    ];
    let indices = vec![0u32, 1, 2];

    let mesh = Mesh::new(vertices, indices, vec![VertexAttribute::POSITION]);
    let basic_material = BasicMaterial::new(Vec3::new(1.0, 0.0, 0.0));

    render_world.add_node_with(mesh, Box::new(basic_material), Transform::identity());

    let mut delta: f32;
    let mut time: f32;
    let mut last_time = glfw.get_time() as f32;

    while !window.should_close() {
        time = glfw.get_time() as f32;
        delta = time - last_time;

        renderer.render(&render_world);

        glfw.poll_events();
        window.swap_buffers();

        let wait_time = FRAME_DURATION;
        let curr_frame_time = glfw.get_time() as f32 - time;
        let dur = 1000.0 * (wait_time - curr_frame_time) + 0.5;

        if dur > 0f32 {
            std::thread::sleep(Duration::from_millis(dur as u64));
        }

        last_time = time;
    }

    renderer.dispose();
}
