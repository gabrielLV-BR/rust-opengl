mod core;
mod servers;

use glfw::{self, Context};
use std::time::Duration;

fn main() {
    const WIDTH: i32 = 500;
    const HEIGHT: i32 = 500;
    const TARGET_FPS: i32 = 60;
    const FRAME_DURATION: f32 = 1f32 / TARGET_FPS as f32;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));

    let (mut window, _) = glfw
        .create_window(WIDTH as u32, HEIGHT as u32, "Hello", glfw::WindowMode::Windowed)
        .unwrap();

    glfw.make_context_current(Some(&window));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let mut renderer = core::renderer::Renderer::new(&mut glfw, &window);

    let mut delta = 0f32;
    let mut time = 0f32;
    let mut last_time = glfw.get_time() as f32;

    while !window.should_close() {
        time = glfw.get_time() as f32;
        delta = time - last_time;

        renderer.update(delta);

        glfw.poll_events();
        window.swap_buffers();

        let wait_time = FRAME_DURATION;
        let curr_frame_time = glfw.get_time() as f32 - time;
        let dur = 1000.0 * ( wait_time - curr_frame_time ) + 0.5;

        if dur > 0f32 {
            std::thread::sleep(Duration::from_millis(dur as u64));
        }

        last_time = time;
    }

    renderer.dispose();

    drop(window);
    drop(glfw);
}
