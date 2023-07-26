use ultraviolet::{Vec2, Vec3};

#[derive(Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn position(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            position: Vec3::new(x, y, z),
            normal: Vec3::zero(),
            uv: Vec2::zero(),
        }
    }
}
