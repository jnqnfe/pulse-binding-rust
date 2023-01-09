# 2.27.1 (January 9th, 2023)

 * Fixed broken build status badge in readme.

# 2.27.0 (January 9th, 2023)

 * Bumped Rust edition to 2021.
 * Added `rust-version` (MSRV) attribute to `cargo.toml` file.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.24 to 2.27.
    - `libpulse-mainloop-glib-sys` from 1.19 to 1.20.

Note: version 2.26 skipped to catch up with main `libpulse-binding` version.

# 2.25.1 (January 13th, 2022)

 * Minor tweaks not worth documenting.

# 2.25.0 (August 29th, 2021)

 * Minor tweaks to work with the changes in verson 2.25.0 of the main binding crate.
 * Minor formatting tweaks.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.24 to 2.25.

# 2.24.0 (July 28th, 2021)

 * Updated required dependencies:
    - `libpulse-binding`           from 2.23 to 2.24.
    - `libpulse-mainloop-glib-sys` from 1.16 to 1.19.

# 2.23.0 (January 30th, 2021)

 * Now using intra-doc-linking introduced in Rust 1.48. The MSRV remains unchanged at 1.41; this is
   too great a simplification to maintaining documentation to not implement immediately; the only
   negative is that links will be broken in locally generated documentation when built with an older
   Rust compiler than version 1.48.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.20 to 2.23.

Note: versions 2.21 and 2.22 skipped, used only for `libpulse-binding` crate changes.

# 2.20.2 (December 26th, 2020)

 * Fixed segfault thanks to @DanielChabrowski on github.

# 2.20.1 (December 15th, 2020)

 * Fixed mistake made trying to conditionally enable `#[cfg(doc)]` for docs.rs.

# 2.20.0 (December 14th, 2020)

 * Made some changes to cargo features:
    - Removed the now obsolete `dox` cargo feature.
    - Removed the `pa_latest` and `pa_latest_common` cargo features.
    - Changed the default version feature level to `pa_v8`.
 * MSRV bumped from 1.40 to 1.41.
 * Made use of `#[cfg(doc)]` to always include stuff behind PA version feature guards in generated
   documentation. (Required bump of minimum supported Rust version from 1.40 to 1.41).
 * Added support for feature tagging in documentation (requires nightly Rust version, so only
   enabled if a certain config flag is used, as for the docs.rs copy).
 * Updated required dependencies:
    - `libpulse-binding`           from 2.18 to 2.20.
    - `libpulse-mainloop-glib-sys` from 1.15 to 1.16.

Note: version 2.19 skipped, used only for `libpulse-binding` crate changes.

# 2.18.1 (November 25th, 2020)

 * Fixed deprecated license attribute syntax.

# 2.18.0 (November 25th, 2020)

 * Updated required dependencies:
    - `libpulse-binding`           from 2.17 to 2.18.
    - `libpulse-mainloop-glib-sys` from 1.14 to 1.15.

# 2.17.0 (November 24th, 2020)

 * Updated required dependencies:
    - `libpulse-binding`           from 2.16 to 2.17.
    - `libpulse-mainloop-glib-sys` from 1.13 to 1.14.

# 2.16.1 (September 9th, 2020)

 * Bumped `pa_latest_common` feature to target PA v13.

# 2.16.0 (April 18th, 2020)

 * Removed deprecated Cargo features.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.15 to 2.16.
    - `libpulse-mainloop-glib-sys` from 1.12 to 1.13.

# 2.15.0 (December 29th, 2019)

 * Updated required dependencies:
    - `libpulse-binding`           from 2.14 to 2.15.
    - `libpulse-mainloop-glib-sys` from 1.11 to 1.12.

# 2.14.0 (October 28th, 2019)

 * Updated required dependencies:
    - `libpulse-binding` from 2.13 to 2.14.

# 2.13.0 (September 17th, 2019)

 * Changed the license model from LGPL to dual MIT and Apache-2.0. See [here][issue26] for details.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.12 to 2.13.
    - `libpulse-mainloop-glib-sys` from 1.10 to 1.11.

# 2.12.0 (September 15th, 2019)

 * Added PA v13 compatibility control feature.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.11 to 2.12.
    - `libpulse-mainloop-glib-sys` from 1.9 to 1.10.

# 2.11.1 (August 19th, 2019)

 * Fixed broken doc.rs documentation generation.

# 2.11.0 (August 19th, 2019)

 * Extended support to even older versions of PA, specifically up to and including v4.
 * Simplified feature flags, old ones left as temorary aliases, to be removed later.
 * Added a `dox` feature flag, for use with `cargo doc`.
   It enables the very latest PA version compatibility, while bypassing the pkg-config check, thus
   is useful for generating documentation that includes information on features from versions of PA
   that may be newer than the version you have installed.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.10 to 2.11.
    - `libpulse-mainloop-glib-sys` from 1.8 to 1.9.

