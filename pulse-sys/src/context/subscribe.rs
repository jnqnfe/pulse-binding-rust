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

//! Daemon introspection event subscription subsystem.

use std::os::raw::c_void;

/// The base integer type passed to the callback, from which the facility and operation components
/// can be extracted.
pub type pa_subscription_event_type_t = u32;

/// Used to express what facilities you are interested in when subscribing.
pub type pa_subscription_mask_t = u32;

/// Mask to extract facility value from the event type passed to the user callback.
pub const PA_SUBSCRIPTION_EVENT_FACILITY_MASK: pa_subscription_event_type_t = 0xf;
/// Mask to extract operation value from the event type passed to the user callback.
pub const PA_SUBSCRIPTION_EVENT_TYPE_MASK: pa_subscription_event_type_t = 0x30;

pub use self::subscription_masks::*;

/// A set of masks used for expressing which facilities you are interested in when subscribing.
pub mod subscription_masks {
    use super::pa_subscription_mask_t;

    pub const PA_SUBSCRIPTION_MASK_NULL: pa_subscription_mask_t = 0;
    pub const PA_SUBSCRIPTION_MASK_SINK: pa_subscription_mask_t = 0x1;
    pub const PA_SUBSCRIPTION_MASK_SOURCE: pa_subscription_mask_t = 0x2;
    pub const PA_SUBSCRIPTION_MASK_SINK_INPUT: pa_subscription_mask_t = 0x4;
    pub const PA_SUBSCRIPTION_MASK_SOURCE_OUTPUT: pa_subscription_mask_t = 0x8;
    pub const PA_SUBSCRIPTION_MASK_MODULE: pa_subscription_mask_t = 0x10;
    pub const PA_SUBSCRIPTION_MASK_CLIENT: pa_subscription_mask_t = 0x20;
    pub const PA_SUBSCRIPTION_MASK_SAMPLE_CACHE: pa_subscription_mask_t = 0x40;
    pub const PA_SUBSCRIPTION_MASK_SERVER: pa_subscription_mask_t = 0x80;
    /* NOTE: value '0x100' previously assigned, obsoleted */
    pub const PA_SUBSCRIPTION_MASK_CARD: pa_subscription_mask_t = 0x200;
    pub const PA_SUBSCRIPTION_MASK_ALL: pa_subscription_mask_t = 0x2ff;
}

pub use self::event_facilities::*;

/// Possible facility variants that could be extracted from an event type.
pub mod event_facilities {
    use super::pa_subscription_event_type_t;

    pub const PA_SUBSCRIPTION_EVENT_SINK: pa_subscription_event_type_t = 0;
    pub const PA_SUBSCRIPTION_EVENT_SOURCE: pa_subscription_event_type_t = 1;
    pub const PA_SUBSCRIPTION_EVENT_SINK_INPUT: pa_subscription_event_type_t = 2;
    pub const PA_SUBSCRIPTION_EVENT_SOURCE_OUTPUT: pa_subscription_event_type_t = 3;
    pub const PA_SUBSCRIPTION_EVENT_MODULE: pa_subscription_event_type_t = 4;
    pub const PA_SUBSCRIPTION_EVENT_CLIENT: pa_subscription_event_type_t = 5;
    pub const PA_SUBSCRIPTION_EVENT_SAMPLE_CACHE: pa_subscription_event_type_t = 6;
    /// Global server change, only occurring with a change operation.
    pub const PA_SUBSCRIPTION_EVENT_SERVER: pa_subscription_event_type_t = 7;
    /* NOTE: value '8' previously assigned, obsoleted */
    pub const PA_SUBSCRIPTION_EVENT_CARD: pa_subscription_event_type_t = 9;
}

pub use self::event_operations::*;

/// Possible operation variants that could be extracted from an event type.
pub mod event_operations {
    use super::pa_subscription_event_type_t;

    pub const PA_SUBSCRIPTION_EVENT_NEW: pa_subscription_event_type_t = 0;
    pub const PA_SUBSCRIPTION_EVENT_CHANGE: pa_subscription_event_type_t = 0x10;
    pub const PA_SUBSCRIPTION_EVENT_REMOVE: pa_subscription_event_type_t = 0x20;
}

/// Returns `true` if an event type `t` matches an event mask bitfield
pub fn pa_subscription_match_flags(m: pa_subscription_mask_t,
    t: pa_subscription_event_type_t) -> bool
{
    (m & (1 << (t & PA_SUBSCRIPTION_EVENT_FACILITY_MASK))) != 0
}

pub type pa_context_subscribe_cb_t = Option<extern "C" fn(c: *mut super::pa_context, t: pa_subscription_event_type_t, idx: u32, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_context_subscribe(c: *mut super::pa_context, m: pa_subscription_mask_t, cb: super::pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_subscribe_callback(c: *mut super::pa_context, cb: pa_context_subscribe_cb_t, userdata: *mut c_void);
}
