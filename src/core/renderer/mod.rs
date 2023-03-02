pub mod api;
pub mod render_stages;

use glfw::{Glfw, Window};
use bevy_ecs as bevy;
use bevy::prelude::*;

use render_stages::*;
use crate::core::components::material::{BasicMaterial, TexturedMaterial};
use crate::core::components::vertex::Vertex;
use crate::core::renderer::api::vao::VertexAttribute;
use crate::core::renderer::api::{shader::*, object::GLObject};
use crate::core::components::mesh::{Mesh, MeshBuilder};
use crate::servers::AssetServer;
use crate::servers::program_server::ProgramServer;

pub struct Renderer {
    world: bevy::world::World,
    init_schedule: bevy::schedule::Schedule,
    update_schedule: bevy::schedule::Schedule,
}

pub fn setup_test_object(
    mut commands: Commands
) {
    use tobj::LoadOptions;
    use ultraviolet::{Vec3, Vec2};

    let (models, _) = tobj::load_obj("assets/cube.obj", &LoadOptions {
        triangulate: true,
        ..Default::default()
    }).unwrap();

    let mut vertices : Vec<Vertex> = vec![];
    let mut indices  : Vec<u32>    = vec![];


    for model in &models {
        for index in &model.mesh.indices {
            let vertex = Vertex {
                position: Vec3::zero(),
                normal: Vec3::zero(),
                uv: Vec2::zero(),
            };

            vertices.push(vertex);
            indices.push(indices.len() as u32);
        }
    }

    let mesh = Mesh::new(vertices, indices, vec![
        VertexAttribute::POSITION,
        VertexAttribute::NORMAL,
        VertexAttribute::UV,
    ]);

    // let mesh = MeshBuilder::new()
    //     .with_positions(model_mesh.positions)
    //     .with_normals(model_mesh.normals)
    //     .with_uv(model_mesh.texcoords)
    //     .with_indices(model_mesh.indices)
    //     .build();

    let texture = api::texture::Texture::load_from("assets/textures/tile.png")
        .expect("Error loading texture");

    commands.spawn((mesh, TexturedMaterial::new(texture)));
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

        Renderer { 
            world, 
            init_schedule,
            update_schedule,
        }
    }

    pub fn setup(&mut self) {
        self.init_schedule.add_stage(InitStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(PrepareStage, SystemStage::parallel());
        self.update_schedule.add_stage(RenderStage, SystemStage::single_threaded());
        self.update_schedule.add_stage(CleanupStage, SystemStage::parallel());
    
        self.world.insert_resource(ProgramServer::new());

        self.init_schedule.add_system_to_stage(InitStage, setup_test_object);
        self.update_schedule.add_system_to_stage(RenderStage, render_basic_material);

        self.init_schedule.run_once(&mut self.world);
    }

    pub fn update(&mut self, _delta: f32) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.update_schedule.run_once(&mut self.world);
    }

    pub fn dispose(self) {
        todo!()
    }
}

fn render_basic_material(
    program_server: Res<ProgramServer>,
    q: Query<(&Mesh, &TexturedMaterial, /* &Transform */)>,
    // u: Query<&Camera>
) {

    let model = ultraviolet::Mat4::from_scale(0.2);

    q.for_each(|(mesh, material)| {
        let program = program_server.get(material.material_type).expect("Could not find material's shader");

        //TODO: set MVP matrix uniform from Transform and Camera component
        program.bind();
        material.texture.bind();
        
        program.set_uniform("uModelMatrix", UniformType::Matrix4(&model));
        program.set_uniform("uTexture", UniformType::Int(0));

        mesh.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                mesh.elements().count() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null()
            );
        }

        mesh.unbind();
        program.unbind();
    });
}