mod graphics;
mod loaders;

use std::f64::consts::TAU;

use glfw::{self, Context};
use loaders::{obj::OBJLoader, LoaderTrait};
use crate::graphics::{
    object::GLObject, vao::{VertexArray, VertexAttribute},
    shader::{Shader, ShaderType, Program, UniformType}, 
    texture::Texture, buffer::Buffer
};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, _) = glfw
        .create_window(500, 500, "Hello", glfw::WindowMode::Windowed)
        .unwrap();

    glfw.make_context_current(Some(&window));

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    unsafe {
        let (width, height) = window.get_size();
        gl::Viewport(0, 0, width, height);
    }

    let vertices = vec![
    //   X     Y     Z  |  R     G     B  |  U     V
        -0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.0,  0.5,  0.0,  0.0,  1.0,  0.0,  0.5,  0.0,
         0.5, -0.5,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0
    ];

    let indices = vec![
        0u32, 1, 2
    ];

    // loader.load("assets/monkey.obj").unwrap();

    // loader.vertices.iter().for_each(|v| println!("{v}"));

    let mut model = glam::Mat4::IDENTITY;
    let view = glam::Mat4::IDENTITY;
    let proj = glam::Mat4::IDENTITY;

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

    while !window.should_close() {

        time = glfw.get_time() as f32;
        delta = last_time - time;

        println!("{time}");

        model = glam::Mat4::from_rotation_y(time * 0.1);

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
                std::ptr::null()
            );
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        glfw.poll_events();
        window.swap_buffers();

        last_time = glfw.get_time() as f32;
    }

    drop(window);
    drop(glfw);
}
