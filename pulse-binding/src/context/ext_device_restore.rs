//! Routines for controlling module-device-restore.

// This file is part of the PulseAudio Rust language binding.
//
// Copyright (c) 2017 Lyndon Brown
//
// This library is free software; you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation; either version
// 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this library;
// if not, see <http://www.gnu.org/licenses/>.

use std;
use capi;
use std::os::raw::c_void;
use std::ptr::null_mut;
use super::{ContextInternal, Context};

pub use capi::pa_ext_device_restore_info as InfoInternal;

/// Stores information about one device in the device database that is maintained by
/// module-device-manager.
#[repr(C)]
pub struct Info {
    /// Device type sink or source?
    pub dtype: ::def::Device,
    /// The device index.
    pub index: u32,
    /// How many formats do we have?
    pub n_formats: u8,
    /// An array of formats (may be `NULL` if ``n_formats == 0``).
    pub formats: *mut *mut ::format::InfoInternal,
}

impl From<Info> for InfoInternal {
    fn from(p: Info) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl From<InfoInternal> for Info {
    fn from(p: InfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

/// A wrapper object providing device restore routines to a context.
pub struct DeviceRestore {
    context: *mut ContextInternal,
    /// Used to avoid freeing the internal object when used as a weak wrapper in callbacks
    weak: bool,
}

impl Context {
    /// Returns a device restore object linked to the current context, giving access to device
    /// restore routines. See [`::context::ext_device_restore`](ext_device_restore/index.html).
    pub fn device_restore(&self) -> DeviceRestore {
        unsafe { capi::pa_context_ref(self.ptr) };
        DeviceRestore::from_raw(self.ptr)
    }
}

/// Callback prototype for [`test`](struct.DeviceRestore.html#method.test).
pub type TestCb = extern "C" fn(c: *mut ContextInternal, version: u32, userdata: *mut c_void);

/// Callback prototype for [`set_subscribe_cb`](struct.DeviceRestore.html#method.set_subscribe_cb).
pub type SubscribeCb = extern "C" fn(c: *mut ContextInternal,
    type_: ::def::Device, idx: u32, userdata: *mut c_void);

/// Callback prototype for [`read_formats`](struct.DeviceRestore.html#method.read_formats).
pub type ReadDevFormatsCb = extern "C" fn(c: *mut ContextInternal, info: *const InfoInternal,
    eol: i32, userdata: *mut c_void);

impl DeviceRestore {
    /// Create a new `DeviceManager` from an existing
    /// [`ContextInternal`](../struct.ContextInternal.html) pointer.
    fn from_raw(context: *mut ContextInternal) -> Self {
        Self { context: context, weak: false }
    }

    /// Create a new `DeviceManager` from an existing
    /// [`ContextInternal`](../struct.ContextInternal.html) pointer. This is the 'weak' version, for
    /// use in callbacks, which avoids destroying the internal object when dropped.
    pub fn from_raw_weak(context: *mut ContextInternal) -> Self {
        Self { context: context, weak: true }
    }

    /// Test if this extension module is available in the server.
    pub fn test(&mut self, cb: (TestCb, *mut c_void)) -> ::operation::Operation {
        let ptr = unsafe { capi::pa_ext_device_restore_test(self.context, Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Subscribe to changes in the device database.
    pub fn subscribe(&mut self, enable: bool, cb: (::context::ContextSuccessCb, *mut c_void)
        ) -> ::operation::Operation
    {
        let ptr = unsafe { capi::pa_ext_device_restore_subscribe(self.context, enable as i32,
            Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the subscription callback that is called when [`subscribe`](#method.subscribe) was
    /// called.
    pub fn set_subscribe_cb(&mut self, cb: (SubscribeCb, *mut c_void)) {
        unsafe { capi::pa_ext_device_restore_set_subscribe_cb(self.context, Some(cb.0), cb.1); }
    }

    /// Read the formats for all present devices from the device database.
    pub fn read_formats_all(&mut self, cb: (ReadDevFormatsCb, *mut c_void)
        ) -> ::operation::Operation
    {
        let ptr = unsafe { capi::pa_ext_device_restore_read_formats_all(self.context, Some(cb.0),
            cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Read an entry from the device database.
    pub fn read_formats(&mut self, type_: ::def::Device, idx: u32,
        cb: (ReadDevFormatsCb, *mut c_void)) -> ::operation::Operation
    {
        let ptr = unsafe { capi::pa_ext_device_restore_read_formats(self.context, type_, idx,
            Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Read an entry from the device database.
    pub fn save_formats(&mut self, type_: ::def::Device, idx: u32,
        formats: &mut [&mut ::format::Info], cb: (::context::ContextSuccessCb, *mut c_void)
        ) -> ::operation::Operation
    {
        // Capture array of pointers to the above ::format::InfoInternal objects
        let mut format_ptrs: Vec<*mut capi::pa_format_info> = Vec::with_capacity(formats.len());
        for format in formats {
            format_ptrs.push(unsafe { std::mem::transmute(&format.ptr) });
        }

        let ptr = unsafe {
            capi::pa_ext_device_restore_save_formats(self.context, type_, idx,
                format_ptrs.len() as u8, format_ptrs.as_ptr(), Some(cb.0), cb.1)
        };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

impl Drop for DeviceRestore {
    fn drop(&mut self) {
        if !self.weak {
            unsafe { capi::pa_context_unref(self.context) };
        }
        self.context = null_mut::<ContextInternal>();
    }
}
