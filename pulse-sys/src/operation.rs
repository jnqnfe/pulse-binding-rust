// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Asynchronous operations.

use std::os::raw::c_void;

/// An asynchronous operation object.
#[repr(C)] pub struct pa_operation { _private: [u8; 0] }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_operation_state_t {
    /// The operation is still running.
    Running,
    /// The operation has completed.
    Done,
    /// The operation has been cancelled. Operations may get cancelled by the application, or as a
    /// result of the context getting disconnected while the operation is pending.
    Cancelled,
}

pub const PA_OPERATION_RUNNING: pa_operation_state_t = pa_operation_state_t::Running;
pub const PA_OPERATION_DONE: pa_operation_state_t = pa_operation_state_t::Done;
pub const PA_OPERATION_CANCELED: pa_operation_state_t = pa_operation_state_t::Cancelled;
pub const PA_OPERATION_CANCELLED: pa_operation_state_t = pa_operation_state_t::Cancelled;

/// A callback for operation state changes.
pub type pa_operation_notify_cb_t = Option<extern "C" fn(o: *mut pa_operation, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_operation_ref(o: *mut pa_operation) -> *mut pa_operation;
    pub fn pa_operation_unref(o: *mut pa_operation);
    pub fn pa_operation_cancel(o: *mut pa_operation);
    pub fn pa_operation_get_state(o: *const pa_operation) -> pa_operation_state_t;
    pub fn pa_operation_set_state_callback(o: *mut pa_operation, cb: pa_operation_notify_cb_t, userdata: *mut c_void);
}
