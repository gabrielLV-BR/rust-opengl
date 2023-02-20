pub mod mesh_server;

pub trait AssetServer<T> {
    type Handle;
    fn store(&mut self, item: T) -> Self::Handle;
    fn get(&self, key: Self::Handle) -> Option<&T>;
}