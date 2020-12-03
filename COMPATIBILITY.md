PulseAudio Version Compatibility
================================

This documentation relates to the topic of compatibility with different versions of the PulseAudio
(“PA”) system libraries.

## Introduction

New versions of PulseAudio sometimes carry changes in its API/ABI. These changes may for instance
include introducing new functions, adding new enum variants, extending structs with additional
members, and so on.

Changes to a shared library’s interface (the ‘contract’ between the library and programs that use
it) naturally raises possible compatibility problems for any programs (or other libraries) that use
it.

## Backward vs. Forward Compatibility

“Backward” compatibility of a shared library is the property that a newer version can act as a
“drop-in” replacement for an older version, such that programs built for the older version can work
just fine with the newer version without being changed or re-compiled. That is, the library can be
freely updated to the newer version without causing problems, or that compiled programs can be
copied to a different system that has a newer version of the shared library and work just fine.

“Forward” compatibility of a shared library is the similar concept that an older version can act as
a “drop-in” replacement for a newer version. That is, the library can be “downgraded” to an older
version without creating problems for programs that were compiled against the newer version, or that
a compiled program can be copied to a different system that has an older version of the shared
library and work just fine.

PulseAudio releases are carefully produced to maintain backward compatibility. Forward compatibility
however is a different and more complicated matter.

## Controlling Compatibility

The Rust crates provided by this project each have a set of Cargo feature flags relating to
PulseAudio versions. The supported versions of PA range from version 4.0 onwards.

The first thing to understand is that such feature flags are provided only when a version of PA
includes changes to its API/ABI that require one, not for every PA version ever released. Hence
there are, for example, feature flags for PA versions 8.0 and 12.0, but not for 9.0, 10.0 and 11.0,
which introduced no relevant changes to the PA interface.

The next thing to understand is what enabling a PA version feature flag does. It enables use of the
corresponding changes that that version of PA made to its API. Thus, it enables new functions, enum
variants and such, along with any functionality built on top of this.

However, it is very important, in terms of forward compatibility concerns, to note that enabling a
version feature flag also has the effect of adapting the operation of certain functions that make
use of the underlying PA C API, as we will get to in the next section.

You should understand that you do **not** need to enable the feature flag for a particular PA
version simply for your program to be compatible with that version. Remember that PA versions are
backward compatible. Thus for example, if you enable the PA version 12.0 feature flag but not the
version 13.0 one, this does **not** mean that your program will only work with PA versions up to
version 12.0. It means the opposite, that your program will work fine with PA versions 12.0 and
newer, but that any new functions and such PA version 13.0 and newer introduced are unavailable for
you to use.

Simply put, when a PA version feature flag is enabled, it should be taken to mean that users of your
program must have at least that version in order for your program to be guaranteed to work
correctly. It is thus advisable to not enable PA version features unless you need to make use of
something a version introduced, and you are happy to make the trade off in raising the minimum PA
version required by your program.

Note that enabling the feature for a particular PA version will automatically also enable those for
any older versions. For example, enabling the PA version 13.0 feature automatically enables the
version 12.0 feature, which automatically enables the version 8.0 feature and so on. Also note that
features enabled by dependencies are cumulative, so if you only enable the version 8.0 feature, but
depend also upon something else which itself depends on these crates and which enables the version
12.0 feature, then cumulatively you have version flags 12.0 and lower all enabled.

Note that the version numbers referenced in the feature flag names are that of the PA
product/source-code version numbers, **not** it’s API/ABI/‘soname’ versioning.

## “Forward” Compatibility Caution

As discussed above, enabling feature flags adapts the crates, making new functionality from newer PA
versions available, and also in places adjusts the operation of functions; thus enabling them should
be considered as raising the minimum version of PA required for your program to work properly.
However, simply enabling the feature flags does nothing by itself to prevent users from trying to
run your program on systems with older versions of PA. This is a problem that could potentially
expose them to buggy behaviour or even possibly security vulnerabilities.

(Note that as discussed in the next section, you will get an error compiling if you don’t have a new
enough version; this error does not occur when running the program though).

