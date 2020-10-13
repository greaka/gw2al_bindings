mod gw2al_api;
pub use gw2al_api::*;
#[cfg(feature = "log")]
use log::{Metadata, Record};
use std::{ffi::c_void, ptr::NonNull};
use widestring::{U16CStr, U16CString};

pub struct Gw2Al<'a> {
    vtable: &'a gw2al_core_vtable,
}

impl Gw2Al<'_> {
    pub fn new(core: *mut gw2al_core_vtable) -> Self {
        Self {
            vtable: unsafe { &*core },
        }
    }

    pub fn hash_name(&self, name: &str) -> gw2al_hashed_name {
        let name = U16CString::from_str(name).unwrap_or_default();
        unsafe { (self.vtable.hash_name)(name.as_ptr() as _) }
    }

    pub fn register_function(
        &self,
        function: *mut c_void,
        name: gw2al_hashed_name,
    ) -> gw2al_api_ret {
        unsafe { (self.vtable.register_function)(function, name) }
    }

    pub fn unregister_function(&self, name: gw2al_hashed_name) {
        unsafe { (self.vtable.unregister_function)(name) }
    }

    pub fn query_function(&self, name: gw2al_hashed_name) -> *mut c_void {
        unsafe { (self.vtable.query_function)(name) }
    }

    pub fn fill_vtable(&self, name_list: *mut gw2al_hashed_name, vtable: *mut *mut c_void) {
        unsafe { (self.vtable.fill_vtable)(name_list, vtable) }
    }

    pub fn load_addon(&self, name: &str) -> gw2al_api_ret {
        let name = U16CString::from_str(name).unwrap_or_default();
        unsafe { (self.vtable.load_addon)(name.as_ptr() as _) }
    }

    pub fn unload_addon(&self, name: gw2al_hashed_name) -> gw2al_api_ret {
        unsafe { (self.vtable.unload_addon)(name) }
    }

    pub fn query_addon(&self, name: gw2al_hashed_name) -> Option<Gw2AlAddonDsc> {
        let dsc = unsafe { (self.vtable.query_addon)(name) };
        if unsafe { (&*dsc).name }.is_null() {
            return None;
        }
        let desc = unsafe { NonNull::new_unchecked(dsc) };
        Some(desc.into())
    }

    pub fn watch_event(
        &self,
        id: gw2al_event_id,
        subscriber: gw2al_hashed_name,
        handler: gw2al_api_event_handler,
        priority: u32,
    ) -> gw2al_api_ret {
        unsafe { (self.vtable.watch_event)(id, subscriber, handler, priority) }
    }

    pub fn unwatch_event(&self, id: gw2al_event_id, subscriber: gw2al_hashed_name) {
        unsafe { (self.vtable.unwatch_event)(id, subscriber) }
    }

    pub fn query_event(&self, name: gw2al_hashed_name) -> gw2al_event_id {
        unsafe { (self.vtable.query_event)(name) }
    }

    pub fn trigger_event(&self, id: gw2al_event_id, data: *mut c_void) -> u32 {
        unsafe { (self.vtable.trigger_event)(id, data) }
    }

    pub fn client_unload(&self) {
        unsafe { (self.vtable.client_unload)() }
    }

    pub fn log_text(&self, level: gw2al_log_level, source: &str, text: &str) {
        let src = U16CString::from_str(source).unwrap_or_default();
        let txt = U16CString::from_str(text).unwrap_or_default();
        unsafe { (self.vtable.log_text)(level, src.as_ptr() as _, txt.as_ptr() as _) }
    }
}

#[cfg(feature = "log")]
impl log::Log for Gw2Al<'_> {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        let from = format!(
            "{}:{}",
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default()
        );
        let body = record.args().to_string();
        self.log_text(record.level().into(), &from, &body);
    }

    fn flush(&self) {}
}

pub struct Gw2AlAddonDsc {
    pub name:            String,
    pub description:     String,
    pub version:         (u8, u8, u32),
    pub dependency_list: Vec<Gw2AlAddonDsc>,
}

impl From<NonNull<gw2al_addon_dsc>> for Gw2AlAddonDsc {
    fn from(raw: NonNull<gw2al_addon_dsc>) -> Self {
        let raw = unsafe { raw.as_ref() };
        let mut i = 0;
        let mut deps = Vec::new();
        loop {
            unsafe {
                let offset = raw.dependList.offset(i);
                let offset = NonNull::new(offset);
                if offset.is_none() {
                    break;
                }
                let offset = offset.unwrap();
                let obj = offset.as_ref();
                if obj.name.is_null() {
                    break;
                }
                let safe = offset.into();
                deps.push(safe);
                i += 1;
            }
        }
        Self {
            name:            unsafe { U16CStr::from_ptr_str(raw.name) }.to_string_lossy(),
            description:     unsafe { U16CStr::from_ptr_str(raw.name) }.to_string_lossy(),
            version:         (raw.majorVer, raw.minorVer, raw.revision),
            dependency_list: deps,
        }
    }
}
