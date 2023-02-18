use bevy_ecs::system::Resource;
use ultraviolet::Vec3;

#[derive(Resource)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3
}