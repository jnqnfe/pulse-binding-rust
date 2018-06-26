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
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use std::os::raw::{c_char, c_void};
use std::ptr::{null, null_mut};
use super::{ContextInternal, Context};
use callbacks::{ListResult, box_closure_get_capi_ptr};
use capi::pa_ext_device_manager_info as InfoInternal;
use capi::pa_ext_device_manager_role_priority_info as RolePriorityInfoInternal;

#[derive(Debug)]
pub struct RolePriorityInfo<'a> {
    pub role: Option<Cow<'a, str>>,
    pub priority: u32,
}

impl<'a> RolePriorityInfo<'a> {
    fn new_from_raw(p: *const RolePriorityInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            RolePriorityInfo {
                role: match src.role.is_null() {
                    false => Some(CStr::from_ptr(src.role).to_string_lossy()),
                    true => None,
                },
                priority: src.priority,
            }
        }
    }
}

/// Stores information about one device in the device database that is maintained by
/// module-device-manager.
#[derive(Debug)]
pub struct Info<'a> {
    /// Identifier string of the device. A string like "sink:" or similar followed by the name of
    /// the device.
    pub name: Option<Cow<'a, str>>,
    /// The description of the device when it was last seen, if applicable and saved.
    pub description: Option<Cow<'a, str>>,
    /// The icon given to the device.
    pub icon: Option<Cow<'a, str>>,
    /// The device index if it is currently available or `None` if invalid.
    pub index: Option<u32>,
    /// A set of role priority structures.
    pub role_priorities: Vec<RolePriorityInfo<'a>>,
}

impl<'a> Info<'a> {
    fn new_from_raw(p: *const InfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };

        let mut rp_vec = Vec::with_capacity(src.n_role_priorities as usize);
        assert!(src.n_role_priorities == 0 || !src.role_priorities.is_null());
        for i in 0..src.n_role_priorities as isize {
            let indexed_ptr = unsafe { src.role_priorities.offset(i) as *mut RolePriorityInfoInternal };
            if !indexed_ptr.is_null() {
                rp_vec.push(RolePriorityInfo::new_from_raw(indexed_ptr));
            }
        }

        unsafe {
            Info {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                description: match src.description.is_null() {
                    false => Some(CStr::from_ptr(src.description).to_string_lossy()),
                    true => None,
                },
                icon: match src.icon.is_null() {
                    false => Some(CStr::from_ptr(src.icon).to_string_lossy()),
                    true => None,
                },
                index: match src.index {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                role_priorities: rp_vec,
            }
        }
    }
}

/// A wrapper object providing device manager routines to a context.
/// Note: Saves a copy of active multi-use closure callbacks, which it frees on drop.
pub struct DeviceManager {
    context: *mut ContextInternal,
    /// Multi-use callback closure pointers
    cb_ptrs: CallbackPointers,
}

/// Holds copies of callback closure pointers, for those that are "multi-use" (may be fired multiple
/// times), for freeing at the appropriate time.
#[derive(Default)]
struct CallbackPointers {
    subscribe: super::ExtSubscribeCb,
}

impl Context {
    /// Returns a device manager object linked to the current context, giving access to device
    /// manager routines. See [`::context::ext_device_manager`](ext_device_manager/index.html).
    pub fn device_manager(&self) -> DeviceManager {
        unsafe { capi::pa_context_ref(self.ptr) };
        DeviceManager::from_raw(self.ptr)
    }
}

impl DeviceManager {
    /// Create a new `DeviceManager` from an existing
    /// [`ContextInternal`](../struct.ContextInternal.html) pointer.
    fn from_raw(context: *mut ContextInternal) -> Self {
        Self { context: context, cb_ptrs: Default::default() }
    }

    /// Test if this extension module is available in the server.
    pub fn test<F>(&mut self, callback: F) -> ::operation::Operation
        where F: FnMut(u32) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(u32)>(Box::new(callback));
        let ptr = unsafe { capi::pa_ext_device_manager_test(self.context,
            Some(super::ext_test_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Read all entries from the device database.
    pub fn read<F>(&mut self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<&Info>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&Info>)>(Box::new(callback));
        let ptr = unsafe {  capi::pa_ext_device_manager_read(self.context, Some(read_list_cb_proxy),
            cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Sets the description for a device.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn set_device_description<F>(&mut self, device: &str, description: &str, callback: F
        ) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a
        // variable, leading to as_ptr() giving dangling pointers!
        let c_dev = CString::new(device.clone()).unwrap();
        let c_desc = CString::new(description.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe {
            capi::pa_ext_device_manager_set_device_description(self.context, c_dev.as_ptr(),
                c_desc.as_ptr(), Some(super::success_cb_proxy), cb_data)
        };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Delete entries from the device database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn delete<F>(&mut self, devices: &[&str], callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
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

        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_ext_device_manager_delete(self.context, c_dev_ptrs.as_ptr(),
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Enable the role-based device-priority routing mode.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn enable_role_device_priority_routing<F>(&mut self, enable: bool, callback: F
        ) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe {
            capi::pa_ext_device_manager_enable_role_device_priority_routing(self.context,
                enable as i32, Some(super::success_cb_proxy), cb_data)
        };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Prefer a given device in the priority list.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn reorder_devices_for_role<F>(&mut self, role: &str, devices: &[&str], callback: F
        ) -> ::operation::Operation
        where F: FnMut(bool) + 'static
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

        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe {
            capi::pa_ext_device_manager_reorder_devices_for_role(self.context, c_role.as_ptr(),
                c_dev_ptrs.as_ptr(), Some(super::success_cb_proxy), cb_data)
        };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Subscribe to changes in the device database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn subscribe<F>(&mut self, enable: bool, callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_ext_device_manager_subscribe(self.context, enable as i32,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the subscription callback that is called when [`subscribe`](#method.subscribe) was
    /// called.
    pub fn set_subscribe_cb<F>(&mut self, callback: F)
        where F: FnMut() + 'static
    {
        let saved = &mut self.cb_ptrs.subscribe;
        *saved = super::ExtSubscribeCb::new(Some(Box::new(callback)));
        let (cb_fn, cb_data) = saved.get_capi_params(super::ext_subscribe_cb_proxy);
        unsafe { capi::pa_ext_device_manager_set_subscribe_cb(self.context, cb_fn, cb_data) };
    }
}

impl Drop for DeviceManager {
    fn drop(&mut self) {
        unsafe { capi::pa_context_unref(self.context) };
        self.context = null_mut::<ContextInternal>();
    }
}

/// Proxy for read list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn read_list_cb_proxy(_: *mut ContextInternal, i: *const InfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe { &mut *(userdata as *mut Box<FnMut(ListResult<&Info>)>) };
            let obj = Info::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<&Info>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}
