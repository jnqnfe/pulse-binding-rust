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

You can help fund my work through one of the following platforms: [patreon][patreon],
[liberapay][liberapay], or [buy-me-a-coffee][buymeacoffee].

[patreon]: https://www.patreon.com/jnqnfe
[liberapay]: https://liberapay.com/jnqnfe/
[buymeacoffee]: https://www.buymeacoffee.com/jnqnfe

Copyright & Licensing
=====================

All parts of these binding libraries are fully open-source and free to use.

All files in this source code repository, except as noted below, are licensed under the MIT license
or the Apache license, Version 2.0, at your option. You can find copies of these licenses either in
the `LICENSE-MIT` and `LICENSE-APACHE` files, or alternatively [here][1] and [here][2] respectively.

[1]: http://opensource.org/licenses/MIT
[2]: http://www.apache.org/licenses/LICENSE-2.0

## Documentation Specifics

The documentation of the *binding* and *sys* libraries provided in this source code repository has
largely been copied (with some modifications in places) from that provided in the LGPL 2.1+ licensed
C header files of the PulseAudio client library itself. This has been done on a fair-use basis.
(Fair-use is permitted by the LGPL license as discussed in [the GPL/LGPL FAQ][gpl_faq_fairuse]).
This should be of no concern for normal use of these crates, you can freely compile them statically
into your projects under the dual MIT and Apache-2.0 licensing (documentation naturally does not get
compiled into your library/application), and you should be able to freely use a personal copy of the
crate documentation generated into HTML form with `cargo doc` (fair-use).

[gpl_faq_fairuse]: https://www.gnu.org/licenses/gpl-faq.en.html#GPLFairUse

## Other Specifics

The files within the ‘includes’ directory, have been copied directly from the PulseAudio source.
These files are kept for development purposes only (to be compared through diff checking against
future versions to find changes that may need propagating into the bindings). To be clear, they are
not used in any compilation processes. They are licensed under LGPL by the PulseAudio project.

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
