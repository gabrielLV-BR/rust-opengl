use ultraviolet::{Vec2, Vec3};

#[derive(Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2
}