# 2.10.0 (August 15th, 2019)

 * Updated `use` conventions to that of Rust 1.30/1.31.
 * Specified edition in toml file.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.7 to 2.10.
    - `libpulse-mainloop-glib-sys` from 1.6 to 1.8.

# 2.9.0 (August 14th, 2019)

 * Changed `Mainloop::new()` to take a ref of `glib::MainContext` not `glib_sys::GMainContext`.

# 2.8.0 (August 13th, 2019)

 * Addressed long standing todo item of linking `GMainContext` to an actual glib crate.
    - Added dependency on `glib-sys`.
    - Replaced our own `GMainContext` type (re-exported from our `sys` crate) with the one from the
      `glib-sys` crate.
    - Deprecated use of `GMainContext` directly from this crate.

# 2.7.1 (August 13th, 2019)

 * Fix missed use of UB empty enum trick.
 * Fixed missing simplification of pointer handling.

# 2.7.0 (August 12th, 2019)

 * Added a new `latest_pa_common_compatibility` feature flag, used by default now instead of
   `latest_pa_compatibility`.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.6 to 2.7.
    - `libpulse-mainloop-glib-sys` from 1.5 to 1.6.

# 2.6.0 (March 10th, 2019)

 * Updated required dependencies:
    - `libpulse-binding` from 2.5 to 2.6.

# 2.5.0 (December 22nd, 2018)

 * Added the new `latest_pa_compatibility` and `pa_v12_compatibility` feature flags, and deprecated
   `pa_encoding_from_string` in favour of `pa_v12_compatibility`.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.4 to 2.5.
    - `libpulse-mainloop-glib-sys` from 1.4 to 1.5.

# 2.4.0 (November 28th, 2018)

 * Updated required dependencies:
    - `libpulse-binding` from 2.3 to 2.4.

# 2.3.0 (November 4th, 2018)

 * Clarified `pa_encoding_from_string` feature purpose.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.2 to 2.3.
    - `libpulse-mainloop-glib-sys` from 1.3 to 1.4.

# 2.2.5 (October 8th, 2018)

 * Fixed broken attempt to include license file in bundled package.

# 2.2.4 (October 8th, 2018)

 * Added dedicated changelog, split off from the old single project overview one.
 * Included copy of license file in bundled package and excluded the `.gitignore` and `README.md`
   files.

# 2.2.3 (September 20th, 2018)

 * Fixed feature control.

# 2.2.2 (September 3rd, 2018)

 * Added homepage and repo links, thanks to @berkus on github for the patch.
 * Expanded the `pa_encoding_from_string` feature to properly control it across dependencies. Thanks
   to @thejpster on github for reporting.

Note, version number 2.2.1 skipped, bumping number into line with the main binding crate.

# 2.2 (August 21st, 2018)

 * Updated required dependencies:
    - `libpulse-binding` from 2.1 to 2.2.

# 2.1 (July 17th, 2018)

 * Mainloop API objects now correctly treated as immutable, per related change in version 2.1 of
   `libpulse-binding`.
 * Implemented new signals trait from version 2.1 of the main binding.
 * Updated required dependencies:
    - `libpulse-binding`           from 2.0 to 2.1.
    - `libpulse-mainloop-glib-sys` from 1.2 to 1.3.

# 2.0.1 (June 26th, 2018)

 * Doc typo fix.
 * Updated version in `README` usage example.

# 2.0 (June 16th, 2018)

 * Updated required dependencies:
    - `libpulse-binding`           from 1.2 to 2.0.
    - `libpulse-mainloop-glib-sys` from 1.1 to 1.2.

# 1.2 (June 1st, 2018)

 * Now returning `get_api()` pointer as ref, as done with standard and threaded mainloops.
 * Updated required dependencies:
    - `libpulse-binding` from 1.1 to 1.2.

# 1.1 (May 27th, 2018)


 * Updated required dependencies:
    - `libpulse-binding`           from 1.0 to 1.1.
    - `libpulse-mainloop-glib-sys` from 1.0 to 1.1.

# 1.0.2 (February 9th, 2018)

 * Added travis badge.

# 1.0.1 (February 2nd, 2018)

 * Fixed toml file license string `LGPL-2.1` → `LGPL-2.1+`.
 * Fixed toml file missing author email address.
 * Removed obsolete readme doc links.

# 1.0 (January 24th, 2018)

 * Original release.

[issue26]: https://github.com/jnqnfe/pulse-binding-rust/issues/26
