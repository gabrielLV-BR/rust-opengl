pub mod texture_server;
pub mod mesh_server;

pub trait AssetServerTrait<K, T, E>
where E: std::error::Error {
    fn load(&mut self, path: &str) -> Result<&T, E>;
    fn get(&self, key: K) -> Option<&T>;
    fn unload(&mut self, path: K);
}