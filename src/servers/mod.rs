pub mod texture_server;

pub trait AssetServerTrait<T, E>
where E: std::error::Error {
    fn new() -> Self;
    fn load(&mut self, path: &str) -> Result<&T, E>;
    fn get(&self, path: &str) -> Option<&T>;
    fn unload(&mut self, path: &str);
    fn dispose(self);
}