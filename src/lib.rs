use fmod_sys::FMOD_RESULT;
use std::{error::Error, fmt, fmt::Display};

mod bundle;
mod event;
mod resource;
mod system;
mod update_system;

pub use bundle::*;
pub use event::*;
pub use resource::*;
pub use system::*;
pub use update_system::*;

#[derive(Debug, Clone, Copy)]
pub struct Status(pub FMOD_RESULT);

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.message())
    }
}

impl Error for Status {}

impl Status {
    pub fn to_result(self) -> Result<(), Status> {
        match self {
            Self(FMOD_RESULT::FMOD_OK) => Ok(()),
            err => Err(err)
        }
    }

    pub fn result(value: FMOD_RESULT) -> Result<(), Status> {
        Self(value).to_result()
    }
}
