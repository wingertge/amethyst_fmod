use crate::{AudioSystem, System, UpdateSystem};
use amethyst::{
    core::{
        shred::{DispatcherBuilder, World},
        SystemBundle
    },
    Error
};

pub struct FmodBundle {
    system: System,
    base_path: String
}

impl FmodBundle {
    pub fn new(system: System, base_path: String) -> Self {
        Self { system, base_path }
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for FmodBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'a, 'b>
    ) -> Result<(), Error> {
        world.insert(AudioSystem::new(self.system, self.base_path));
        dispatcher.add(UpdateSystem, "fmod_update_system", &[]);

        Ok(())
    }
}
