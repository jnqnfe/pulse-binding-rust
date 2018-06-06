//! Daemon introspection event subscription subsystem.

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

//! # Overview
//!
//! The application can be notified, asynchronously, whenever the internal layout of the server
//! changes. The set of facilities and operations for which notifications are generated are
//! enumerated in [`Facility`] and [`Operation`].
//!
//! The application sets the notification mask using [`::context::Context::subscribe`] and the
//! callback function that will be called whenever a notification occurs using
//! [`::context::Context::set_subscribe_callback`].
//!
//! The mask provided to [`::context::Context::subscribe`] can be created by binary ORing a set of
//! values produced with [`facility_to_mask`](fn.facility_to_mask.html).
//!
//! The callback will be called with an [`EventType`] representing the event that caused the
//! callback. Clients can examine what type of object (facility) changed using [`get_facility`]. The
//! actual event type can then be extracted with [`get_operation`].
//!
//! # Example
//!
//! Subscribe (declare interest):
//!
//! ```rust,ignore
//! use pulse::context::subscribe::subscription_masks;
//!
//! let interest = subscription_masks::SINK |
//!     subscription_masks::SOURCE;
//!
//! let op = my_context.subscribe(
//!     interest,   // Our interest mask
//!     None        // We won't bother with a success callback in this example
//! );
//! ```
//!
//! A callback:
//!
//! ```rust,ignore
//! use std::os::raw::c_void;
//! use pulse::context::ContextInternal;
//! use pulse::context::subscribe::*;
//!
//! extern "C"
//! fn my_subscription_callback(
//!     _: *mut ContextInternal, // Ignoring context pointer
//!     t: EventType,            // The combined facility and operation
//!     _: u32,                  // Ignoring index
//!     _: *mut c_void)          // Ignoring userdata pointer
//! {
//!     if get_facility(t).unwrap() == Facility::Source &&
//!        get_operation(t).unwrap() == Operation::New
//!     {
//!         //... a source was added, let's do stuff! ...
//!     }
//! }
//! ```
//!
//! [`Facility`]: enum.Facility.html
//! [`Operation`]: enum.Operation.html
//! [`EventType`]: type.EventType.html
//! [`facility_to_mask`]: fn.facility_to_mask.html
//! [`get_facility`]: fn.get_facility.html
//! [`get_operation`]: fn.get_operation.html
//! [`::context::Context::subscribe`]: ../struct.Context.html#method.subscribe
//! [`::context::Context::set_subscribe_callback`]: ../struct.Context.html#method.set_subscribe_callback

use capi;
use std::os::raw::c_void;
use super::{ContextInternal, Context, ContextSuccessCb};
use ::util::unwrap_optional_callback;

pub use capi::context::subscribe::pa_subscription_event_type_t as EventType;
pub use capi::PA_SUBSCRIPTION_EVENT_FACILITY_MASK as FACILITY_MASK;
pub use capi::PA_SUBSCRIPTION_EVENT_TYPE_MASK as OPERATION_MASK;

/// A set of facility masks, passed to
/// [`Context::subscribe`](../struct.Context.html#method.subscribe). Convert a
/// [`Facility`](enum.Facility.html) to a mask with [`facility_to_mask`](fn.facility_to_mask.html).
pub type InterestMaskSet = capi::context::subscribe::pa_subscription_mask_t;

/// A set of masks used for expressing which facilities you are interested in when subscribing.
pub mod subscription_masks {
    use capi;
    use super::InterestMaskSet;

    pub const NULL: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_NULL;
    pub const SINK: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_SINK;
    pub const SOURCE: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_SOURCE;
    pub const SINK_INPUT: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_SINK_INPUT;
    pub const SOURCE_OUTPUT: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_SOURCE_OUTPUT;
    pub const MODULE: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_MODULE;
    pub const CLIENT: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_CLIENT;
    pub const SAMPLE_CACHE: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_SAMPLE_CACHE;
    pub const SERVER: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_SERVER;
    pub const AUTOLOAD: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_AUTOLOAD;
    pub const MASK_CARD: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_CARD;
    pub const ALL: InterestMaskSet = capi::PA_SUBSCRIPTION_MASK_ALL;
}

