Overview
========

This repository contains *sys* and *binding* libraries (crates) for connecting to PulseAudio (PA)
from the Rust programming language.

These are provided for each of the three system libraries:
 * `libpulse_binding` for `libpulse`,
 * `libpulse_simple_binding` for `libpulse-simple`, and
 * `libpulse_glib_binding` for `libpulse-mainloop-glib`.

The *sys* crates provide basic interfaces to the raw C APIs, while the *bindings* add Rust-oriented
higher-level abstractions on top of these. (It is the bindings that you should prefer to make direct
use of in Rust applications).

See the respective library sub-directories for details.

PulseAudio Version Compatibility
================================

Please see the separate `COMPATIBILITY.md` file for discussion of PA version compatibility.

Author
======

These bindings were not produced by the PulseAudio project, they were produced by an independent
developer - Lyndon Brown.

Copyright & Licensing
=====================

All parts of these binding libraries are fully open-source and free to use.

## Primary Licensing - LGPL

All files in this source code repository, except as noted below, are licensed under the GNU Lesser
General Public License (LGPL) version 2.1+. (See file `LICENSE-LGPL` for details).

This matches the current licensing of PulseAudio itself.

Note, compliance with LGPL imposes an important restriction on use that you need to consider. It is
natural in Rust projects to just simply *depend* on various other crates, and what actually happens
by default is that those crate libraries are *statically* compiled into your object (binary or
dynamic shared library). You can optionally of course choose to compile such a lib crate dependency
as a shared library (dynamically compiled). LGPL requires that in the case of static compilation
(the default with Rust dependencies) that the object that LGPL code is statically compiled into is
itself either licensed as GPL or LGPL (and specific versions of those licenses only). If this is not
acceptable or possible, then the only alternative is to compile and use the LGPL lib crate as a
dynamically compiled shared library object, which imposes no such restriction.

## Alternate Licensing

A popular alternative to LGPL, especially I have noticed in the world of Rust crates, is a dual MIT
and Apache-2.0 licensing model. These do not impose the same restrictions with static compilation as
LGPL does; such licensed crates can be used with less restriction on what licensing you can choose
for the binaries or shared libraries that are produced.

I *would* license this project under such a model, however unfortunately I believe that there is a
strong possibility that it would be considered a ‘derivative work’ of PulseAudio, and as such is
constrained to LGPL by the licensing of PulseAudio itself.

That said, you may freely consider this project available under said dual MIT and Apache-2.0
licensing under one of the following conditions:
 - Either, should I simply be wrong about the ‘derivative work’ principle, i.e. that there is no
   actual restriction imposed by the PulseAudio LGPL license on my bindings being licensed under
   this alternate model. (In which case please inform me thusly).
 - Or, you are granted license to use PulseAudio itself under such licensing, removing the barrier
   to use of these bindings under same (I would expect). (Unlikely).
 - Or, should the PulseAudio project grant explicit permission for this project to be licensed under
   such a model.
 - Or, should the PulseAudio project grant *you* explicit permission to use the Rust crates of this
   project under such license.

To be clear, the “dual” aspect of this licensing model is that you get to pick which of these two
available licenses (MIT *or* Apache-2.0) you wish to make use of.

## Licensing/Copyright Specifics

The files within the ‘includes’ directory, have been copied directly from the PulseAudio source.
These files are kept for development purposes only (to be compared through diff checking against
future versions to find changes that may need propagating into the bindings). To be clear, they are
not used in any compilation processes. They are licensed under LGPL by the PulseAudio project.

The *binding* and *sys* libraries provided in this source code repository have been built upon the
public API of PulseAudio, as described by the PulseAudio ‘include’ files, with documentation in
particular largely copied from those files.

The logo image files are a combined derivative of the Rust programming language icon and the
PulseAudio icon, taking core elements from each. I apply no specific image-oriented license upon
them (I am not familiar with such licenses). As a substitute, subject to any constraints of
licensing of those original images, I freely permit use on a common-sense fair-use basis. For
instance, you may freely make use of them in articles discussing this project (should anyone ever
care to do so). Feel free to make your own such derived logos, I make no claim upon it being an
original idea.

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
 - workspace/                   - Just part of the Cargo workspace setup.
