use bevy_ecs::prelude::Component;
use ultraviolet::{Vec3, Rotor3, Mat4};

#[derive(Component)]
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

    pub fn get_matrix(&self) -> Mat4 {
        todo!()
    }
}