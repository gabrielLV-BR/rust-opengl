use ultraviolet::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub program_handle: u32,
    pub texture_handle: Option<u32>,
}