The first thing to understand here is that if the older version of PA happens to lack one or more PA
functions that your program is using, then a “missing symbol” error will be encountered upon trying
to load your program, thus preventing it from loading. This is a good thing.

However, what if that is not the case? In that case the program can load perfectly fine. Here lies a
hidden danger. Certain functions in the crates operate slightly differently if they are told that
they can expect at least a certain version of PA, as is signalled by your use of PA version feature
flags. This change in operation is hard coded. They do **not** (because it is not practical) ask PA
what version it is at runtime, they just work on the assumption that the program will never be run
with an older PA version. Thus, if this is ever not the case, things can go very wrong.

As an example, and in fact the primary area of concern, PA versions 5.0 and 14.0 both added new
members to the end of some of the introspection structs. If you enable the feature flags for these
versions (or newer), this adapts the crates to PA version 5.0+ or 14.0+ mode respectively. You will
thus see that these new members are available in the corresponding structs offered by the crates,
both in the raw FFI structs and also the ones returned by the introspection functions that translate
those into the more “Rust-ified” ones. The operation of the “translation” functions will have been
adjusted to include translation of those additional sruct members. The danger here is that if your
program is run on a system with an older version of PA, then the structs it’s C API provides will
not match up correctly to the FFI struct description, lacking those extra members. The translation
function will not know this however and will read and process the extra memory after the end of the
actual structs the C API provided, assuming that memory to hold valid data. What then happens is
undefined behaviour. Possibly a crash, possibly not, possibly an opportunity that could be seized
upon as a security vulnerability, especially considering that some of these members involve
pointers.

Ideally a mechanism should be in place to properly prevent programs from loading at all unless the
version of PA is new enough. Unfortunately, as I understand it, the system library versioning scheme
used by PA itself does not prevent this. PA does not even itself offer a suitable version checking
function to assist programs in checking that they are talking to a new enough version. Trying to
hack a means into these crate functions to double-check the PA version at runtime to catch such
problems does not seem like a good solution. This is an area of significant concern to the author of
these crates, and needs further work/investigation.

Currently it is left to program authors to somehow not let their programs be used on older versions
of PA than they were compiled for, and/or to make careful choices as to the use of version features.
(PA version 5.0 is pretty old, so it is unlikely that programs built for 5.0+ will get run on 4.0;
and PA version 14.0 is very new, so unlikely that programs will select the version 14.0 feature flag
for quite a while. So hopefully this should not be a practical problem).

Some helper functions are provided in the `version` mod of the main binding crate to assist in
checking at runtime the version of the available PA system library by processing the version string
it makes available. You can use these to detect and prevent your program from loading with old PA
versions to thus avoid the potential issues described here.

## A Note On Linking

Note that since the nature of the crates are such that they have code that talks to most of the PA
C API, under certain circumstances, such as building a binary, you must have PulseAudio itself
installed on your system to be able to compile. Furthermore, you must have a version equal to or
newer than the newest selected via the previously mentioned feature flags. If you do not have it
installed, or have too old a version, this would result in linking errors. To that end, the crates
have build scripts which use `pkg-config` to check that you have it installed, and a sufficiently
new version, giving a cleaner error about what the problem is if not.

Note that for this check to work, you need the PA `pkg-config` information file to be installed,
which is typically installed alongside the PA C header files from the PA “developer” package (the
`libpulse-dev` package on Debian Linux).

## Examples:

The following examples demonstrate controlling compatibility via the dependency information in your
project’s `Cargo.toml` file.

Selecting compatibility for PA version 12.0+ (raising it from the current default of 8.0+):

```toml
libpulse-binding = { version = "2.0", features = "pa_v12" }
```

Reducing the level to PA version 6.0+ (requires disabling the current default of 8.0+):

```toml
libpulse-binding = { version = "2.0", default-features = false, features = "pa_v6" }
```

Reducing the level to the absolute minimum supported (currently PA v4.0+):

```toml
libpulse-binding = { version = "2.0", default-features = false }
```
