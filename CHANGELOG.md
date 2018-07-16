# ??? (???)

project:

 * Renamed `NEWS` file to `CHANGELOG.md` and reformatted to markdown

pulse-binding:

 * Renamed the `timeval` mod to `time`
 * Moved the `rtclock::now` function to `time::rtclock_now`
 * Mainloop: Events now take closures for callbacks, like the rest of the API
 * Operation: Fixed possible memory leak with cancellation
 * Context: Now takes a ref to the mainloop instead of the mainloop's API in creation methods
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
   This method returned a 'weak' wrapper object, where 'weak' means that it deliberately will not
   decrement the ref count of the underlying C object on drop. This was exactly what was wanted
   back in v1.x, however in v2.x we introduced closure based callbacks, and the `Context` object
   (wrapper) was extended to hold saved multi-use callbacks. This causes a problem. If you use this
   `get_context` method to get a weak ref, then use it to change a multi-use callback, the new
   callback gets saved into the 'weak' object, and then you need both that and the original context
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
 * Timeval: Added convenience `new`, `new_zero` and `new_tod` methods
 * Timeval: Added math operation implementations
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
 * Added documentation discussing Unix and monotonic time to the `timeval` (now `time`) mod
 * Context: Purged documentation discussing ref counting
 * Context: Moved documentation discussing mainloop abstraction to the `mainloop` mod
 * Mainloop: Added documentation discussing callback execution

pulse-simple-binding:

 * Updated with respect to having renamed the `timeval` mod to `time` in the main binding

pulse-glib-mainloop-binding:

 * Mainloop API objects now correctly treated as immutable, per above

pulse-sys:

 * Mainloop API objects now correctly treated as immutable, per above
 * Default-enabled inclusion of the `pa_encoding_from_string` function symbol, which was missing
   from PA's symbol file and thus not available in the client library before v12.

pulse-glib-mainloop-sys:

 * Mainloop API objects now correctly treated as immutable, per above

# 2.0.1 (June 26th, 2018)

pulse-binding:

 * Updated declared version compatibility
 * Operation: Documented possible memory leak
 * Proplist: Fixed debug output to use comma separator rather than newline (for consistency in
   dumping introspection data), and output in list style instead of mixed struct/list style.
 * Doc typo fixes

pulse-sys (1.2.1):

 * Updated declared version compatibility

pulse-glib-mainloop-binding:

 * Doc typo fix

# 2.0 (June 16th, 2018)

pulse-binding:

 * Changed handling of callbacks to support closures! Now you can simply supply a closure, instead
   of an "extern C" function and a raw `c_void` data pointer, in almost any place across the
   binding's API that takes a callback. (Note, there are a few places that have not been changed:
   Tackling the mainloop API stuff has been postponed to look at later; same for the stream write
   method's optional 'free' callback; the `SpawnApi` has no userdata arg and thus cannot be done,
   which is not a big deal; finally the standard mainloop's function for specifying an alternate
   'poll' function has been left, at least for now).
 * Introspection: Big clean up. Previously we simply transmuted from the raw C structs, which was
   very efficient, and some attributes (like enums) transmuted perfectly to binding counterparts thus
   were 'clean'. A lot of attributes however were ugly, exposing raw pointers, particularly with
   strings and lists. Now instead a proper conversion is done. This takes more effort, but gives a
   much more pleasant to use interface. Note, the `Debug` trait has been implemented, thus combined
   with the new support of closures for callbacks, makes it very easy to dump a copy of
   introspection data.
 * Context: ext_*: Cleaned up in the same way as introspection.
 * Volume: Renamed `CVolume` to `ChannelVolumes`
 * Removed `Option` wrapper around `Operation` objects, using assertion that the inner pointer is
   not null.
 * Context: Handful of methods changed to take immutable `self`. Patches were previously sent in to
   apply `const` to a lot of pointers in the C API, which have been accepted. Some context related
   functions were not changed simply because of an artefact relating to validation checks.
   Additional patches solving this have now been sent in. This change has been reflected here. (No
   need to worry about whether or not these are accepted, though discussion indicates they will, nor
   for a new version of PA to be released; logically they should be immutable, and we do not need to
   propagate this unfortunate artefact).
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
   wasn't needed.
 * Subscribe: Purged autoload API (deprecated since 2009)

pulse-sys (1.2.0):

 * Context: Handful of functions changed to take `const` pointers, as discussed above.
 * Introspect & subscribe: Purged autoload API (deprecated since 2009)

# 1.2.2 (June 16th, 2018)

pulse-binding:

 * Format: Restored access to `Info`'s `ptr` attribute

# 1.2.1 (June 15th, 2018)

pulse-binding:

 * Stream: Fixed use-after-frees with `get_format_info` and `get_context`. These should have used
   `from_raw_weak` instead of `from_raw` to avoid incorrectly freeing the underlying C object,
   leaving a dangling pointer.

# 1.2 (June 1st, 2018)

project:

 * Further licensing clarification

