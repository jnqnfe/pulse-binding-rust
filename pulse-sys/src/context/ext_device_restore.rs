//! Routines for controlling module-device-restore.

// This file is part of the PulseAudio Rust language linking library.
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

use std::os::raw::c_void;
use super::{pa_context, pa_context_success_cb_t};

#[repr(C)]
pub struct pa_ext_device_restore_info {
    pub dtype: ::def::pa_device_type_t,
    pub index: u32,
    pub n_formats: u8,
    pub formats: *mut *mut ::format::pa_format_info,
}

pub type pa_ext_device_restore_test_cb_t = Option<extern "C" fn(c: *mut pa_context, version: u32, userdata: *mut c_void)>;

pub type pa_ext_device_restore_subscribe_cb_t = Option<extern "C" fn(c: *mut pa_context, type_: ::def::pa_device_type_t, idx: u32, userdata: *mut c_void)>;

pub type pa_ext_device_restore_read_device_formats_cb_t = Option<extern "C" fn(c: *mut pa_context, info: *const pa_ext_device_restore_info, eol: i32, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_ext_device_restore_test(c: *mut pa_context, cb: pa_ext_device_restore_test_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_ext_device_restore_subscribe(c: *mut pa_context, enable: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_ext_device_restore_set_subscribe_cb(c: *mut pa_context, cb: pa_ext_device_restore_subscribe_cb_t, userdata: *mut c_void);
    pub fn pa_ext_device_restore_read_formats_all(c: *mut pa_context, cb: pa_ext_device_restore_read_device_formats_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_ext_device_restore_read_formats(c: *mut pa_context, type_: ::def::pa_device_type_t, idx: u32, cb: pa_ext_device_restore_read_device_formats_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_ext_device_restore_save_formats(c: *mut pa_context, type_: ::def::pa_device_type_t, idx: u32, n_formats: u8, formats: *const *mut ::format::pa_format_info, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
}
