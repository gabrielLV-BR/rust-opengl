use std::collections::HashMap;
use bevy_ecs::system::Resource;

use super::AssetServerTrait;
use crate::core::renderer::api::texture::Texture;

#[derive(Resource)]
pub struct TextureServer {
    map: HashMap<String, Texture>,
}

impl<> AssetServerTrait<Texture, image::error::ImageError> for TextureServer {
    fn new() -> Self {
        TextureServer { 
            map: HashMap::new()
        }        
    }

    fn load(&mut self, path: &str) -> Result<&Texture, image::ImageError> {
        let image = image::open(path)?;
        let texture = Texture::new(image);

        self.map.insert(path.to_owned(), texture);
        Ok(self.map.get(path).unwrap())
    }

    fn get(&self, path: &str) -> Option<&Texture> {
        self.map.get(path)
    }

    fn unload(&mut self, path: &str) {
        self.map.remove(path);
    }

    fn dispose(self) {
        for texture in self.map.values() {
            drop(texture)
        }
    }
}