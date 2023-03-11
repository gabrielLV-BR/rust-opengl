pub mod api;
pub mod components;
pub mod config;

use glfw::{Glfw, Window};

use self::components::{scene::Scene, camera::Camera};

pub struct Renderer {
    
}

// pub fn setup_test_object(
//     mut commands: Commands
// ) {
//     use tobj::LoadOptions;

//     let (models, _) = tobj::load_obj("assets/cube.obj", &LoadOptions {
//         triangulate: true,
//         ..Default::default()
//     }).unwrap();

//     let model = Model::from(models.get(0).unwrap().to_owned());

//     let texture = api::texture::Texture::load_from("assets/textures/tile.png")
//         .expect("Error loading texture");

//     commands.spawn((model, TexturedMaterial::new(texture)));
// }

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

    pub fn draw(scene: Scene, camera: Camera) {

    }

    // fn draw() {
    //     let model_matrix = ultraviolet::Mat4::from_scale(0.2);

    //     for (model, material) in q.iter() {
    //         let program = program_server.get(material.material_type).expect("Could not find material's shader");

    //         //TODO: set MVP matrix uniform from Transform and Camera component
    //         program.bind();
    //         material.texture.bind();
            
    //         program.set_uniform("uModelMatrix", UniformType::Matrix4(&model_matrix));
    //         program.set_uniform("uTexture", UniformType::Int(0));

    //         for mesh in model.meshes.iter() {
    //             mesh.bind();
    //             unsafe {
    //                 gl::DrawElements(
    //                     gl::TRIANGLES,
    //                     mesh.elements().count() as i32,
    //                     gl::UNSIGNED_INT,
    //                     std::ptr::null()
    //                 );
    //             }
    //         }
            
    //         program.unbind();
    //     }
    // }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn dispose(self) {
        todo!()
    }
}