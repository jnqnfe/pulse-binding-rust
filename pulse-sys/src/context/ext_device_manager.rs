// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
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

//! Routines for controlling module-device-manager.

use std::os::raw::{c_char, c_void};
use super::{pa_context, pa_context_success_cb_t};
use crate::operation::pa_operation;

#[repr(C)]
pub struct pa_ext_device_manager_role_priority_info {
    pub role: *const c_char,
    pub priority: u32,
}

#[repr(C)]
pub struct pa_ext_device_manager_info {
    pub name: *const c_char,
    pub description: *const c_char,
    pub icon: *const c_char,
    pub index: u32,
    pub n_role_priorities: u32,
    pub role_priorities: *mut pa_ext_device_manager_role_priority_info,
}

pub type pa_ext_device_manager_test_cb_t = Option<extern "C" fn(c: *mut pa_context, version: u32, userdata: *mut c_void)>;

pub type pa_ext_device_manager_read_cb_t = Option<extern "C" fn(c: *mut pa_context, info: *const pa_ext_device_manager_info, eol: i32, userdata: *mut c_void)>;

pub type pa_ext_device_manager_subscribe_cb_t = Option<extern "C" fn(c: *mut pa_context, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_ext_device_manager_test(c: *mut pa_context, cb: pa_ext_device_manager_test_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_device_manager_read(c: *mut pa_context, cb: pa_ext_device_manager_read_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_device_manager_set_device_description(c: *mut pa_context, device: *const c_char, description: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_device_manager_delete(c: *mut pa_context, s: *const *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_device_manager_enable_role_device_priority_routing(c: *mut pa_context, enable: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_device_manager_reorder_devices_for_role(c: *mut pa_context, role: *const c_char, devices: *const *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_device_manager_subscribe(c: *mut pa_context, enable: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_device_manager_set_subscribe_cb(c: *mut pa_context, cb: pa_ext_device_manager_subscribe_cb_t, userdata: *mut c_void);
}
