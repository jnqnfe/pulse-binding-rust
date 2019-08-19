# <unreleased>

 * Extended support to older versions of PA, specifically v4
 * Improved build script
 * Simplified feature flags, old ones left as temorary aliases, to be removed later
 * Added a `dox` feature flag, for use with `cargo doc`.
   It enables the very latest PA version compatibility, while bypassing the pkg-config check, thus
   is useful for generating documentation that includes information on features from versions of PA
   that may be newer than the version you may have installed.

# 1.8.0 (August 15th, 2019)

 * Specified edition in toml file
 * Updated `libpulse-sys` version dependency (1.6 → 1.8)

Note: version 1.7 skipped, used only for `libpulse-mainloop-glib-sys` crate changes

# 1.6.0 (August 12th, 2019)

 * Replaced use of empty enums for opaque types with a struct based alternative. According to the
   Rust nomicon ([here](https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs))
   the use of the empty enum trick is apparently undefined behaviour.
 * Added a new `latest_pa_common_compatibility` feature flag, used by default now instead of
   `latest_pa_compatibility`.
 * Updated `libpulse-sys` version dependency (1.5 → 1.6)

# 1.5.0 (December 22nd, 2018)

 * Added the new `latest_pa_compatibility` and `pa_v12_compatibility` feature flags, and deprecated
   `pa_encoding_from_string` in favour of `pa_v12_compatibility`.
 * Updated `libpulse-sys` version dependency (1.4 → 1.5)

# 1.4.0 (November 4th, 2018)

 * Switched to using package-config for linking (though retaining the direct linking as a fallback
   for those without the necessary *.pc files installed). Thanks to @JohnAZoidberg on github for
   the original patch.
 * Clarified `pa_encoding_from_string` feature purpose
 * Updated `libpulse-sys` version dependency (1.3 → 1.4)

# 1.3.4 (October 8th, 2018)

 * Fixed broken attempt to include license file in bundled package

# 1.3.3 (October 8th, 2018)

 * Added dedicated changelog, split off from the old single project overview one
 * Included copy of license file in bundled package and excluded the `.gitignore` file

# 1.3.2 (September 20th, 2018)

 * Fixed feature control

# 1.3.1 (September 3rd, 2018)

 * Added homepage and repo links, thanks to @berkus on github for the patch
 * Expanded the `pa_encoding_from_string` feature to properly control it across dependencies. Thanks
   to @thejpster on github for reporting.

# 1.3 (July 17th, 2018)

 * Updated `libpulse-sys` version dependency (1.2 → 1.3)

# 1.2 (June 16th, 2018)

 * Updated `libpulse-sys` version dependency (1.1 → 1.2)

# 1.1 (May 27th, 2018)

 * Updated `libpulse-sys` version dependency (1.0 → 1.1)

# 1.0.2 (February 9th, 2018)

 * Added travis badge

# 1.0.1 (February 2nd, 2018)

 * Fixed toml file license string `LGPL-2.1` → `LGPL-2.1+`
 * Fixed toml file missing author email address

# 1.0 (January 24th, 2018)

 * Original release