/// Facility variants for an `EventType`.
/// You can extract the facility portion of the `EventType` value using
/// [`get_facility`](fn.get_facility.html).
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Facility {
    Sink = 0,
    Source = 1,
    SinkInput = 2,
    SourceOutput = 3,
    Module = 4,
    Client = 5,
    SampleCache = 6,
    /// Global server change, only occurring with
    /// [`Operation::Changed`](enum.Operation.html#Changed.v).
    Server = 7,
    /// Autoload table changes.
    #[deprecated]
    AutoLoad = 8,
    Card = 9,
}

/// Operation variants for an `EventType`.
/// You can extract the operation portion of the `EventType` value using
/// [`get_operation`](fn.get_operation.html).
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operation {
    /// A new object was created
    New = 0,
    /// A property of the object was modified
    Changed = 0x10,
    /// An object was removed
    Removed = 0x20,
}

impl Facility {
    pub fn from_int(value: u32) -> Option<Facility> {
        match value {
            0 => Some(Facility::Sink),
            1 => Some(Facility::Source),
            2 => Some(Facility::SinkInput),
            3 => Some(Facility::SourceOutput),
            4 => Some(Facility::Module),
            5 => Some(Facility::Client),
            6 => Some(Facility::SampleCache),
            7 => Some(Facility::Server),
            8 => {
                #[allow(deprecated)]
                Some(Facility::AutoLoad)
            },
            9 => Some(Facility::Card),
            _ => None,
        }
    }
}

impl Operation {
    pub fn from_int(value: u32) -> Option<Operation> {
        match value {
            0 => Some(Operation::New),
            0x10 => Some(Operation::Changed),
            0x20 => Some(Operation::Removed),
            _ => None,
        }
    }
}

/// Convert facility to mask
pub fn facility_to_mask(f: Facility) -> InterestMaskSet {
    1u32 << (f as u32)
}

/// Combine facility and operation to form an `EventType` value.
pub fn make_eventtype(f: Facility, o: Operation) -> EventType {
    (f as EventType) | (o as EventType)
}

/// Extract facility from `EventType` value
pub fn get_facility(value: EventType) -> Option<Facility> {
    Facility::from_int((value & FACILITY_MASK) as u32)
}

/// Extract operation from `EventType` value
pub fn get_operation(value: EventType) -> Option<Operation> {
    Operation::from_int((value & OPERATION_MASK) as u32)
}

/// Subscription event callback prototype
pub type Callback = extern "C" fn(c: *mut ContextInternal, t: EventType, idx: u32,
    userdata: *mut c_void);

impl Context {
    /// Enable event notification.
    /// The `mask` parameter is used to specify which facilities you are interested in being
    /// modified about. Use [`set_subscribe_callback`](#method.set_subscribe_callback) to set the
    /// actual callback that will be called when an event occurs.
    pub fn subscribe(&mut self, mask: InterestMaskSet, cb: (ContextSuccessCb, *mut c_void)
        ) -> ::operation::Operation
    {
        let ptr = unsafe { capi::pa_context_subscribe(self.ptr, mask, Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the context specific call back function that is called whenever a subscribed-to event
    /// occurs. Use [`subscribe`](#method.subscribe) to set the facilities you are interested in
    /// recieving notifications for, and thus to start receiving notifications with the callback set
    /// here.
    pub fn set_subscribe_callback(&mut self, cb: Option<(Callback, *mut c_void)>) {
        let (cb_f, cb_d) = unwrap_optional_callback::<Callback>(cb);
        unsafe { capi::pa_context_set_subscribe_callback(self.ptr, cb_f, cb_d); }
    }
}
