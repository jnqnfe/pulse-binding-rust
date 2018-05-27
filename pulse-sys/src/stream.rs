//! Audio streams for input, output and sample upload.

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

use std::os::raw::{c_char, c_void};

/// An opaque stream for playback or recording.
pub enum pa_stream {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_stream_state_t {
    Unconnected,
    Creating,
    Ready,
    Failed,
    Terminated,
}

pub const PA_STREAM_UNCONNECTED: pa_stream_state_t = pa_stream_state_t::Unconnected;
pub const PA_STREAM_CREATING: pa_stream_state_t = pa_stream_state_t::Creating;
pub const PA_STREAM_READY: pa_stream_state_t = pa_stream_state_t::Ready;
pub const PA_STREAM_FAILED: pa_stream_state_t = pa_stream_state_t::Failed;
pub const PA_STREAM_TERMINATED: pa_stream_state_t = pa_stream_state_t::Terminated;

/// Returns `true` if the passed state is one of the connected states.
#[inline(always)]
pub fn pa_stream_is_good(state: pa_stream_state_t) -> bool {
    state == pa_stream_state_t::Creating ||
    state == pa_stream_state_t::Ready
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_stream_direction_t {
    Invalid,
    Playback,
    Record,
    Upload,
}

pub const PA_STREAM_NODIRECTION: pa_stream_direction_t = pa_stream_direction_t::Invalid;
pub const PA_STREAM_PLAYBACK: pa_stream_direction_t = pa_stream_direction_t::Playback;
pub const PA_STREAM_RECORD: pa_stream_direction_t = pa_stream_direction_t::Record;
pub const PA_STREAM_UPLOAD: pa_stream_direction_t = pa_stream_direction_t::Upload;

pub type pa_stream_flags_t = u32;

pub use self::flags::*;

/// Some special flags for stream connections.
pub mod flags {
    use super::pa_stream_flags_t;

    pub const PA_STREAM_NOFLAGS: pa_stream_flags_t = 0x0;
    pub const PA_STREAM_START_CORKED: pa_stream_flags_t = 0x1;
    pub const PA_STREAM_INTERPOLATE_TIMING: pa_stream_flags_t = 0x2;
    pub const PA_STREAM_NOT_MONOTONIC: pa_stream_flags_t = 0x4;
    pub const PA_STREAM_AUTO_TIMING_UPDATE: pa_stream_flags_t = 0x8;
    pub const PA_STREAM_NO_REMAP_CHANNELS: pa_stream_flags_t = 0x10;
    pub const PA_STREAM_NO_REMIX_CHANNELS: pa_stream_flags_t = 0x20;
    pub const PA_STREAM_FIX_FORMAT: pa_stream_flags_t = 0x40;
    pub const PA_STREAM_FIX_RATE: pa_stream_flags_t = 0x80;
    pub const PA_STREAM_FIX_CHANNELS: pa_stream_flags_t = 0x100;
    pub const PA_STREAM_DONT_MOVE: pa_stream_flags_t = 0x200;
    pub const PA_STREAM_VARIABLE_RATE: pa_stream_flags_t = 0x400;
    pub const PA_STREAM_PEAK_DETECT: pa_stream_flags_t = 0x800;
    pub const PA_STREAM_START_MUTED: pa_stream_flags_t = 0x1000;
    pub const PA_STREAM_ADJUST_LATENCY: pa_stream_flags_t = 0x2000;
    pub const PA_STREAM_EARLY_REQUESTS: pa_stream_flags_t = 0x4000;
    pub const PA_STREAM_DONT_INHIBIT_AUTO_SUSPEND: pa_stream_flags_t = 0x8000;
    pub const PA_STREAM_START_UNMUTED: pa_stream_flags_t = 0x10000;
    pub const PA_STREAM_FAIL_ON_SUSPEND: pa_stream_flags_t = 0x20000;
    pub const PA_STREAM_RELATIVE_VOLUME: pa_stream_flags_t = 0x40000;
    pub const PA_STREAM_PASSTHROUGH: pa_stream_flags_t = 0x80000;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_seek_mode_t {
    /// Seek relatively to the write index.
    Relative = 0,
    /// Seek relatively to the start of the buffer queue.
    Absolute = 1,
    /// Seek relatively to the read index.
    RelativeOnRead = 2,
    /// Seek relatively to the current end of the buffer queue.
    RelativeEnd = 3,
}

pub const PA_SEEK_RELATIVE: pa_seek_mode_t = pa_seek_mode_t::Relative;
pub const PA_SEEK_ABSOLUTE: pa_seek_mode_t = pa_seek_mode_t::Absolute;
pub const PA_SEEK_RELATIVE_ON_READ: pa_seek_mode_t = pa_seek_mode_t::RelativeOnRead;
pub const PA_SEEK_RELATIVE_END: pa_seek_mode_t = pa_seek_mode_t::RelativeEnd;

pub const PA_STREAM_EVENT_REQUEST_CORK: &str = "request-cork";
pub const PA_STREAM_EVENT_REQUEST_UNCORK: &str = "request-uncork";
pub const PA_STREAM_EVENT_FORMAT_LOST: &str = "format-lost";

pub type pa_stream_success_cb_t = Option<extern "C" fn(s: *mut pa_stream, success: i32, userdata: *mut c_void)>;

pub type pa_stream_request_cb_t = Option<extern "C" fn(p: *mut pa_stream, nbytes: usize, userdata: *mut c_void)>;

pub type pa_stream_notify_cb_t = Option<extern "C" fn(p: *mut pa_stream, userdata: *mut c_void)>;

pub type pa_stream_event_cb_t = Option<extern "C" fn(p: *mut pa_stream, name: *const c_char, pl: *mut ::proplist::pa_proplist, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_stream_connect_upload(s: *mut pa_stream, length: usize) -> i32;
    pub fn pa_stream_finish_upload(s: *mut pa_stream) -> i32;

    pub fn pa_stream_new(c: *mut ::context::pa_context, name: *const c_char, ss: *const ::sample::pa_sample_spec, map: *const ::channelmap::pa_channel_map) -> *mut pa_stream;
    pub fn pa_stream_new_with_proplist(c: *mut ::context::pa_context, name: *const c_char, ss: *const ::sample::pa_sample_spec, map: *const ::channelmap::pa_channel_map, p: *mut ::proplist::pa_proplist) -> *mut pa_stream;
    pub fn pa_stream_new_extended(c: *mut ::context::pa_context, name: *const c_char, formats: *const *const ::format::pa_format_info, n_formats: u32, p: *mut ::proplist::pa_proplist) -> *mut pa_stream;
    pub fn pa_stream_unref(s: *mut pa_stream);
    pub fn pa_stream_ref(s: *mut pa_stream) -> *mut pa_stream;
    pub fn pa_stream_get_state(s: *mut pa_stream) -> pa_stream_state_t;
    pub fn pa_stream_get_context(s: *mut pa_stream) -> *mut ::context::pa_context;
    pub fn pa_stream_get_index(s: *mut pa_stream) -> u32;
    pub fn pa_stream_get_device_index(s: *mut pa_stream) -> u32;
    pub fn pa_stream_get_device_name(s: *mut pa_stream) -> *const c_char;
    pub fn pa_stream_is_suspended(s: *mut pa_stream) -> i32;
    pub fn pa_stream_is_corked(s: *mut pa_stream) -> i32;
    pub fn pa_stream_connect_playback(s: *mut pa_stream, dev: *const c_char, attr: *const ::def::pa_buffer_attr, flags: pa_stream_flags_t, volume: *const ::volume::pa_cvolume, sync_stream: *mut pa_stream) -> i32;
    pub fn pa_stream_connect_record(s: *mut pa_stream, dev: *const c_char, attr: *const ::def::pa_buffer_attr, flags: pa_stream_flags_t) -> i32;
    pub fn pa_stream_disconnect(s: *mut pa_stream) -> i32;
    pub fn pa_stream_begin_write(s: *mut pa_stream, data: *mut *mut c_void, nbytes: *mut usize) -> i32;
    pub fn pa_stream_cancel_write(s: *mut pa_stream) -> i32;
    pub fn pa_stream_write(s: *mut pa_stream, data: *const c_void, nbytes: usize, free_cb: ::def::pa_free_cb_t, offset: i64, seek: pa_seek_mode_t) -> i32;
    pub fn pa_stream_write_ext_free(s: *mut pa_stream, data: *const c_void, nbytes: usize, free_cb: ::def::pa_free_cb_t, free_cb_data: *mut c_void, offset: i64, seek: pa_seek_mode_t) -> i32;
    pub fn pa_stream_peek(s: *mut pa_stream, data: *mut *const c_void, nbytes: *mut usize) -> i32;
    pub fn pa_stream_drop(s: *mut pa_stream) -> i32;
    pub fn pa_stream_writable_size(s: *mut pa_stream) -> usize;
    pub fn pa_stream_readable_size(s: *mut pa_stream) -> usize;
    pub fn pa_stream_drain(s: *mut pa_stream, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_update_timing_info(s: *mut pa_stream, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_set_state_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_write_callback(s: *mut pa_stream, cb: pa_stream_request_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_read_callback(s: *mut pa_stream, cb: pa_stream_request_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_overflow_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_get_underflow_index(s: *mut pa_stream) -> i64;
    pub fn pa_stream_set_underflow_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_started_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_latency_update_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_moved_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_suspended_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_event_callback(s: *mut pa_stream, cb: pa_stream_event_cb_t, userdata: *mut c_void);
    pub fn pa_stream_set_buffer_attr_callback(s: *mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void);
    pub fn pa_stream_cork(s: *mut pa_stream, b: i32, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_flush(s: *mut pa_stream, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_prebuf(s: *mut pa_stream, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_trigger(s: *mut pa_stream, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_set_name(s: *mut pa_stream, name: *const c_char, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_get_time(s: *mut pa_stream, r_usec: *mut ::sample::pa_usec_t) -> i32;
    pub fn pa_stream_get_latency(s: *mut pa_stream, r_usec: *mut ::sample::pa_usec_t, negative: *mut i32) -> i32;
    pub fn pa_stream_get_timing_info(s: *mut pa_stream) -> *const ::def::pa_timing_info;
    pub fn pa_stream_get_sample_spec(s: *mut pa_stream) -> *const ::sample::pa_sample_spec;
    pub fn pa_stream_get_channel_map(s: *mut pa_stream) -> *const ::channelmap::pa_channel_map;
    pub fn pa_stream_get_format_info(s: *mut pa_stream) -> *mut ::format::pa_format_info;
    pub fn pa_stream_get_buffer_attr(s: *mut pa_stream) -> *const ::def::pa_buffer_attr;
    pub fn pa_stream_set_buffer_attr(s: *mut pa_stream, attr: *const ::def::pa_buffer_attr, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_update_sample_rate(s: *mut pa_stream, rate: u32, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_proplist_update(s: *mut pa_stream, mode: ::proplist::pa_update_mode_t, p: *mut ::proplist::pa_proplist, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_proplist_remove(s: *mut pa_stream, keys: *const *const c_char, cb: pa_stream_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_stream_set_monitor_stream(s: *mut pa_stream, sink_input_idx: u32) -> i32;
    pub fn pa_stream_get_monitor_stream(s: *mut pa_stream) -> u32;
}
