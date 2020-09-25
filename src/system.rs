use crate::{EventDescription, Status};
use amethyst::core::math::Vector3;
use fmod_sys::*;
use log::debug;
use std::{
    ffi::{CStr, CString},
    mem,
    mem::MaybeUninit,
    ptr::null_mut
};

pub struct System {
    system: *mut FMOD_STUDIO_SYSTEM,
    master_bank: Option<Bank>,
    strings_bank: Option<Bank>,
    is_first: bool
}

unsafe impl Send for System {}
unsafe impl Sync for System {}

impl Drop for System {
    fn drop(&mut self) {
        //self.release().unwrap();
    }
}

impl System {
    pub fn new(master_name: &str, channels: i32) -> Result<System, Status> {
        let mut tmp = null_mut();

        unsafe {
            debug!("Creating system");
            Status::result(FMOD_Studio_System_Create(&mut tmp, FMOD_VERSION))?;
            let mut system = System {
                system: tmp,
                is_first: true,
                master_bank: None,
                strings_bank: None
            };
            debug!("Initializing system");
            system.init(channels)?;
            debug!("Loading master");
            system
                .master_bank
                .replace(system.load_bank_file(&format!("{}.bank", master_name))?);
            debug!("Loading strings");
            system
                .strings_bank
                .replace(system.load_bank_file(&format!("{}.strings.bank", master_name))?);
            Ok(system)
        }
    }

    pub fn find_event(&self, path: &str) -> Result<EventDescription, Status> {
        let event = CString::new(path).unwrap();
        let mut desc = null_mut();
        unsafe {
            Status::result(FMOD_Studio_System_GetEvent(
                self.system,
                event.as_ptr(),
                &mut desc
            ))?;
        }
        Ok(EventDescription { desc })
    }

    fn init(&self, channels: i32) -> Result<(), Status> {
        unsafe {
            Status::result(FMOD_Studio_System_Initialize(
                self.system,
                channels,
                FMOD_STUDIO_INIT_SYNCHRONOUS_UPDATE,
                FMOD_INIT_3D_RIGHTHANDED,
                null_mut()
            ))?;
        }
        Ok(())
    }

    pub fn core_system(&self) -> Result<CoreSystem, Status> {
        let mut system = null_mut();
        unsafe {
            Status::result(FMOD_Studio_System_GetCoreSystem(self.system, &mut system))?;
        }
        Ok(CoreSystem { system })
    }

    pub fn release(&mut self) -> Result<(), Status> {
        if self.is_first && !self.system.is_null() {
            unsafe {
                Status::result(FMOD_Studio_System_Release(self.system))?;
                self.system = null_mut();
            }
        }
        Ok(())
    }

    pub fn load_bank_file(&self, file_name: &str) -> Result<Bank, Status> {
        let c_string = CString::new(file_name).unwrap();
        let mut temp = null_mut();

        unsafe {
            Status::result(FMOD_Studio_System_LoadBankFile(
                self.system,
                c_string.as_ptr(),
                0,
                &mut temp
            ))?;
        };

        Ok(Bank { bank: temp })
    }

    pub fn update(&self) -> Result<(), Status> {
        unsafe { Status::result(FMOD_Studio_System_Update(self.system)) }
    }

    pub fn set_listener_attributes(
        &self,
        listener: i32,
        attributes: Attributes3D
    ) -> Result<(), Status> {
        let mut attributes = attributes.into();
        unsafe {
            Status::result(FMOD_Studio_System_SetListenerAttributes(
                self.system,
                listener,
                &mut attributes
            ))
        }
    }
}

static UP: FMOD_VECTOR = FMOD_VECTOR {
    x: 0.0,
    y: 1.0,
    z: 0.0
};

static FORWARD: FMOD_VECTOR = FMOD_VECTOR {
    x: 0.0,
    y: 0.0,
    z: -1.0
};

pub struct Attributes3D {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>
}

