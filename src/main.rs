mod graphics;

use glfw::{self, Context};
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
        -0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.0,  0.5,  0.0,  0.0,  1.0,  0.0,  0.5,  1.0,
         0.5, -0.5,  0.0,  0.0,  0.0,  1.0,  1.0,  0.0
    ];

    let mut vao = VertexArray::new();
    let mut vbo = Buffer::new(gl::ARRAY_BUFFER);


    vao.bind();

    vao.vertex_attributes(vec![
        VertexAttribute::with_count(3),
        VertexAttribute::with_count(3),
        VertexAttribute::with_count(2)
    ]);

    vbo.set_vertices(gl::STATIC_DRAW, vertices);
    vbo.bind();

    vao.unbind();
    vbo.unbind();
    
    let vertex_shader_source = include_str!("../assets/shaders/basic.vert");
    let fragment_shader_source = include_str!("../assets/shaders/basic.frag");

    let vertex_shader   = Shader::new(vertex_shader_source, ShaderType::Vertex);
    let fragment_shader = Shader::new(fragment_shader_source, ShaderType::Fragment);
    let program = Program::new(vertex_shader, fragment_shader);

    let obama_texture = image::open("assets/textures/obama.jpg").unwrap();
    let texture = Texture::new(obama_texture);

    while !window.should_close() {

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            texture.bind();
            vao.bind();
            program.bind();
            program.set_uniform("uTexture", UniformType::Int(0));

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        glfw.poll_events();
        window.swap_buffers();
    }

    drop(window);
    drop(glfw);
}
