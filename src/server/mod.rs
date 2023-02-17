pub mod texture_server;

#[derive(Debug)]
pub struct NotFound {}

pub trait AssetServerTrait<T> {
    fn init() -> Self;
    fn load(path: &str) -> Option<T>;
    fn get(handle: u32) -> Result<T, NotFound>;
    fn unload(handle: u32);
    fn dispose() -> Self;
}