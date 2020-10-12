use crate::{update_camera_position_system, update_fmod_system, AudioSystem, System};
use amethyst::{
    core::{dispatcher::SystemBundle, ecs::DispatcherBuilder},
    Error
};
use legion::{Resources, World};

pub struct FmodBundle {
    system: System,
    base_path: String
}

impl FmodBundle {
    pub fn new(system: System, base_path: String) -> Self {
        Self { system, base_path }
    }
}

impl SystemBundle for FmodBundle {
    fn load(
        &mut self,
        _: &mut World,
        resources: &mut Resources,
        builder: &mut DispatcherBuilder
    ) -> Result<(), Error> {
        resources.insert(AudioSystem::new(
            self.system.clone(),
            self.base_path.clone()
        ));
        builder.add_system(update_camera_position_system());
        builder.add_system(update_fmod_system());
        Ok(())
    }
}
