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

Example: Selecting PA v8+ compatibility (the oldest supported)

```toml
libpulse-binding = { version = "2.0", default-features = false }
```
