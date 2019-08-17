PulseAudio Version Compatibility
=============================

This project aims to always provide compatibility with the latest stable version of PulseAudio (PA).
The minimum supported version is v5.0.

## Compiling & Linking Concerns

Of concern for some users of these libraries will be the ability to make use of them with older
versions of PA than the latest stable one, since it is not always possible to install the very
latest stable version promptly upon release, and since making use of features from versions newer
than the version you have installed will result in linker errors.

Worry not, Cargo feature flags have you covered!

The `sys` and `binding` crates provided by this project each include a set of PA version related
compatibility feature flags, provided to allow you to avoid use of features from versions of PA newer
than your version of PA. Note that a feature flag is **not** provided for each and every major new
PA version, they are only introduced when a new major PA version make API changes that require one.

By default support for new features of a new major PA version are left disabled for a while, until
use of that new version is more widespread. Otherwise, they are enabled by default. I.e. the current
stable PA release at the time of writing is v12; support for this is default enabled since it is in
widespread use; a release of PA v13 is being prepared and introduces new features, support for which
will be disabled by default until PA v13 is in widespread use.

The set of flags provided each **enable** use of features added in a particular version.

Note that only one single feature flag directly needs to be used in your dependencies on these
libraries; the flags automatically enable other flags as appropriate. For instance, if you enable PA
v13.0 support, this will automatically enable PA v12.x support, on top of the base PA v8.0-11.x
support. (No API changes occurred from v8.0 until v12.0).

Alongside PA version specific feature flags, two useful aliases are also provided:
 - The `latest_pa_compatibility` feature enables everything, targetting the very latest supported
   version.
 - The `latest_pa_common_compatibility` feature enables everything except only new features from a
   very new release, as just discussed.

By default, `latest_pa_common_compatibility` is enabled.

### Examples:

Specifically enabling PA v12+ compatibility:

```toml
libpulse-binding = { version = "2.0", default-features = false, features = "pa_v12_compatibility" }
```

Specifically lowering to minimal compatibility (PA v5+):

```toml
libpulse-binding = { version = "2.0", default-features = false }
```
