use bevy_ecs::prelude::Component;
use ultraviolet::Mat4;

#[derive(Component)]
pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_perspective_matrix() -> Mat4 {
        todo!()
    }

    pub fn get_view_matrix() -> Mat4 {
        todo!()
    }
}