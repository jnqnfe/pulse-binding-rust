# <unreleased>

 * Fixed issues with a threaded mainloop example in the documentation

# 2.15.0 (December 29th, 2019)

 * Updated `libpulse-sys` version dependency (1.11 → 1.12)

# 2.14.1 (December 29th, 2019)

 * Fixed issues compiling on Windows
   - Needed to reference a different `pollfd` definition
   - Needed to reference different inner `timeval` attribute type definitions
   Thanks to @allquixotic on github for reporting

# 2.14.0 (October 28th, 2019)

 * Reverted "Changed the `mainloop` param of `Context::rttime_new` from trait object to generic
   (`dyn` to`impl`)" from v2.7. Failed to test sufficiently. This introduces an E0632 error in a
   test app and I was unsuccessful in finding a compilable workaround.

# 2.13.0 (September 17th, 2019)

 * Changed the license model from LGPL to dual MIT and Apache-2.0.
   See <https://github.com/jnqnfe/pulse-binding-rust/issues/26> for details.
 * Updated `libpulse-sys` version dependency (1.10 → 1.11)

# 2.12.0 (September 15th, 2019)

 * Implemented PA v13 enhancements, including:
    - Added `get_sample_format()`, `get_rate()`, `get_channel_count()` and `get_channel_map()` to
      `format::Info`.
    - Added `util::make_thread_realtime()`
    - Added `Encoding::TRUEHD_IEC61937` and `Encoding::DTSHD_IEC61937`
   A wrapper for `pa_threaded_mainloop_once_unlocked` has not been added at this time.
 * Changed the `ss` param of `Context::get_tile_size()` to add an `Option` wrapper.
   The C API function allows a null pointer to be used for this param, which was not a use case
   allowed in the binding, for no particular reason. This enables it.
 * Added PA v13 compatibility control feature
 * Updated `libpulse-sys` version dependency (1.9 → 1.10)

# 2.11.1 (August 19th, 2019)

 * Fixed broken doc.rs documentation generation

# 2.11.0 (August 19th, 2019)

 * Extended support to even older versions of PA, specifically up to and including v4
 * Version: Revised `Compatibility` variants to make more sense
 * Simplified feature flags, old ones left as temorary aliases, to be removed later
 * Added a `dox` feature flag, for use with `cargo doc`.
   It enables the very latest PA version compatibility, while bypassing the pkg-config check, thus
   is useful for generating documentation that includes information on features from versions of PA
   that may be newer than the version you have installed.
 * Version: improved documentation further
 * Updated `libpulse-sys` version dependency (1.8 → 1.9)

# 2.10.1 (August 17th, 2019)

 * Version: Improved mod documentation
 * Added missing version info to deprecation notices

# 2.10.0 (August 15th, 2019)

 * Modified `ChannelVolumes` and `channelmap::Map` to be more Rust-like.
   Both of these types hold an array of size `sample::CHANNELS_MAX`, along with a `channels`
   attribute which controls how much of the initial portion of that array is “active” (equivalent to
   the `len()` of a `Vec`). Previously the array and len were simply public attributes, with changes
   to be applied directly. While for the time being they remain publically accessible for backwards
   compatibility, this will become private in a future release. These types should now be used in
   a way more similar to a `Vec`:
    - New methods `len()` and `set_len()` get read/write access to the `channels` attribute that
      records how much of the array is considered “active”.
    - Traits `Borrow<[_]>` and `BorrowMut<[_]>` have been implemented, along with the addition of
      new methods `get()` and `get_mut()` (for convenience - you can avoid type abiguity), for
      accessing the array as a slice (of just the “active” portion).
 * Updated `use` conventions to that of Rust 1.30/1.31
 * Specified edition in toml file
 * Made the following `const` functions:
    - `version::get_compatibility()`
    - `Volume::is_valid()`
    - `Timeval::new()` and `Timeval::new_zero()`
    - `subscribe::Facility::to_interest_mask()`
 * Version: purged deprecated items
 * Updated `libpulse-sys` version dependency (1.6 → 1.8)

