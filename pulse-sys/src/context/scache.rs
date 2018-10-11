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

//! Sample cache mechanism.

use std::os::raw::{c_char, c_void};

pub type pa_context_play_sample_cb_t = Option<extern "C" fn(c: *mut super::pa_context, idx: u32, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_context_remove_sample(c: *mut super::pa_context, name: *const c_char, cb: super::pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_play_sample(c: *mut super::pa_context, name: *const c_char, dev: *const c_char, volume: ::volume::pa_volume_t, cb: super::pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_play_sample_with_proplist(c: *mut super::pa_context, name: *const c_char, dev: *const c_char, volume: ::volume::pa_volume_t, proplist: *const ::proplist::pa_proplist, cb: pa_context_play_sample_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
}
