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

//! Routines for controlling module-stream-restore.

use std::os::raw::{c_char, c_void};
use super::{pa_context, pa_context_success_cb_t};
use crate::{operation::pa_operation, proplist::pa_update_mode_t};
use crate::{volume::pa_cvolume, channelmap::pa_channel_map};

#[repr(C)]
pub struct pa_ext_stream_restore_info {
    pub name: *const c_char,
    pub channel_map: pa_channel_map,
    pub volume: pa_cvolume,
    pub device: *const c_char,
    pub mute: i32,
}

pub type pa_ext_stream_restore_test_cb_t = Option<extern "C" fn(c: *mut pa_context, version: u32, userdata: *mut c_void)>;

pub type pa_ext_stream_restore_read_cb_t = Option<extern "C" fn(c: *mut pa_context, info: *const pa_ext_stream_restore_info, eol: i32, userdata: *mut c_void)>;

pub type pa_ext_stream_restore_subscribe_cb_t = Option<extern "C" fn(c: *mut pa_context, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_ext_stream_restore_test(c: *mut pa_context, cb: pa_ext_stream_restore_test_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_stream_restore_read(c: *mut pa_context, cb: pa_ext_stream_restore_read_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_stream_restore_write(c: *mut pa_context, mode: pa_update_mode_t, data: *const *const pa_ext_stream_restore_info, n: u32, apply_immediately: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_stream_restore_delete(c: *mut pa_context, s: *const *const c_char, b: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_stream_restore_subscribe(c: *mut pa_context, enable: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_ext_stream_restore_set_subscribe_cb(c: *mut pa_context, cb: pa_ext_stream_restore_subscribe_cb_t, userdata: *mut c_void);
}
