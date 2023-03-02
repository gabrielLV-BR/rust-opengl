use std::hash::Hash;

pub mod camera;
pub mod mesh;
pub mod transform;
pub mod material;
pub mod vertex;

#[derive(PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Eq for Color {}

impl Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&[
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ])
    }
}