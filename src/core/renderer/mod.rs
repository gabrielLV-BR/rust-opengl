pub mod api;
pub mod render_stages;

use glfw::{Glfw, Window};
use bevy_ecs as bevy;
use bevy::prelude::*;

use render_stages::*;

use crate::servers::{texture_server::TextureServer, AssetServerTrait, mesh_server::{MeshServer, self}};

use self::api::{shader::{Program, Shader, ShaderType}, object::GLObject, vao::VertexArray, buffer::{VertexBuffer, ElementBuffer}};

use super::components::mesh::{MeshInstance, MeshData, self};

pub struct Renderer {
    world: bevy::world::World,
    init_schedule: bevy::schedule::Schedule,
    update_schedule: bevy::schedule::Schedule,
}

fn setup_object(
    mut command: bevy::system::Commands,
    mut mesh_server: ResMut<MeshServer>
    // mut texture_server: ResMut<TextureServer>
) {

    // let texture = texture_server.load("assets/textures/obama.jpg").unwrap();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
        0.5, -0.5, 0.0
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2
    ];

    let mesh = MeshData::new(vertices, indices);    
    let handle = mesh_server.as_mut().insert(mesh);
    let mesh_instance = mesh_server.as_mut().new_instance(handle).unwrap();


    // let mut vertex_array = api::vao::VertexArray::new();
    // vertex_array.set_vertex_attributes(vec![
    //     api::vao::VertexAttribute::POSITION
    // ]);
    // vertex_array.bind();

    // let mut vertex_buffer = VertexBuffer::new(gl::ARRAY_BUFFER);
    // let mut element_buffer = ElementBuffer::new(gl::ELEMENT_ARRAY_BUFFER);

    // vertex_buffer.set_data(gl::STATIC_COPY, vertices);
    // element_buffer.set_data(gl::STATIC_DRAW, indices);    

    // vertex_buffer.bind();
    // element_buffer.bind();
    // vertex_array.unbind();

    // program
    let vertex_shader = Shader::from_file(
        "assets/shaders/debug.vert", 
        ShaderType::Vertex
    ).unwrap();

    let fragment_shader = Shader::from_file(
        "assets/shaders/debug.frag", 
        ShaderType::Fragment
    ).unwrap();

    let program = Program::new(vertex_shader, fragment_shader);
    
    command.spawn((mesh_instance, program));
    // command.spawn((vertex_array, vertex_buffer, element_buffer, program));
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

        let mut renderer = Renderer { 
            world, 
            init_schedule,
            update_schedule 
        };

        Self::setup(&mut renderer);

        renderer
    }

    pub fn setup(&mut self) {
        self.init_schedule.add_stage(InitStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(PrepareStage, SystemStage::parallel());
        self.update_schedule.add_stage(RenderStage, SystemStage::parallel());
        self.update_schedule.add_stage(CleanupStage, SystemStage::parallel());
    
        self.world.insert_resource(TextureServer::new());
        self.world.insert_resource(MeshServer::new());

        self.init_schedule.add_system_to_stage(InitStage, setup_object);
        self.update_schedule.add_system_to_stage(RenderStage, Self::render);

        self.init_schedule.run_once(&mut self.world);
    }

    pub fn update(&mut self, _delta: f32) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.update_schedule.run_once(&mut self.world);
    }

    pub fn render(
        q: Query<(&MeshInstance, &Program)>
    ) {
        // let model = camera.as_ref;

        for (instance, program) in q.iter() {

            program.bind();
            instance.bind();

            // program.set_uniform("aColor", UniformType::Vec3(&Vec3::new(1.0, 0.0, 1.0)));

            unsafe {
                // gl::BindTexture(gl::TEXTURE_2D, mesh.texture_handle.unwrap());
                // program.set_uniform("uTexture", UniformType::Int(0));
    
                // gl::DrawArrays(
                //     gl::TRIANGLES,
                //     0,
                //     3
                // );

                gl::DrawElements(
                    gl::TRIANGLES, 
                    instance.element_count as i32,
                    gl::UNSIGNED_INT, 
                    0 as *const _
                );

                // gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }

    pub fn dispose(self) {

    }
}