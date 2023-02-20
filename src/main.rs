mod core;
mod servers;

use glfw::{self, Context};
use std::{time::Duration, mem::size_of_val};

use crate::{servers::mesh_server::MeshServer, core::{renderer::api::{shader::{Program, Shader, ShaderType}, object::GLObject, vao::{VertexArray, VertexAttribute}, buffer::{VertexBuffer, ElementBuffer}}, components::mesh::{MeshData, MeshInstance}}};

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
    // glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    glfw.make_context_current(Some(&window));

    gl::load_with(
        |s| glfw.get_proc_address_raw(s)
    );
    
    unsafe {
        let (width, height) = window.get_size();
        gl::Viewport(0, 0, width, height);
    }

    // let mut renderer = core::renderer::Renderer::new(&mut glfw, &window);

    let mut delta = 0f32;
    let mut time = 0f32;
    let mut last_time = glfw.get_time() as f32;

    let vertices = vec![
        -0.5f32, -0.5, 0.0,
        0.0, 0.5, 0.0,
        0.5, -0.5, 0.0
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2
    ];

    // let mut vao = VertexArray::new();
    // vao.set_vertex_attributes(vec![VertexAttribute::POSITION]);
    // vao.bind();

    // let mut vbo = VertexBuffer::new(gl::ARRAY_BUFFER);
    // vbo.set_data(gl::STATIC_DRAW, vertices);

    // let mut ebo = ElementBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
    // ebo.set_data(gl::STATIC_DRAW, indices);

    // vbo.bind();
    // ebo.bind();

    // vao.unbind();

    let mut vao = 0u32;
    let mut vbo = 0u32;
    let mut ebo = 0u32;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(2, [&mut vbo, &mut ebo].as_mut_ptr().cast());

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            0 as *const _
        );
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);
    }

    // let instance = MeshInstance::new(meshes.first_mut().unwrap());

    let program = Program::new(
        Shader::from_file("assets/shaders/debug.vert", ShaderType::Vertex).unwrap(),
        Shader::from_file("assets/shaders/debug.frag", ShaderType::Fragment).unwrap()
    );

    while !window.should_close() {
        time = glfw.get_time() as f32;
        delta = time - last_time;

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            program.bind();
            // vao.bind();

            gl::DrawElements(
                gl::TRIANGLES,
                3,
                gl::UNSIGNED_INT,
                0 as *const _
            )
        }
        // renderer.update(delta);

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

    // renderer.dispose();

    drop(window);
    drop(glfw);
}
