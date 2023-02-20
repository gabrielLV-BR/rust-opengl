
use std::collections::HashMap;
use bevy_ecs::system::Resource;

use super::AssetServerTrait;
use crate::core::components::mesh::{MeshData, MeshInstance};

#[derive(Resource)]
pub struct MeshServer {
    map: HashMap<u32, MeshData>
}

impl MeshServer {
    pub fn new() -> Self {
        MeshServer { 
            map: HashMap::new()
        }        
    }

    pub fn new_mesh(&mut self, vertices: Vec<f32>, indices: Vec<u32>) -> &mut MeshData {
        let mesh_data = MeshData::new(
            vertices, indices
        );
        let key = mesh_data.vao.handle;

        self.map.insert(key, mesh_data);
        self.map.get_mut(&key).unwrap()
    }

    pub fn new_instance(&mut self, key: u32) -> Option<MeshInstance> {
        match self.map.get(&key) {
            Some(mesh) => Some(MeshInstance::new(mesh)),
            None => None
        }
    }
}

impl Drop for MeshServer {
    fn drop(&mut self) {
        for texture in self.map.values() {
            drop(texture)
        }
    }
}

impl<> AssetServerTrait<u32, MeshData, tobj::LoadError> for MeshServer {
    fn load(&mut self, path: &str) -> Result<&MeshData, tobj::LoadError> {
        use tobj::LoadOptions;

        let (models, _materials) = tobj::load_obj(
            path, 
        &LoadOptions { 
                triangulate: true,
                ..Default::default() 
            }
        )?;

        let mesh = MeshData::from_model_list(models);

        let handle = mesh.vao.handle;

        self.map.insert(handle, mesh);
        Ok(&self.map.get(&handle).unwrap())
    }

    fn get(&self, handle: u32) -> Option<&MeshData> {
        self.map.get(&handle)
    }

    fn unload(&mut self, handle: u32) {
        self.map.remove(&handle);
    }
}