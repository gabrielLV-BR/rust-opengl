mod graphics;
mod loaders;
mod components;

use std::time::Duration;

use glfw::{self, Context};
use loaders::{obj::OBJLoader, LoaderTrait};
use ultraviolet::Vec3;
use crate::graphics::{
    object::GLObject, vao::{VertexArray, VertexAttribute},
    shader::{Shader, ShaderType, Program, UniformType}, 
    texture::Texture, buffer::Buffer
};

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

    let (mut window, events) = glfw
        .create_window(WIDTH as u32, HEIGHT as u32, "Hello", glfw::WindowMode::Windowed)
        .unwrap();

    glfw.make_context_current(Some(&window));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    unsafe {
        let (width, height) = window.get_size();
        gl::Viewport(0, 0, width, height);
    }

    let vertices: Vec<f32> = vec![
    //   X     Y     Z  |  R     G     B  |  U     V
        -0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.0,  0.5,  0.0,  0.0,  1.0,  0.0,  0.5,  0.0,
         0.5, -0.5,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0
    ];

    let indices = vec![
        0u32, 1, 2
    ];

    let mut loader = OBJLoader::new();
    loader.load("assets/prism2.obj").unwrap();


    println!("{:?}", loader.vertices);
    println!("{:?}", loader.indices);
    // loader.vertices.iter().for_each(|v| println!("{v}"));
    // loader.indices.iter().for_each(|v| println!("{v}"));


    let aspect = window.get_size().0 as f32 / window.get_size().1 as f32;
    let fov = 90f32;

    let mut model;
    let view =ultraviolet::Mat4::identity();
    let proj = ultraviolet::projection::perspective_gl(fov, aspect, 0.01f32, 1000.0f32);

    let mut vao = VertexArray::new();
    let mut vbo = Buffer::<f32>::new(gl::ARRAY_BUFFER);
    let mut ebo = Buffer::<u32>::new(gl::ELEMENT_ARRAY_BUFFER);

    vao.bind();

    vao.vertex_attributes(vec![
        VertexAttribute::POSITION,
        VertexAttribute::COLOR,
        VertexAttribute::UV,
    ]);

    vbo.set_data(gl::STATIC_DRAW, vertices);
    vbo.bind();

    ebo.set_data(gl::STATIC_DRAW, indices);
    ebo.bind();

    vao.unbind();
    vbo.unbind();
    ebo.unbind();

    let vertex_shader   = 
        Shader::from_file("assets/shaders/basic.vert", ShaderType::Vertex)
        .unwrap();
    let fragment_shader = 
        Shader::from_file("assets/shaders/basic.frag", ShaderType::Fragment)
        .unwrap();
    let program = Program::new(vertex_shader, fragment_shader);

    let obama_texture = image::open("assets/textures/obama.jpg").unwrap();
    let texture = Texture::new(obama_texture);

    let mut delta = 0f32;
    let mut time = 0f32;
    let mut last_time = glfw.get_time() as f32;

    let mut rotation = 0f32;

    window.set_key_polling(true);

    while !window.should_close() {
        time = glfw.get_time() as f32;
        delta = time - last_time;

        println!("Delta: {}", delta);

        model = ultraviolet::Mat4::from_rotation_x(rotation).translated(&Vec3::new(0., 0., -3.));

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

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

        glfw.poll_events();
        window.swap_buffers();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Repeat, _) => {
                    rotation += 0.01f32;
                },
                _ => {}
            }
        }

        let wait_time = FRAME_DURATION;
        let curr_frame_time = glfw.get_time() as f32 - time;
        let dur = 1000.0 * ( wait_time - curr_frame_time ) + 0.5;

        if dur > 0f32 {
            std::thread::sleep(Duration::from_millis(dur as u64));
        }

        last_time = time;
    }

    drop(window);
    drop(glfw);
}