Note: versions 2.8 and 2.9 skipped, used only for `libpulse-glib-binding` crate changes

# 2.7.1 (August 13th, 2019)

 * Fixed overlooked use of new method names in docs

# 2.7.0 (August 12th, 2019)

 * Renamed a few methods of `Proplist` for clarity: `sets()` → `set_str()`; `gets()` → `get_str()`;
   and `setp()` → `set_pl()`.
 * Added `Stream::write_copy()` which is just a simplified interface for asking PA to make an
   internal copy of the to-be-written data (same as providing `None` in the `free_cb` param of
   `Stream::write()`.
 * Changed the `mainloop` param of `Context::rttime_new` from trait object to generic (`dyn` to
   `impl`).
 * Deprecated `ChannelMap::is_equal_to()`, `ChannelVolumes::equal_to()`,
   `ChannelVolumes::channels_equal_to()`, `Spec::equal_to()` and `Proplist::equal_to()` methods in
   favour of `PartialEq` implementations.
 * Added `PartialEq` implementation for `Proplist` and `PartialEq<Volume>` impl for `ChannelVolumes`
 * Changed `PartialEq` implementations for `channelmap::Map`, `Spec` and `ChannelVolumes` to
   delegate the logic to the C API.
 * Removed stray `repr(C)` attribute from `SinkPortInfo` introspection type.
 * Added a new `latest_pa_common_compatibility` feature flag, used by default now instead of
   `latest_pa_compatibility`.
 * Added tests to assert that size and alignment of certain structs and enums are identical to their
   `sys` crate counterparts.
 * Updated `libpulse-sys` version dependency (1.5 → 1.6)

# 2.6.0 (March 10th, 2019)

 * Implemented use of `std::panic::catch_unwind` in callbacks

# 2.5.0 (December 22nd, 2018)

**Note: This includes a security fix!**

 * Proplist: Fixed potential use-after-free with `proplist::Iterator` (not to be confused with the
   `std::iter::Iterator` trait). An instance of this object type is created from a `Proplist` object
   and holds a copy of the same raw pointer to the underlying C object; the `Proplist` object had
   sole responsibility for destroying it via its `Drop` implementation. There was no actual lifetime
   association however linking the lifetime of the `Iterator` object to the `Proplist` object, and
   thus it was possible for the `Proplist` object to be destroyed first, leaving the `Iterator`
   object working on a freed C object. This is unlikely to have been done in actual user code, but
   would have been trivial to achieve, including simply by using the `into_iter()` function. This
   affects versions all the way back to 1.0.5.
 * Enabled `Send`+`Sync` for various types.
   This was previously not done due to uncertainty as to whether or not it was safe to do so, but I
   have now reconsidered it and arrived at the conclusion that it should be okay: With the threaded
   mainloop, a lock must be held when using objects; this is taken by the mainloop dispatcher when
   executing callbacks, and otherwise must be taken by the user before using objects within any
   thread. With that locking mechanism, it should be safe I presume for these objects to be marked
   Send+Sync. It is not ideal that the user can so easily just forget to grab the lock, as opposed
   to the Rust design of `Arc<Mutex<_>>` forcing unlocking to get at things, but it’s not certain
   that we can easily really do anything to address this. So long as users stick to the principle of
   grabbing the mainloop lock though, they should be fine.
 * Time: Simplified converting `Duration` to `MicroSeconds` or `Timeval` using
   `Duration::subsec_millis()`.
 * Proplist: Made `Iterator::new()` private, since it’s very unlikely anyone needs it
 * Added the new `latest_pa_compatibility` and `pa_v12_compatibility` feature flags, and deprecated
   `pa_encoding_from_string` in favour of `pa_v12_compatibility`.
 * Format/Info: Removed unnecessary `From` conversion implementation to/from C type
 * Updated `libpulse-sys` version dependency (1.4 → 1.5)

# 2.4.0 (November 28th, 2018)

 * Channelmap/Map: Changed the `new_from_string` method return value to use a `Result` wrapper.
   Previously failure was just ignored, expecting strings provided to always be valid, as obtained
   from the `print` and `to_name` methods, but let's be more cautious.
 * Updated out-of-date return info for a large number of functions, particularly introspection
   ones, which had not been updated following the removal of the `Option` wrapper back in v2.0.
   Thanks to @0xpr03 on github for noticing a discrepancy.
 * Restored the `Option` wrapper to the `Context::drain` return value. It was incorrectly removed
   from this function at the same time as legitimately being removed from many others.
 * Changed the `Drop` implementation on `Stream` to no longer unwrap the `Result` returned by the
   `disconnect()` attempt it makes. This should fix the problem encountered by @futpib on github,
   reported here: https://github.com/jnqnfe/pulse-binding-rust/issues/11

# 2.3.0 (November 4th, 2018)

 * Improved the `version` mod:
    - Constants now vary depending upon backwards compatibility flags, correctly indicating the
      newest supported PA version.
    - Added the `Compatibility` enum and `get_compatibility` function
    - Renamed `BINDING_TARGET_VERSION` to `TARGET_VERSION_STRING`
    - Introduced `TARGET_VERSION` and deprecated `MAJOR`, `MINOR` and `MICRO`
    - Deprecated `get_headers_version`
 * Clarified PA version compatibility in `version` mod
 * Clarified `pa_encoding_from_string` feature purpose
 * Updated `libpulse-sys` version dependency (1.3 → 1.4)

# 2.2.6 (October 28th, 2018)

 * Minor tweaks, improving code clarity and such

# 2.2.5 (October 8th, 2018)

 * Fixed broken attempt to include license file in bundled package

# 2.2.4 (October 8th, 2018)

 * Added dedicated changelog, split off from the old single project overview one
 * Included copy of license file in bundled package and excluded the `.gitignore` and `README.md`
   files.

# 2.2.3 (September 20th, 2018)

 * Fixed feature control

# 2.2.2 (September 3rd, 2018)

 * Added homepage and repo links, thanks to @berkus on github for the patch

# 2.2.1 (August 29th, 2018)

 * time/Timeval: (Hopefully) fixed reported compilation errors on macOS and raspberry pi platforms.
   Thanks to @noahbkim on github for reporting and @ssendev for the suggested solution.

# 2.2 (August 21st, 2018)

 * Mainloop/standard: Fixed a typo - renamed `InterateResult` to `IterateResult`. Thanks to
   @HyperHamster on github for reporting.
 * Time/timeval: Fixed a couple of compile time warnings

# 2.1 (July 17th, 2018)

 * Renamed the `timeval` mod to `time`
 * Time: Introduced `UnixTs` and `MonotonicTs` timestamp types, and put them to use with functions
   handling time events.
 * Time: Replaced `rtclock::now` with `MonotonicTs::now`
 * Mainloop: Events now take closures for callbacks, like the rest of the API
 * Operation: Fixed possible memory leak with cancellation
 * Context: Now takes a ref to the mainloop instead of the mainloop’s API in creation methods
 * Mainloop/api: Moved the `mainloop_api_once` method to the `Mainloop` trait from the mainloop API
   structure, and renamed it to `once_event`.
 * Mainloop/signals: Converted to a trait and implemented on mainloops, rather than being
   implemented as methods implemented on the mainloop API structure.
 * Mainloop API objects now correctly treated as immutable. The PA devs have informed me that
   these structures are not intended to be mutated. Patches have been sent to them to correct it in
   the PA C API itself. No point in waiting for those to be accepted. The get/set userdata methods
   have been removed.
 * Mainloop/api: Added method for creating time events using monotonic time values, as an alternate
   to the similar method available on the `Context` object.
 * Events/timer: Added `restart_rt` method, taking monotonic time
 * Context: Removed `rttime_restart` method, made obsolete by the new `restart_rt` method on the
   event itself.
 * Stream: Removed the `get_context` method.
   This method returned a ‘weak’ wrapper object, where ‘weak’ means that it deliberately will not
   decrement the ref count of the underlying C object on drop. This was exactly what was wanted
   back in v1.x, however in v2.0 we introduced closure based callbacks, and the `Context` object
   (wrapper) was extended to hold saved multi-use callbacks. This causes a problem. If you use this
   `get_context` method to get a weak ref, then use it to change a multi-use callback, the new
   callback gets saved into the ‘weak’ object, and then you need both that and the original context
   object wrapper to both exist for the lifetime that you want the new callback to remain in use.
   Not ideal, and not obvious. To fix it would require that `Stream` creation methods take the
   `Context` with an `Rc` wrapper so it can hold onto a cloned `Rc` ref, instead of taking a
   reference, and then returning a cloned `Rc` from `get_context`. The more simple option was to
   just remove this method, as I have done.
 * Events: Removed the `set_destroy_cb` event methods, which became obsolete with the switch to
   closure based callbacks.
 * Events/deferred: Split the `enable` method into separate `enable` and `disable` methods
 * Events/timer: Fixed api pointer type in callback types. Was using C API (sys) type instead of
   binding type, unlike other event callbacks.
 * Timeval: Added convenience `new` and `new_zero` methods
 * Timeval: Added math operation implementations, and removed the `add` and `sub` methods in favour
   of them. Also removed the `set` method since you can convert and assign from `MicroSeconds`
   simply with `From` and `into()`.
 * Time: Enabled conversions between `Duration` and `MicroSeconds`, and between `Duration` and
   `Timeval`.
 * Time: Added math ops to `MicroSeconds` and `Timeval` for adding/subtracting `Duration`
 * Timeval: Removed the `get_time_of_day` method, preferring use of the new timestamp types
 * Stream: Changed the `get_device_name` method to return a `Cow<str>` instead of a `CStr`
 * Avoided creating owned copies of static strings in a few places, returning `Cow<str>` instead.
   This includes `channelmap::Position::to_string`, `channelmap::Map::to_name`,
   `format::Encoding::to_string` and `sample::Format::to_string`.
 * Default-enabled inclusion of the `format::Encoding::from_string` method, for which the underlying
   function symbol was missing from PA's symbol file and thus not available in the client library
   before v12.
 * Format: Fixed `Encoding::from_string`, which would not compile due to a return type mismatch.
   (As mentioned above, the underlying symbol was missing before PA v12, leaving this previously
   untested, and it unfortunately turned out to have been broken).
 * Simplified callback proxy / setup code (internal change only)
 * Context: Purged documentation discussing ref counting
 * Context: Moved documentation discussing mainloop abstraction to the `mainloop` mod
 * Mainloop: Added documentation discussing callback execution
 * Updated `libpulse-sys` version dependency (1.2 → 1.3)

# 2.0.1 (June 26th, 2018)

 * Updated declared PA version compatibility (11.0 → 12.0)
 * Operation: Documented possible memory leak
 * Proplist: Fixed debug output to use comma separator rather than newline (for consistency in
   dumping introspection data), and output in list style instead of mixed struct/list style.
 * Doc typo fixes
 * Updated version in `README` usage example

# 2.0 (June 16th, 2018)

 * Changed handling of callbacks to support closures!
   Now you can simply supply a closure, instead of an `extern C` function and a raw `c_void` data
   pointer, in almost any place across the binding’s API that takes a callback. (Note, there are a
   few places that have not been changed: Tackling the mainloop API stuff has been postponed to look
   at later; same for the stream write method’s optional ‘free’ callback; the `SpawnApi` has no
   userdata arg and thus cannot be done, which is not a big deal; finally the standard mainloop’s
   function for specifying an alternate ‘poll’ function has been left, at least for now).
 * Introspection: Big clean up.
   Previously we simply transmuted from the raw C structs, which was very efficient, and some
   attributes (like enums) transmuted perfectly to binding counterparts thus were ‘clean’. A lot of
   attributes however were ugly, exposing raw pointers, particularly with strings and lists. Now
   instead a proper conversion is done. This takes more effort, but gives a much more pleasant to
   use interface. Note, the `Debug` trait has been implemented, thus combined with the new support
   of closures for callbacks, makes it very easy to dump a copy of introspection data.
 * Context: ext_*: Cleaned up in the same way as introspection
 * Volume: Renamed `CVolume` to `ChannelVolumes`
 * Removed `Option` wrapper around `Operation` objects, using assertion that the inner pointer is
   not null.
 * Context: Handful of methods changed to take immutable `self`, per respective change in version
   1.2 of `libpulse-sys`.
 * Format: Simplified. The inner pointer to a raw `InfoInternal` object (mistakenly hidden in v1.1,
   restored in v1.2.2) has been hidden, replaced with cleaner set/get methods. A method already
   existed for setting the encoding; one has now also been added for reading it, saving users from
   having to do an unsafe raw pointer dereference. Also, with respect to the property list, if the
   existing convenience methods are insufficient and direct access is needed, this is also now far
   cleaner; not only is the unsafe raw pointer dereference avoided, but now you are given access to
   a clean `Proplist` wrapper (see `get_properties` and `get_properties_mut`).
 * Assert added to block use of threaded mainloop `lock` method from within the event loop thread
   (i.e. within most instances of callback execution).
 * Stream: Renamed `proplist_remove` to `remove_proplist`
 * Stream: Common `set_event_callback` callback event names moved to submodule
 * Introspection: Removed unnecessary converters.
   The `From` trait was implemented for introspection objects in both directions between the binding
   and the sys instances. While this is necessary for the sys to binding direction, the other really
   wasn’t needed.
 * Subscribe: Purged autoload API (deprecated since 2009)
 * Updated `libpulse-sys` version dependency (1.1 → 1.2)

# 1.2.2 (June 16th, 2018)

 * Format: Restored access to `Info`’s `ptr` attribute

# 1.2.1 (June 15th, 2018)

**Note: This includes a security fix!**

 * Stream: Fixed use-after-frees with `get_format_info` and `get_context`. These should have used
   `from_raw_weak` instead of `from_raw` to avoid incorrectly freeing the underlying C object,
   leaving a dangling pointer.

# 1.2 (June 1st, 2018)

 * Fixed lifetime issues with a handful of stream methods
 * Fixed lifetime issues with `get_api` mainloop method
 * mainloop/standard: Fixed `run` method’s return data. Incorrectly was returning function call
   result, while claiming in the documentation that this was the quit retval, which wasn’t returned
   at all.
 * Tidied up error code handling:
    - Added `PAErr` wrapper for the `i32` error type, for cleaner interfaces
    - Moved the `strerror` function to be a `PAErr` method
    - Renamed the `sterror` method of `PAErr` and `Code` to `to_string`
    - Converted the error `CStr` to `String` for users; no need to make users do it
    - Added `PAErr` ↔ `Code` `From` impls
 * Simplified volume handling:
    - `Volume` and `VolumeDB` are now wrappers rather than type aliases
    - Added the `VolumeLinear` wrapper. I had mistakenly taken floating point volumes to all be dB
      values, but there is actually a distinction between dB and ‘linear factor’, as per the C API
      conversion functions. This is now used in linear related conversions, which thus no longer
      incorrectly portray such values to be dB scale.
    - Renamed `DECIBEL_MININFTY` to `DECIBEL_MINUS_INFINITY`
    - Renamed `CVolume`’s `inc` and `dec` methods to `increase` and `decrease` respectively for
      clarity (they are not increment/decrement).
 * Stream:
    - The buffer given by `begin_write` is now converted to a slice for you, rather than burdening
      the caller.
    - The return type of `begin_write` has been further simplified, ditching the custom
      `BufferResult` object.
    - Minor doc fixes
 * Derived more common traits for a handful of structs/enums
 * Implemented `PartialEq` for `channelmap::Map` and `volume::CVolume`
 * Removed `Copy` and `Clone` derives from `def::SpawnApi`
 * Improved time handling:
    - Added `Timeval` wrapper
    - Introduced `MicroSeconds` as an `u64` wrapper, replacing use of `sample::Usecs` (now removed)
    - Tidied up conversion constants. Note, names (and in some cases types) have changed
    - Re-exported `libc::timeval` (primarily for timer event callback use) from the `timeval` mod
      instead of `mainloop::events::timer`.
 * Added and put to use wrapper for ‘quit return values’
 * Changed a handful of methods to return `String` rather than `CStr`. The original intention was
   to avoid unnecessary conversion, but users most likely would prefer `Strings`s, and there should
   definitely not be a problem with “lossy” utf8 conversion in these cases.
 * Stream: Now returning unsigned from `get_underflow_index`
 * Hid string printing length constants, only used internally
 * Rewrote string printing functions to use a `Vec` as the string buffer instead of `libc::malloc`,
   thus more simple, and removed the `Option` result wrapper.
 * Changed several `sample::Format` related functions to methods

# 1.1 (May 27th, 2018)

 * Privatised the `from_raw` method of various objects, having become aware of the more granular
   `pub(crate)` type scope limiting syntax. Note, `from_raw_weak` methods designed for use in
   callbacks are still available.
 * Also privatised inner pointer attributes of structs in a few places on the same basis
 * Privatised the `utils::unwrap_optional_callback` function
 * Privatised timer event's `get_ptr` method
 * Various functions have been changed to take immutable references for certain params, reflecting
   the change in version 1.1 of the `libpulse-sys` crate.
 * Promoted `self` reference to mutable for various methods across context, mainloops, streams,
   proplist and operation objects. Although our objects in these cases can be used immutably (no
   change in the wrapped pointer), we should reflect mutability needs of the operation being
   performed on the underlying C objects.
 * Updated `libpulse-sys` version dependency (1.0 → 1.1)

# 1.0.5 (May 27th, 2018)

 * Fixed and simplified `Proplist` iteration:
    - Fixed an infinite loop bug: I misread the documentation, it’s the return value from the C
      function call that will be NULL when it reaches the end of the list, not the state variable.
    - Fixed infinite loop bug #2: The state tracking variable used for the underlying C function
      cannot be hidden within the iterate function, it causes an infinite loop whereby the function
      always just returns the first entry wrapped in `Some`. I don’t know wtf I was thinking.
    - Implemented proper iterator semantics. The `iterate` method was renamed `iter` and now returns
      an actual Rust `Iterator` object, which makes iterating much more simple and tidy.
 * CVolume: Made `self` for `is_[muted|norm]` immutable
 * Stream: Fixed unwanted double option wrapping of callback fn ptr with write methods
 * Stream: Combined `write_ext_free` `free_cb_data` param with `free_cb` as tuple, as done elsewhere

Note, version number 1.0.4 skipped (it was used for non-crate project changes)

# 1.0.3 (February 10th, 2018)

 * Added `From` methods for transmuting between certain introspection structs and their `sys`
   counterparts. (They are identical, and only duplicated in the binding to add documentation).

# 1.0.2 (February 9th, 2018)

 * Added travis badge

# 1.0.1 (February 2nd, 2018)

 * Fixed toml file license string `LGPL-2.1` → `LGPL-2.1+`
 * Fixed toml file missing author email address
 * Removed obsolete readme doc links

# 1.0 (January 24th, 2018)

 * Original release
