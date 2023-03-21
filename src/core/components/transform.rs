use ultraviolet::{Vec3, Rotor3, Mat4};

pub struct Transform {
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Rotor3
}

impl Transform {
    pub fn new() -> Self {
        Transform { 
            translation: Vec3::zero(), 
            scale: Vec3::zero(), 
            rotation: Rotor3::identity() 
        }
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        todo!()
    }

    pub fn identity() -> Self {
        Transform { 
            translation: Vec3::zero(), 
            scale: Vec3::zero(), 
            rotation: Rotor3::identity() 
        }
    }
}