//! Routines for controlling module-device-manager.

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

use capi;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr::{null, null_mut};
use super::{ContextInternal, Context};

pub use capi::pa_ext_device_manager_role_priority_info as RolePriorityInfo;
pub use capi::pa_ext_device_manager_info as InfoInternal;

/// Stores information about one device in the device database that is maintained by
/// module-device-manager.
#[repr(C)]
pub struct Info {
    /// Identifier string of the device. A string like "sink:" or similar followed by the name of
    /// the device.
    pub name: *const c_char,
    /// The description of the device when it was last seen, if applicable and saved.
    pub description: *const c_char,
    /// The icon given to the device.
    pub icon: *const c_char,
    /// The device index if it is currently available or
    /// [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub index: u32,
    /// How many role priorities do we have?
    pub n_role_priorities: u32,
    /// An array of role priority structures or `NULL`.
    pub role_priorities: *mut RolePriorityInfo,
}

/// A wrapper object providing device manager routines to a context.
pub struct DeviceManager {
    context: *mut ContextInternal,
    /// Used to avoid freeing the internal object when used as a weak wrapper in callbacks
    weak: bool,
}

impl Context {
    /// Returns a device manager object linked to the current context, giving access to device
    /// manager routines. See [`::context::ext_device_manager`](ext_device_manager/index.html).
    pub fn device_manager(&self) -> DeviceManager {
        unsafe { capi::pa_context_ref(self.ptr) };
        DeviceManager::from_raw(self.ptr)
    }
}

/// Callback prototype for [`test`](struct.DeviceManager.html#method.test)
pub type TestCb = extern "C" fn(c: *mut ContextInternal, version: u32, userdata: *mut c_void);

/// Callback prototype for [`read`](struct.DeviceManager.html#method.read).
pub type ReadCb = extern "C" fn(c: *mut ContextInternal, info: *const InfoInternal, eol: i32,
    userdata: *mut c_void);

/// Callback prototype for [`set_subscribe_cb`](struct.DeviceManager.html#method.set_subscribe_cb).
pub type SubscribeCb = extern "C" fn(c: *mut ContextInternal, userdata: *mut c_void);

impl DeviceManager {
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
    pub fn test(&mut self, cb: (TestCb, *mut c_void)) -> Option<::operation::Operation> {
        let ptr = unsafe { capi::pa_ext_device_manager_test(self.context, Some(cb.0), cb.1) };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Read all entries from the device database.
    pub fn read(&mut self, cb: (ReadCb, *mut c_void)) -> Option<::operation::Operation> {
        let ptr = unsafe {  capi::pa_ext_device_manager_read(self.context, Some(cb.0), cb.1) };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Sets the description for a device.
    pub fn set_device_description(&mut self, device: &str, description: &str,
        cb: (::context::ContextSuccessCb, *mut c_void)) -> Option<::operation::Operation>
    {
        // Warning: New CStrings will be immediately freed if not bound to a
        // variable, leading to as_ptr() giving dangling pointers!
        let c_dev = CString::new(device.clone()).unwrap();
        let c_desc = CString::new(description.clone()).unwrap();
        let ptr = unsafe {
            capi::pa_ext_device_manager_set_device_description(self.context, c_dev.as_ptr(),
                c_desc.as_ptr(), Some(cb.0), cb.1)
        };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Delete entries from the device database.
    pub fn delete(&mut self, devices: &[&str], cb: (::context::ContextSuccessCb, *mut c_void)
        ) -> Option<::operation::Operation>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let mut c_devs: Vec<CString> = Vec::with_capacity(devices.len());
        for device in devices {
            c_devs.push(CString::new(device.clone()).unwrap());
        }

        // Capture array of pointers to the above CString values.
        // We also add a `NULL` pointer entry on the end, as expected by the C function called here.
        let mut c_dev_ptrs: Vec<*const c_char> = Vec::with_capacity(c_devs.len()+1);
        for c_dev in c_devs {
            c_dev_ptrs.push(c_dev.as_ptr());
        }
        c_dev_ptrs.push(null());

        let ptr = unsafe { capi::pa_ext_device_manager_delete(self.context, c_dev_ptrs.as_ptr(),
            Some(cb.0), cb.1) };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Enable the role-based device-priority routing mode.
    pub fn enable_role_device_priority_routing(&mut self, enable: bool,
        cb: (::context::ContextSuccessCb, *mut c_void)) -> Option<::operation::Operation>
    {
        let ptr = unsafe {
            capi::pa_ext_device_manager_enable_role_device_priority_routing(self.context,
                enable as i32, Some(cb.0), cb.1)
        };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Prefer a given device in the priority list.
    pub fn reorder_devices_for_role(&mut self, role: &str, devices: &[&str],
        cb: (::context::ContextSuccessCb, *mut c_void)) -> Option<::operation::Operation>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_role = CString::new(role.clone()).unwrap();
        let mut c_devs: Vec<CString> = Vec::with_capacity(devices.len());
        for device in devices {
            c_devs.push(CString::new(device.clone()).unwrap());
        }

        // Capture array of pointers to the above CString values.
        // We also add a `NULL` pointer entry on the end, as expected by the C function called here.
        let mut c_dev_ptrs: Vec<*const c_char> = Vec::with_capacity(c_devs.len() + 1);
        for c_dev in c_devs {
            c_dev_ptrs.push(c_dev.as_ptr());
        }
        c_dev_ptrs.push(null());

        let ptr = unsafe {
            capi::pa_ext_device_manager_reorder_devices_for_role(self.context, c_role.as_ptr(),
                c_dev_ptrs.as_ptr(), Some(cb.0), cb.1)
        };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Subscribe to changes in the device database.
    pub fn subscribe(&mut self, enable: bool, cb: (::context::ContextSuccessCb, *mut c_void)
        ) -> Option<::operation::Operation>
    {
        let ptr = unsafe { capi::pa_ext_device_manager_subscribe(self.context, enable as i32,
            Some(cb.0), cb.1) };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Set the subscription callback that is called when [`subscribe`](#method.subscribe) was
    /// called.
    pub fn set_subscribe_cb(&mut self, cb: (SubscribeCb, *mut c_void)) {
        unsafe { capi::pa_ext_device_manager_set_subscribe_cb(self.context, Some(cb.0), cb.1) };
    }
}

impl Drop for DeviceManager {
    fn drop(&mut self) {
        if !self.weak {
            unsafe { capi::pa_context_unref(self.context) };
        }
        self.context = null_mut::<ContextInternal>();
    }
}
