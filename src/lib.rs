use std::{error::Error, fmt, fmt::Display};

mod bundle;
mod event;
mod resource;
mod system;
mod update_system;

pub use crate::animation::event_system::*;
pub use bundle::*;
pub use event::*;
pub use resource::*;
pub use system::*;
pub use update_system::*;

#[derive(Clone, PartialEq, Debug, Copy)]
#[repr(C)]
pub enum Status {
    Ok,
    BadCommand,
    ChannelAlloc,
    ChannelStolen,
    Dma,
    DspConnection,
    DspDontProcess,
    DspFormat,
    DspInUse,
    DspNotFound,
    DspReserved,
    DspSilence,
    DspType,
    FileBad,
    FileCouldNotSeek,
    FileDiskEjected,
    FileEof,
    FileEndOfData,
    FileNotFound,
    Format,
    HeaderMismatch,
    Http,
    HttpAccess,
    HttpProxyAuth,
    HttpServerError,
    HttpTimeout,
    Initialization,
    Initialized,
    Internal,
    InvalidFloat,
    InvalidHandle,
    InvalidParam,
    InvalidPosition,
    InvalidSpeaker,
    InvalidSyncPoint,
    InvalidThread,
    InvalidVector,
    MaxAudible,
    Memory,
    MemoryCantPoint,
    Needs3D,
    NeedsHardware,
    NetConnect,
    NetSocketError,
    NetUrl,
    NetWouldBlock,
    NotReady,
    OutputAllocated,
    OutputCreateBuffer,
    OutputDriverCall,
    OutputFormat,
    OutputInit,
    OutputNoDrivers,
    Plugin,
    PluginMissing,
    PluginResource,
    PluginVersion,
    Record,
    ReverbChannelGroup,
    ReverbInstance,
    SubSounds,
    SubSoundAllocated,
    SubSoundCantMove,
    TagNotFound,
    TooManyChannels,
    Truncated,
    Unimplemented,
    Uninitialized,
    Unsupported,
    Version,
    EventAlreadyLoaded,
    EventLiveUpdateBusy,
    EventLiveUpdateMismatch,
    EventLiveUpdateTimeout,
    EventNotFound,
    StudioUninitialized,
    StudioNotLoaded,
    InvalidString,
    AlreadyLocked,
    NotLocked,
    RecordDisconnected,
    TooManySamples,
    File(File),
    Event(Event)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    NotFound
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    NotFound
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Status {}
impl Error for Event {}
impl Error for File {}

impl Status {
    pub fn to_result(self) -> Result<(), Status> {
        match self {
            Status::Ok => Ok(()),
            err => Err(err)
        }
    }

    pub fn result(value: i32) -> Result<(), Status> {
        Self::from_status(value).to_result()
    }

    pub fn from_status(value: i32) -> Status {
        match value {
            0 => Status::Ok,
            18 => Status::File(File::NotFound),
            74 => Status::Event(Event::NotFound),
            _ => panic!("Unknown error code: {}", value)
        }
    }
}
