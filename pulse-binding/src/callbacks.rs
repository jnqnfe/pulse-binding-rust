//! Callback handling.

// This file is part of the PulseAudio Rust language binding.
//
// Copyright (c) 2018 Lyndon Brown
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

use std;
use std::os::raw::c_void;
use std::ptr::null_mut;

/// List result instance. Fetching a list can result in a callback being fired for each list item,
/// and then once to signal that the end of the list having been reached. This is used to
/// distinguish such state to a closure callback.
pub enum ListResult<T> {
    /// List item
    Item(T),
    /// End of list reached
    End,
    /// Failure, an error occurred
    Error,
}

/// Unwraps optional callback function + data pointer tuple, wrapping the function pointer in an
/// option wrapper. Used internally in passing such parameters to an underlying C function.
///
/// Example:
///
/// ```rust,ignore
/// fn foo(cb: Option<(SuccessCb, *mut c_void)>) {
///     let (cb_f, cb_d) = ::util::unwrap_optional_callback::<SuccessCb>(cb);
///     //do something, i.e. passing cb_f and cb_d to C function
/// }
/// ```
#[inline]
pub(crate) fn unwrap_optional_callback<T>(cb: Option<(T, *mut c_void)>) -> (Option<T>, *mut c_void) {
    match cb {
        Some((f, d)) => (Some(f), d),
        None => (None, null_mut::<c_void>()),
    }
}

/// A saved multi-use callback. Closures of multi-use callbacks (those that may be called multiple
/// times) need saving, and releasing later at an appropriate time (on change of registered
/// callback, or on destruction of associated object). This is used for saving the pointer to it
/// for such deferred destruction.
pub(crate) struct MultiUseCallback<ClosureProto: ?Sized, ProxyProto> {
    saved: Option<*mut Box<ClosureProto>>,
    proxy: std::marker::PhantomData<*const ProxyProto>,
}

impl<ClosureProto: ?Sized, ProxyProto> Default for MultiUseCallback<ClosureProto, ProxyProto> {
    fn default() -> Self {
        MultiUseCallback::<ClosureProto, ProxyProto> { saved: None, proxy: std::marker::PhantomData }
    }
}

impl<ClosureProto: ?Sized, ProxyProto> MultiUseCallback<ClosureProto, ProxyProto> {
    /// Create a new instance. **Note**, an existing instance should always be overwritten with a
    /// new one, to ensure the old one is correctly freed.
    pub fn new(cb: Option<Box<ClosureProto>>) -> Self {
        match cb {
            Some(f) => MultiUseCallback::<ClosureProto, ProxyProto> {
                saved: Some(Box::into_raw(Box::new(f))),
                proxy: std::marker::PhantomData,
            },
            None => Default::default(),
        }
    }

    /// Returns callback params to give to the C API (a tuple of function pointer and data pointer).
    pub fn get_capi_params(&self, proxy: ProxyProto) -> (Option<ProxyProto>, *mut c_void) {
        match self.saved {
            Some(ref f) => (Some(proxy), *f as *mut c_void),
            None => (None, std::ptr::null_mut::<c_void>()),
        }
    }
}

impl<ClosureProto: ?Sized, ProxyProto> Drop for MultiUseCallback<ClosureProto, ProxyProto> {
    fn drop(&mut self) {
        if self.saved.is_some() {
            let _to_drop = unsafe { Box::from_raw(self.saved.unwrap()) };
        }
    }
}
