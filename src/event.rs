use crate::{Attributes3D, Status};
use fmod_sys::*;
use std::{ffi::CString, ptr::null_mut};

#[repr(transparent)]
pub struct EventDescription {
    pub(crate) desc: *mut FMOD_STUDIO_EVENTDESCRIPTION
}

impl EventDescription {
    pub fn release_all_instances(&mut self) -> Result<(), Status> {
        unsafe { Status::result(FMOD_Studio_EventDescription_ReleaseAllInstances(self.desc)) }
    }

    pub fn create_instance(&self) -> Result<EventInstance, Status> {
        let mut instance = null_mut();
        unsafe {
            Status::result(FMOD_Studio_EventDescription_CreateInstance(
                self.desc,
                &mut instance
            ))?;
        }
        Ok(EventInstance { instance })
    }

    pub fn path(&self) -> Result<String, Status> {
        //println!("Getting path");
        let mut buf = vec![0u8; 1024];
        let mut count = 0;

        unsafe {
            //println!("Calling C function");
            Status::result(FMOD_Studio_EventDescription_GetPath(
                self.desc,
                buf.as_mut_ptr() as *mut i8,
                1024,
                &mut count
            ))?;
        };
        //println!("Getting truncated bytes");
        //println!("Count: {}", count);
        buf.truncate(count as usize - 1);
        let s = String::from_utf8(buf).unwrap();
        //let s = unsafe { String::from_raw_parts(mem::transmute(buf), count as usize, count as usize) };
        //println!("Result: {}", result);
        Ok(s)
    }

    pub fn load_sample_data(&self) -> Result<(), Status> {
        unsafe { Status::result(FMOD_Studio_EventDescription_LoadSampleData(self.desc)) }
    }
}

#[repr(transparent)]
pub struct EventInstance {
    instance: *mut FMOD_STUDIO_EVENTINSTANCE
}

impl Drop for EventInstance {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

#[repr(C)]
pub enum StopMode {
    AllowFadeout = 0,
    Immediate = 1
}

impl From<StopMode> for FMOD_STUDIO_STOP_MODE {
    fn from(mode: StopMode) -> Self {
        match mode {
            StopMode::AllowFadeout => FMOD_STUDIO_STOP_MODE::FMOD_STUDIO_STOP_ALLOWFADEOUT,
            StopMode::Immediate => FMOD_STUDIO_STOP_MODE::FMOD_STUDIO_STOP_IMMEDIATE
        }
    }
}

impl EventInstance {
    pub fn start(&self) -> Result<(), Status> {
        unsafe { Status::result(FMOD_Studio_EventInstance_Start(self.instance)) }
    }

    pub fn stop(&self, mode: StopMode) -> Result<(), Status> {
        unsafe { Status::result(FMOD_Studio_EventInstance_Stop(self.instance, mode.into())) }
    }

    pub fn set_3d_attributes(&mut self, attributes: Attributes3D) -> Result<(), Status> {
        let mut attributes = attributes.into();
        unsafe {
            Status::result(FMOD_Studio_EventInstance_Set3DAttributes(
                self.instance,
                &mut attributes
            ))
        }
    }

    pub fn set_parameter_by_name(&self, name: &str, value: f32) -> Result<(), Status> {
        let name_c = CString::new(name).unwrap();
        unsafe {
            Status::result(FMOD_Studio_EventInstance_SetParameterByName(
                self.instance,
                name_c.as_ptr(),
                value,
                0
            ))
        }
    }

    pub fn release(&mut self) -> Result<(), Status> {
        unsafe {
            Status::result(FMOD_Studio_EventInstance_Release(self.instance))?;
            self.instance = null_mut();
            Ok(())
        }
    }
}
