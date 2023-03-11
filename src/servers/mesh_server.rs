// use std::collections::HashMap;

// use crate::core::{components::mesh::Mesh, renderer::api::object::GLObject};

// use super::AssetServer;

// pub struct MeshServer {
//     map: HashMap<u32, Mesh>
// }

// impl MeshServer {
//     pub fn new() -> Self {
//         MeshServer {
//             map: HashMap::new()
//         }
//     }
// }

// impl AssetServer<Mesh> for MeshServer {
//     type Handle = u32;

//     fn get(&self, key: Self::Handle) -> Option<&Mesh> {
//         self.map.get(&key)
//     }

//     fn store(&mut self, item: Mesh) -> Self::Handle {
//         let key = item.handle();
//         self.map.insert(key, item);
//         key
//     }
// }