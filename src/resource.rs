use crate::{
    Attributes3D, Bank, EventInstance, Status, System
};
use amethyst::core::{math::Vector2, num::Zero};
use std::ops::Deref;
use std::collections::HashMap;

pub struct AudioSystem {
    system: System,
    banks: HashMap<String, Bank>,
    base_path: String,
    simple_names: HashMap<String, String>
}

macro_rules! name {
    ($self: expr, $name: expr) => {
        $self
            .simple_names
            .get($name)
            .map(String::as_str)
            .unwrap_or($name)
    };
}

impl AudioSystem {
    pub fn new(system: System, base_path: String) -> Self {
        AudioSystem {
            system,
            banks: Default::default(),
            base_path,
            simple_names: Default::default()
        }
    }

    fn populate_simple_names(&mut self, bank: &Bank) -> Result<(), Status> {
        for event in bank.events()? {
            let path = event.path()?;
            let mut simple = path.rsplitn(2, '/').next().unwrap();
            if simple.contains(':') {
                simple = simple.rsplitn(2, ':').next().unwrap();
            }
            self.simple_names.insert(simple.to_string(), path);
        }
        Ok(())
    }

    pub fn load_bank(&mut self, name: &str) -> Result<(), Status> {
        if !self.banks.contains_key(name) {
            let bank = self
                .system
                .load_bank_file(&format!("{}{}.bank", self.base_path, name))?;
            self.populate_simple_names(&bank)?;
            self.banks.insert(name.to_string(), bank);
        }
        Ok(())
    }

    pub fn unload_bank(&mut self, name: &str) {
        self.banks.remove(name);
    }

    pub fn play_simple(&self, name: &str) -> Result<(), Status> {
        let name = name!(self, name);
        let description = self.system.find_event(name)?;
        let instance = description.create_instance()?;
        instance.start()
    }

    pub fn sound<'a>(&'a self, name: &'a str) -> SoundBuilder<'a> {
        SoundBuilder::new(self, name)
    }

    pub fn preload(&self, name: &str) -> Result<(), Status> {
        let description = self.system.find_event(name!(self, name))?;
        description.load_sample_data()?;
        Ok(())
    }
}

impl Deref for AudioSystem {
    type Target = System;

    fn deref(&self) -> &Self::Target {
        &self.system
    }
}

pub struct SoundBuilder<'a> {
    system: &'a AudioSystem,
    event: &'a str,
    position: Option<Vector2<f32>>,
    params: HashMap<&'a str, f32>
}

impl<'a> SoundBuilder<'a> {
    pub fn new(system: &'a AudioSystem, event: &'a str) -> Self {
        Self {
            system,
            event,
            position: None,
            params: Default::default()
        }
    }

    pub fn with_position(mut self, pos: Vector2<f32>) -> Self {
        self.position.replace(pos);
        self
    }

    pub fn with_param(mut self, name: &'a str, value: f32) -> Self {
        self.params.insert(name, value);
        self
    }

    pub fn with_params(mut self, params: &[(&'a str, f32)]) -> Self {
        self.params.extend(params.iter().copied());
        self
    }

    pub fn build(self) -> Result<EventInstance, Status> {
        let name = name!(self.system, self.event);
        let description = self.system.system.find_event(name)?;
        let mut instance = description.create_instance()?;

        if let Some(pos) = self.position {
            instance.set_3d_attributes(Attributes3D {
                position: pos,
                velocity: Vector2::zero()
            })?;
        }

        for (name, value) in self.params {
            instance.set_parameter_by_name(name, value)?;
        }

        Ok(instance)
    }

    pub fn play_once(self) -> Result<(), Status> {
        let instance = self.build()?;
        instance.start()
    }
}