impl From<Attributes3D> for FMOD_3D_ATTRIBUTES {
    fn from(attributes: Attributes3D) -> Self {
        let position = FMOD_VECTOR {
            x: attributes.position.x,
            y: attributes.position.y,
            z: attributes.position.z
        };
        let velocity = FMOD_VECTOR {
            x: attributes.velocity.x,
            y: attributes.velocity.y,
            z: attributes.velocity.z
        };

        FMOD_3D_ATTRIBUTES {
            position,
            velocity,
            forward: FORWARD,
            up: UP
        }
    }
}

pub struct CoreSystem {
    system: *mut FMOD_SYSTEM
}

impl CoreSystem {
    pub fn num_drivers(&self) -> Result<i32, Status> {
        let mut num = 0;
        unsafe {
            Status::result(FMOD_System_GetNumDrivers(self.system, &mut num))?;
        }
        Ok(num)
    }

    pub fn driver_info(&self, id: i32) -> Result<DriverInfo, Status> {
        unsafe {
            let mut name_buf = vec![0i8; 1024];
            let mut guid = MaybeUninit::zeroed().assume_init();
            let mut sample_rate = 0;
            let mut speaker_mode = FMOD_SPEAKERMODE::FMOD_SPEAKERMODE_DEFAULT;
            let mut speaker_mode_channels = 0;
            Status::result(FMOD_System_GetDriverInfo(
                self.system,
                id,
                name_buf.as_mut_ptr(),
                1024,
                &mut guid,
                &mut sample_rate,
                &mut speaker_mode,
                &mut speaker_mode_channels
            ))?;
            let name = CStr::from_ptr(mem::transmute(name_buf.as_ptr()))
                .to_string_lossy()
                .to_string();
            Ok(DriverInfo {
                name,
                guid,
                sample_rate,
                speaker_mode,
                speaker_mode_channels
            })
        }
    }

    pub fn set_driver(&self, id: i32) -> Result<(), Status> {
        unsafe { Status::result(FMOD_System_SetDriver(self.system, id)) }
    }
}

#[derive(Clone, Debug)]
pub struct DriverInfo {
    name: String,
    guid: FMOD_GUID,
    sample_rate: i32,
    speaker_mode: FMOD_SPEAKERMODE,
    speaker_mode_channels: i32
}

pub struct Bank {
    bank: *mut FMOD_STUDIO_BANK
}

unsafe impl Send for Bank {}
unsafe impl Sync for Bank {}

impl Drop for Bank {
    fn drop(&mut self) {
        debug!("Dropping bank");
        self.unload().unwrap();
    }
}

impl Bank {
    pub fn unload(&mut self) -> Result<(), Status> {
        unsafe {
            Status::result(FMOD_Studio_Bank_Unload(self.bank))?;
            self.bank = null_mut();
            Ok(())
        }
    }

    pub fn events(&self) -> Result<Vec<EventDescription>, Status> {
        let mut count = 0;
        unsafe {
            Status::result(FMOD_Studio_Bank_GetEventCount(self.bank, &mut count))?;

            let mut result = vec![null_mut(); count as usize];

            Status::result(FMOD_Studio_Bank_GetEventList(
                self.bank,
                result.as_mut_ptr(),
                count,
                &mut count
            ))?;

            Ok(result
                .into_iter()
                .map(|ptr| EventDescription { desc: ptr })
                .collect())
        }
    }

    pub fn load_sample_data(&self) -> Result<(), Status> {
        unsafe { Status::result(FMOD_Studio_Bank_LoadSampleData(self.bank)) }
    }

    pub fn path(&self) -> Result<String, Status> {
        //println!("Test");
        let mut buf = vec![0i8; 1024];
        let mut retrieved = 0;

        unsafe {
            //println!("Getting path");
            Status::result(FMOD_Studio_Bank_GetPath(
                self.bank,
                buf.as_mut_ptr(),
                1024,
                &mut retrieved
            ))?;
            //println!("Result: {}", result);
            //println!("Got path");
        }

        buf.truncate(retrieved as usize);

        //println!("Retrieved: {}", retrieved);
        let bytes: Vec<_> = buf[..retrieved as usize]
            .iter()
            .map(|b| unsafe { mem::transmute(*b) })
            .collect();
        Ok(String::from_utf8(bytes).unwrap())
    }
}
