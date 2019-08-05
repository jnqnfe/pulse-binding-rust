Overview
========

This repository contains binding libraries for connecting to PulseAudio from the Rust programming
language.

Three bindings are provided:
 * `libpulse_binding` for `libpulse`,
 * `libpulse_simple_binding` for `libpulse-simple`, and
 * `libpulse_glib_binding` for `libpulse-mainloop-glib`.

See the respective library sub-directories for details.

The bindings are based upon the public API of PulseAudio, as provided in the PulseAudio ‘include’ C
header files. They provide:
 * A basic export of the C API.
 * For much of the API, simpler and safer interfaces to the underlying C functions and data
   structures, for instance providing wrappers for PulseAudio objects with drop trait
   implementations that automatically free the object upon going out of scope, ala smart pointers.

PulseAudio Version Compatibility
=============================

This project always intends to provide compatibility with the latest stable version of PulseAudio.
It also however provides backwards compatibility with a limited number of past major releases.

The `sys` and `binding` crates provided by this project include a set of PA version related
compatibility feature flags, used for controlling the client library API version to link to at
compile time. Understand that the crates naturally use all (or almost all) of the API available in
the PA client libraries, and the *linking* stage of compiling them requires that all of those
symbols exist in the version of the libraries installed on the compiled-on system. This introduces
a problem, both for supporting systems rarely updated and also for supporting any new symbols of new
PA versions while the new version is not yet in wide spread use. To tackle this problem, when new
versions of PA introduce new symbols, we introduce a new compatibility feature flag relating to that
new version, and hide all use of the new symbols behind it. Thus, you can target a level of version
compatibility, and freely update to new versions of the crates.

Note that the `latest_pa_compatibility` feature (enabled by default) selects enabling the newest
compatibility available, but this is risky. You can disable this and instead select a specific
version compatibility as demonstrated below. Alternatively `latest_pa_common_compatibility` selects
the latest version deemed to be in widespread use (just excludes compatibility with recently a
released major version temporarily).

Example: Selecting PA v12 compatibility

```toml
libpulse-binding = { version = "2.0", default-features = false, features = "pa_v12_compatibility" }
```

Example: Selecting PA v10/11 compatibility (the oldest supported)

```toml
libpulse-binding = { version = "2.0", default-features = false }
```

Note that new version-targeting features are intended to only be introduced for new PA versions
which introduce new symbols, thus if PA v13 does not do so, the `pa_v12_compatibility` feature will
give both v12 **and** v13 compatibility (consider it to be v12+ until such time that a new one is
needed).

Author
======

These bindings were not produced by the PulseAudio project, they were produced by an independent
developer - Lyndon Brown.

Copyright & Licensing
=====================

## Primary/Current Licensing - LGPL

All files in this source code repository, except as noted below, are licensed under the GNU Lesser
General Public License (LGPL). (See file LICENSE-LGPL for details).

## Alternate Licensing - Read carefully!

Whilst I am actually entirely *open* to MIT/Apache-2.0 licensing, LGPL was chosen simply because
PulseAudio itself is licensed under LGPL, and I expect that this would be considered a derivative
work, which blocks licensing this work under those licenses. Should PulseAudio itself ever relicense
to MIT and/or Apache-2.0, it is intended that this source code repository will be relicensed to
match. Should you have an exception from PulseAudio, licensing it to you under MIT and/or Apache-2.0
licensing, then you may freely consider that to apply here also.

## Licensing/Copyright Specifics

The files within the ‘includes’ directory, have been copied directly from the PulseAudio source.
These files are kept for development purposes only (to be compared through diff checking against
future versions to find changes that may need propagating into new versions of this binding library).
To be clear, they are not used in any compilation processes. They are licensed under LGPL by the
PulseAudio project.

The binding libraries provided in this source code repository have been built upon the public API of
PulseAudio, as described in the PulseAudio ‘include’ files, with documentation in particular largely
copied from those files. These bindings may be considered derivative works under the PulseAudio
license. PulseAudio itself is licensed under LGPL version 2.1+. These bindings, as specified above,
are under that same license.

The logo image files are a combined derivative of the Rust programming language icon and the
PulseAudio icon, taking core elements from each.

Source Code Contents
====================

 - includes/                    - A copy of the original C header files the bindings have been built
                                  to interface with.
 - pulse-binding/               - The main high-level binding library.
 - pulse-binding-mainloop-glib/ - The high-level binding library for the GLIB mainloop.
 - pulse-binding-simple/        - The high-level binding library for the PulseAudio ‘simple’
                                  component.
 - pulse-sys/                   - The main raw C API interface library.
 - pulse-sys-mainloop-glib/     - The raw C API interface library for the GLIB mainloop.
 - pulse-sys-simple/            - The raw C API interface library for the PulseAudio ‘simple’
                                  component.
 - src/                         - Just part of the Cargo workspace setup.
