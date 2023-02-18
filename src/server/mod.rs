pub mod texture_server;

#[derive(Debug)]
pub struct NotFound {}

pub trait AssetServerTrait<T> {
    fn init() -> Self;
    fn load(&mut self, path: &str) -> Result<&T, image::ImageError>;
    fn get(&self, path: &str) -> Option<&T>;
    fn unload(&mut self, path: &str);
    fn dispose(self);
}