pulse-binding:

 * Fixed lifetime issues with a handful of stream methods.
 * Fixed lifetime issues with `get_api` mainloop method.
 * mainloop/standard: Fixed `run` method's return data. Incorrectly was returning function call
   result, while claiming in the documentation that this was the quit retval, which wasn't returned
   at all.
 * Tidied up error code handling:
    - Added `PAErr` wrapper for the `i32` error type, for cleaner interfaces
    - Moved the `strerror` function to be a `PAErr` method
    - Renamed the `sterror` method of `PAErr` and `Code` to `to_string`
    - Converted the error `CStr` to `String` for users; no need to make users do it
    - Added `PAErr` <=> `Code` `From` impls
 * Simplified volume handling:
    - `Volume` and `VolumeDB` are now wrappers rather than type aliases
    - Added the `VolumeLinear` wrapper. I had mistakenly taken floating point volumes to all be dB
      values, but there is actually a distinction between dB and 'linear factor', as per the C API
      conversion functions. This is now used in linear related conversions, which thus no longer
      incorrectly portray such values to be dB scale.
    - Renamed `DECIBEL_MININFTY` to `DECIBEL_MINUS_INFINITY`
    - Renamed `CVolume`'s `inc` and `dec` methods to `increase` and `decrease` respectively for
      clarity (they are not increment/decrement).
 * Stream:
    - The buffer given by `begin_write` is now converted to a slice for you, rather than burdening
      the caller.
    - The return type of `begin_write` has been further simplified, ditching the custom
      `BufferResult` object.
    - Minor doc fixes
 * Derived more common traits for a handful of structs/enums
 * Implemented `PartialEq` for `::channelmap::Map` and `::volume::CVolume`
 * Removed `Copy` and `Clone` derives from `def::SpawnApi`
 * Improved time handling:
    - Added `Timeval` wrapper
    - Introduced `MicroSeconds` `u64` wrapper, replacing use of `::sample::Usecs` (now removed)
    - Tidied up conversion constants. Note, names (and in some cases types) have changed.
    - Re-exported `libc::timeval` (primarily for timer event callback use) from `::timeval` instead
      of `::mainloop::events::timer`.
 * Added and put to use wrapper for 'quit return values'
 * Changed a handful of methods to return `String` rather than `CStr`. The original intention was
   to avoid unnecessary conversion, but users most likely would prefer `Strings`s, and there should
   definitely not be a problem with "lossy" utf8 conversion in these cases.
 * Stream: Now returning unsigned from `get_underflow_index`
 * Hid string printing length constants, only used internally
 * Rewrote string printing functions to use a `Vec` as the string buffer instead of `libc::malloc`,
   thus more simple, and removed the `Option` result wrapper.
 * Changed several `::sample::Format` related functions to methods

pulse-simple-binding:

 * Tidied up error code handling, per above
 * Improved time handling, per above

pulse-glib-binding:

 * Now returning `get_api` pointer as ref, as done with standard and threaded mainloops

# 1.1 (May 27th, 2018)

pulse-binding:

 * Privatised the `from_raw` method of various objects, having become aware of the more granular
   `pub(crate)` type scope limiting syntax. Note, `from_raw_weak` methods designed for use in
   callbacks are still available.
 * Also privatised inner pointer attributes of structs in a few places on the same basis.
 * Privatised the `utils::unwrap_optional_callback` function
 * Privatised timer event's `get_ptr` method
 * As described below, a number of C function declarations in the sys crates now take certain object
   pointers immutably. A small number of functions in the binding have been similarly updated.
 * Promoted `self` reference to mutable for various methods across context, mainloops, streams,
   proplist and operation objects. Although our objects in these cases can be used immutably (no
   change in the wrapped pointer), we should reflect mutability needs of the operation being
   performed on the underlying C objects.

pulse-simple-binding:

 * Privatised `SimpleInternal`

pulse-sys:

 * Various functions have been changed to take immutable `const` pointers.
   There are numerous functions in the C API which take mutable pointers to objects where there is
   no intention to actually mutate those objects. Patches have been sent in to the PA project to
   correct many of these cases. There is no point in waiting for those to be accepted, so the
   change has been reflected in the C API description in the sys crate.

# 1.0.5 (May 27th, 2018)

pulse-binding:

 * Fixed and simplified Proplist iteration:
    - Fixed an infinite loop bug: I misread the documentation, it's the return value from the C
      function call that will be NULL when it reaches the end of the list, not the state variable.
    - Fixed infinite loop bug #2: The state tracking variable used for the underlying C function
      cannot be hidden within the iterate function, it causes an infinite loop whereby the function
      always just returns the first entry wrapped in `Some`. I don't know wtf I was thinking.
    - Implemented proper iterator semantics. The `iterate` method was renamed `iter` and now returns
      an actual Rust Iterator object, which makes iterating much more simple and tidy.
 * CVolume: Made `self` for `is_[muted|norm]` immutable
 * Stream: Fixed unwanted double option wrapping of callback fn ptr with write methods
 * Stream: Combined `write_ext_free` `free_cb_data` param with `free_cb` as tuple, as done elsewhere

pulse-simple-binding:

 * Enabled pulse-simple doc test

pulse-sys:

 * Minor, non-functional consistency fix only

# 1.0.4d (February 15th, 2018)

project:

 * Now using explicit test script for Travis, rather than relying on default, to ensure that the
   `--all` flag is passed to `cargo test`, which I thought was already done, but seems not from the
   logs and thus tests were not actually being done.
 * Fixed Travis failures - added missing `libpulse-mainloop-glib0` dependency to test environment.
 * Properly fixed Travis tests - added pulseaudio dependency and get it started in test environment.

# 1.0.3 (February 10th, 2018)

pulse-binding:

 * Added `From` methods for transmuting between certain introspection structs and their `sys`
   counterparts. (They are identical, and only duplicated in the binding to add documentation).

# 1.0.2 (February 9th, 2018)

 * Added travis badge to individual crates

# 1.0.1 (February 2nd, 2018)

 * Fixed toml file license string `LGPL-2.1` => `LGPL-2.1+`
 * Fixed toml file missing author email address
 * Removed obsolete readme doc links

# 1.0 (January 24th, 2018)

 * Original release
