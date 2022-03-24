#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

use std::os::raw::c_void;

pub const EMPTY_ADDON_DSC: gw2al_addon_dsc = gw2al_addon_dsc {
    name: std::ptr::null(),
    description: std::ptr::null(),
    majorVer: 0,
    minorVer: 0,
    revision: 0,
    dependList: std::ptr::null_mut(),
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gw2al_addon_dsc {
    pub name: *const u16,
    pub description: *const u16,
    pub majorVer: u8,
    pub minorVer: u8,
    pub revision: u32,
    pub dependList: *mut gw2al_addon_dsc,
}

unsafe impl Send for gw2al_addon_dsc {}
unsafe impl Sync for gw2al_addon_dsc {}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum gw2al_api_ret {
    OK = 0,
    FAIL = 1,
    IN_USE = 2,
    NOT_FOUND = 3,
    BAD_DLL = 4,
    DEP_NOT_LOADED = 5,
    DEP_OUTDATED = 6,
    DEP_STILL_LOADED = 7,
    STATIC_LIMIT_HIT = 8,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum gw2al_log_level {
    INFO = 0,
    ERR = 1,
    WARN = 2,
    DEBUG = 3,
}

#[cfg(feature = "log")]
impl From<log::Level> for gw2al_log_level {
    fn from(level: log::Level) -> Self {
        use log::Level;
        match level {
            Level::Error => Self::ERR,
            Level::Warn => Self::WARN,
            Level::Info => Self::INFO,
            _ => Self::DEBUG,
        }
    }
}

pub type gw2al_api_event_handler = unsafe extern "C" fn(data: *mut c_void);
pub type gw2al_hashed_name = u64;
pub type gw2al_event_id = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gw2al_core_vtable {
    pub hash_name: unsafe extern "C" fn(name: *mut u16) -> gw2al_hashed_name,
    pub register_function:
        unsafe extern "C" fn(function: *mut c_void, name: gw2al_hashed_name) -> gw2al_api_ret,
    pub unregister_function: unsafe extern "C" fn(name: gw2al_hashed_name),
    pub query_function: unsafe extern "C" fn(name: gw2al_hashed_name) -> *mut c_void,
    pub fill_vtable:
        unsafe extern "C" fn(nameList: *mut gw2al_hashed_name, vtable: *mut *mut c_void),
    pub unload_addon: unsafe extern "C" fn(name: gw2al_hashed_name) -> gw2al_api_ret,
    pub load_addon: unsafe extern "C" fn(name: *mut u16) -> gw2al_api_ret,
    pub query_addon: unsafe extern "C" fn(name: gw2al_hashed_name) -> *mut gw2al_addon_dsc,
    pub watch_event: unsafe extern "C" fn(
        id: gw2al_event_id,
        subscriber: gw2al_hashed_name,
        handler: gw2al_api_event_handler,
        priority: u32,
    ) -> gw2al_api_ret,
    pub unwatch_event: unsafe extern "C" fn(id: gw2al_event_id, subscriber: gw2al_hashed_name),
    pub query_event: unsafe extern "C" fn(name: gw2al_hashed_name) -> gw2al_event_id,
    pub trigger_event: unsafe extern "C" fn(id: gw2al_event_id, data: *mut c_void) -> u32,
    pub client_unload: unsafe extern "C" fn(),
    pub log_text_sync:
        unsafe extern "C" fn(level: gw2al_log_level, source: *mut u16, text: *mut u16),
    pub log_text: unsafe extern "C" fn(level: gw2al_log_level, source: *mut u16, text: *mut u16),
}

// addon must export these functions as
// gw2addon_get_description
pub type gw2al_addon_get_dsc_proc = unsafe extern "C" fn() -> *mut gw2al_addon_dsc;
// gw2addon_load
pub type gw2al_addon_load_proc =
    unsafe extern "C" fn(core_api: *mut gw2al_core_vtable) -> gw2al_api_ret;
// gw2addon_unload
pub type gw2al_addon_unload_proc = unsafe extern "C" fn(gameExiting: i32) -> gw2al_api_ret;
