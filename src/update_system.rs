use crate::{resource::AudioSystem, Attributes3D};
use amethyst::{
    core::{
        ecs::{Join, ReadStorage},
        math::Vector2,
        Transform
    },
    derive::SystemDesc,
    ecs::{ReadExpect, System, SystemData},
    renderer::Camera
};

#[derive(SystemDesc)]
pub struct UpdateSystem;

impl<'s> System<'s> for UpdateSystem {
    type SystemData = (
        ReadExpect<'s, AudioSystem>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>
    );

    fn run(&mut self, (fmod, camera, transform): Self::SystemData) {
        fmod.update().unwrap();
        let camera = (&camera, &transform).join().next();
        if let Some((_, transform)) = camera {
            let pos = transform.translation();
            fmod.set_listener_attributes(
                0,
                Attributes3D {
                    position: Vector2::new(pos.x, pos.y),
                    velocity: Vector2::new(0.0, 0.0)
                }
            )
            .unwrap();
        }
    }
}
