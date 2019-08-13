PulseAudio Version Compatibility
=============================

This project aims to always provide compatibility with the latest stable version of PulseAudio (PA).
The minimum supported version is v8.0.

## Compiling & Linking Concerns

Of concern for some users of these libraries will be the ability to make use of them with older
versions of PA than the latest stable one, since it is not always possible to install the very
latest stable version promptly upon release, and since a mismatch of versions can cause problems.

Understand that the nature of these libraries is such that they make use of (almost) the entire PA C
API (i.e. all available PA client library symbols), and when compiling, missing symbols (due to
compiling against too old a version) would cause failure at link time. To be clear, any and all
symbols used by these libraries must exist within the version of the PA system library installed on
the system you are compiling on; if these libraries include support for a function added in a newer
version of PA, then a potential problem arises. (You must, of course, have PA actually installed).

Worry not, Cargo feature flags have you covered!

The `sys` and `binding` crates provided by this project each include a set of PA version related
compatibility feature flags, provided to allow you to avoid use of symbols from versions of PA newer
than your version of PA. Note that a feature flag is **not** provided for each and every major new
PA version, they are only introduced when a new major PA version make API changes that require one.

By default support for new symbols of a new major PA version are left disabled for a while, until
use of that new version is more widespread. Otherwise, they are enabled by default. I.e. the current
stable PA release at the time of writing is v12; support for this is default enabled since it is in
widespread use; a release of PA v13 is being prepared and introduces new symbols, support for which
will be disabled by default until PA v13 is in widespread use. This is done in order to include the
maximum number of features by default, whilst reducing likelihood of users encountering linking
issues.

The set of flags provided each **enable** use of symbols added in a particular version.

Note that only one single feature flag directly needs to be used in your dependencies on these
libraries; the flags automatically enable other flags as appropriate. For instance, if you enable PA
v13.0 support, this will automatically enable PA v12.x support, on top of the base PA v8.0-11.x
support. (No API changes occurred from v8.0 until v12.0).

Alongside PA version specific feature flags, two useful aliases are also provided:
 - The `latest_pa_compatibility` feature enables everything, targetting the very latest supported
   version.
 - The `latest_pa_common_compatibility` feature enables everything except only new symbols from a
   very new release, as just discussed.

By default, `latest_pa_common_compatibility` is enabled.

### Examples:

Specifically selecting PA v12 compatibility:

```toml
libpulse-binding = { version = "2.0", default-features = false, features = "pa_v12_compatibility" }
```

Specifcally selecting minimal (PA v8-11) compatibility (the oldest supported):

```toml
libpulse-binding = { version = "2.0", default-features = false }
```
