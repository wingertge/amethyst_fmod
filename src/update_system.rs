use crate::{resource::AudioSystem, Attributes3D};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::system,
    renderer::Camera
};

#[system(for_each)]
pub fn update_camera_position(_: &Camera, transform: &Transform, #[resource] fmod: &AudioSystem) {
    let pos = transform.translation();
    fmod.set_listener_attributes(
        0,
        Attributes3D {
            position: *pos,
            velocity: Vector3::new(0.0, 0.0, 0.0)
        }
    )
    .unwrap();
}

#[system]
pub fn update_fmod(#[resource] fmod: &AudioSystem) {
    fmod.update().unwrap();
}
